use crate::state::{Entry, Priority, Status};
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    NewEntry {description: String, priority: Option<Priority>},
    UpdateEntry { id: u64, description: Option<String>, status: Option<Status>, priority: Option<Priority> },
    DeleteEntry { id: u64 },
    TransferOwnership { new_owner: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(EntryResponse)]
    QueryEntry {id: u64},
    #[returns(ListResponse)]
    QueryList {start_after: Option<u64>, limit: Option<u32>},
}

// A custom struct is defined for each query response
#[cw_serde]
pub struct EntryResponse {
    pub id: u64,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
}

#[cw_serde]
pub struct ListResponse {
    pub entries: Vec<Entry>,
}