use crate::state::State;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer { recipient: String }, // Owner can transfer ownership to another address

    Execute {}, // Owner can post counter_offer on unexpired option to execute and get the collateral

    Burn {}, // Release collateral if expired
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(ConfigResponse)]
    Config {},
}

pub type ConfigResponse = State;