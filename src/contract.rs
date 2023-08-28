#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    DenomAuthorityMetadata, TokenfactoryQuerier,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tokenfactory-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateDenom { denom } => execute::try_create_denom(env, denom),
    }
}

pub mod execute {
    use super::*;
    pub fn try_create_denom(env: Env, subdenom: String) -> Result<Response, ContractError> {
        let sender = env.contract.address.into();

        // construct message and convert them into cosmos message
        // (notice `CosmosMsg` type and `.into()`)
        let msg_create_denom: CosmosMsg = MsgCreateDenom { sender, subdenom }.into();

        Ok(Response::new()
            .add_message(msg_create_denom)
            .add_attribute("method", "try_create_denom"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetDenomAuthorityMetadata { denom } => {
            to_binary(&query::denom_metadata(deps, denom)?)
        }
    }
}

pub mod query {
    use super::*;

    pub fn denom_metadata(deps: Deps, denom: String) -> StdResult<Option<DenomAuthorityMetadata>> {
        let res = TokenfactoryQuerier::new(&deps.querier).denom_authority_metadata(denom)?;
        Ok(res.authority_metadata)
    }
}
