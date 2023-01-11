use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, CONFIG};

// INSTANTIATE

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    // Option expiration date is checked and the state is saved and response returned.
    if msg.expires <= env.block.height {
        return Err(ContractError::OptionExpired {
            expired: msg.expires,
        });
    }

    let state = State {
        creator: info.sender.clone(),
        owner: info.sender.clone(),
        collateral: info.funds,
        counter_offer: msg.counter_offer,
        expires: msg.expires,
    };

    CONFIG.save(deps.storage, &state)?;

    Ok(Response::default())
}

// EXECUTE

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { recipient } => execute_transfer(deps, env, info, recipient),
        ExecuteMsg::Execute {} => execute_execute(deps, env, info),
        ExecuteMsg::Burn {} => execute_burn(deps, env, info),
    }
}

pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
) -> Result<Response, ContractError> {
    // ensure msg sender is the owner
    let mut state = CONFIG.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    // set new owner on state
    state.owner = deps.api.addr_validate(&recipient)?;
    CONFIG.save(deps.storage, &state)?;

    let res =
        Response::new().add_attributes([("action", "transfer"), ("owner", recipient.as_str())]);
    Ok(res)
}

pub fn execute_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // ensure msg sender is the owner
    let state = CONFIG.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    // ensure not expired
    if env.block.height >= state.expires {
        return Err(ContractError::OptionExpired {
            expired: state.expires,
        });
    }

    // ensure sending proper counter_offer
    if info.funds != state.counter_offer {
        return Err(ContractError::CounterOfferMismatch {
            offer: info.funds,
            counter_offer: state.counter_offer,
        });
    }

    // release counter_offer to creator
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send {
        to_address: state.creator.to_string(),
        amount: state.counter_offer,
    });

    // release collateral to sender
    res = res.add_message(BankMsg::Send {
        to_address: state.owner.to_string(),
        amount: state.collateral,
    });

    // delete the option
    CONFIG.remove(deps.storage);

    res = res.add_attribute("action", "execute");
    Ok(res)
}

pub fn execute_burn(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // ensure is expired
    let state = CONFIG.load(deps.storage)?;
    if env.block.height < state.expires {
        return Err(ContractError::OptionNotExpired {
            expires: state.expires,
        });
    }

    // ensure sending proper counter_offer
    if !info.funds.is_empty() {
        return Err(ContractError::FundsSentWithBurn {});
    }

    // release collateral to creator
    let mut res = Response::new();
    res = res.add_message(BankMsg::Send {
        to_address: state.creator.to_string(),
        amount: state.collateral,
    });

    // delete the option
    CONFIG.remove(deps.storage);

    res = res.add_attribute("action", "burn");
    Ok(res)
}

// QUERY

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}