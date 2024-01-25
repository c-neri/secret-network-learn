use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::{Keymap,Item};
pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Card {
    pub name: String,
    pub address: String,
    pub phone: String,
}

//b meaning is in bytes
pub static USER_CARDS: Keymap<u8, Card> = Keymap::new(b"user cards");
pub static CARD_VIEWING_KEY: Keymap<String, bool> = Keymap::new(b"card viewing key"); // b"card viewing key" => prefix of the key
pub static ENTROPY: Item<String> = Item::new(b"entropy");

// pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
//     singleton(storage, CONFIG_KEY)
// }

// pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
//     singleton_read(storage, CONFIG_KEY)
// }
