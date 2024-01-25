use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,Addr
};

use crate::msg::{CardResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Card, ENTROPY, USER_CARDS, CARD_VIEWING_KEY};

use secret_toolkit::viewing_key::{ViewingKey, ViewingKeyStore};

//BLockchain is written in go, and pass some values to the instantiate:
//DepsMut ha molte dipendenze come: Storage in key value
//Env, information about the current state of the blockchain: Block time, Block height, The address of the current contract etc.
//Message info, sono informazioni su chi è il sender e quanti token ha inviato
//InstantiateMsg i messaggi del nostro contratto
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    ENTROPY.save(deps.storage, &msg.entropy)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Create {card, index} => try_create_card(deps, info, card, index),
        ExecuteMsg::Burn { index } => try_burn_card(deps, info, index),
        ExecuteMsg::GenerateViewingKey { index } => {
            try_generate_viewing_key(deps, env, info, index)
        }
    }
}

pub fn try_create_card(deps: DepsMut,  info: MessageInfo, card:Card, index:u8) -> StdResult<Response> {
    USER_CARDS.add_suffix(info.sender.as_bytes()).insert(deps.storage, &index, &card)?;
    Ok(Response::default())
}

pub fn try_burn_card(deps: DepsMut, info: MessageInfo, index: u8) -> StdResult<Response> {
    let user_exists = USER_CARDS.add_suffix(info.sender.as_bytes()).get(deps.storage, &index);
    let _ = match user_exists {
        Some(_) =>  USER_CARDS.add_suffix(info.sender.as_bytes()).remove(deps.storage, &index),
        None => Err(StdError::generic_err("User not Found!"))
    };
    Ok(Response::default())
}


pub fn try_generate_viewing_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    index: u8,
) -> StdResult<Response> {
    //map for viewing keys
    let viewing_keys_for_card = CARD_VIEWING_KEY
        .add_suffix(info.sender.as_bytes())
        .add_suffix(&[index]);

    let viewing_key = ViewingKey::create(
        deps.storage,
        &info,
        &env,
        &info.sender.to_string(),
        b"entropy",
    );

    //add viewing key to viewing key map
    viewing_keys_for_card.insert(deps.storage, &viewing_key, &true)?;

    let res = Response::default().add_attribute("viewing_key", viewing_key);

    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCard {wallet, viewing_key, index} => to_binary(&query_card(deps, wallet, viewing_key, index)?),
    }
}

fn query_card(deps: Deps, wallet: Addr, viewing_key: String, index: u8) -> StdResult<CardResponse> {
    let viewing_keys_exists = CARD_VIEWING_KEY
        .add_suffix(wallet.as_bytes())
        .add_suffix(&[index]);

    //Check if viewing key exists
    if viewing_keys_exists.contains(deps.storage, &viewing_key) {
        let card_exists = USER_CARDS
            .add_suffix(wallet.as_bytes())
            .get(deps.storage, &index);

        match card_exists {
            Some(card) => Ok(CardResponse { card: card }),
            None => Err(StdError::generic_err("Card not here!")),
        }
    } else {
        Err(StdError::generic_err("Wrong viewing key!"))
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::*;
//     use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies();
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "earth".to_string(),
//                 amount: Uint128::new(1000),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies_with_balance(&[Coin {
//             denom: "token".to_string(),
//             amount: Uint128::new(2),
//         }]);
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         // anyone can increment
//         let info = mock_info(
//             "anyone",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );

//         let exec_msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies_with_balance(&[Coin {
//             denom: "token".to_string(),
//             amount: Uint128::new(2),
//         }]);
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         // not anyone can reset
//         let info = mock_info(
//             "anyone",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let exec_msg = ExecuteMsg::Reset { count: 5 };

//         let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

//         match res {
//             Err(StdError::GenericErr { .. }) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let exec_msg = ExecuteMsg::Reset { count: 5 };

//         let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// pub struct Attestation {
//     pub name: String,
//     pub description: String,
//     pub issuer: String,
//     pub startDate: String,
//     pub endDate: String,
//     pub markleTree:String,
// }
// 
// pub struct User {
//     pub firstname: String,
//     pub lastname: String,
//     pub birthdate: String,
// }
// 
// pub static ATTESTATIONS: Keymap<String, Attestation> = Keymap::new(b"attestations");
// pub static USERS: Keymap<Addr, User> = Keymap::new(b"users");
// pub static USER_ATTESTATION: Keymap<Addr, String> = Keymap::new(b"user attestation");
// 
// 
// pub fn try_create_user(deps: DepsMut,  info: MessageInfo, user:User) -> StdResult<Response> {
//     USERS.add_suffix(info.sender.as_bytes()).insert(deps.storage, &user)?;
//     Ok(Response::default())
// }
// pub fn try_create_attestation(deps: DepsMut,  info: MessageInfo, attestationId:String, attestation:Attestation) -> StdResult<Response> {
//     ATTESTATIONS.add_suffix(&attestationId.as_bytes()).insert(deps.storage, &attestation)?;
//     Ok(Response::default())
// }
// pub fn try_attestate_user(deps: DepsMut,  info: MessageInfo, env: Env, attestationId:String) -> StdResult<Response> {
//     let viewing_key = ViewingKey::create(
//        deps.storage,
//        &info,
//        &env,
//        &info.sender.to_string(),
//        b"entropy",
//    );
//     USER_ATTESTATION.add_suffix(info.sender.as_bytes()).insert(deps.storage, &viewing_key, &attestationId.as_bytes())?;
//     Ok(Response::default())
// }
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 
// 