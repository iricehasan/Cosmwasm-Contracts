use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
}
#[cw_serde]
pub struct Entry {
    pub id: u64,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
}
#[cw_serde]
pub enum Status {
    ToDo,
    InProgress,
    Done,
    Cancelled,
}
#[cw_serde]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}

pub const CONFIG: Item<Config> = Item::new("config");
//Item stores a single variable of a given type, identified by a string storage key.
pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
//An Item that holds an integer value. The value stored in ENTRY_SEQ is incremented every time a new entry is appended to the list.
pub const LIST: Map<u64, Entry> = Map::new("list");