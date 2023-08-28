use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
use anybuf::Anybuf;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    ensure, to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
    DenomAuthorityMetadata, TokenfactoryQuerier,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint};
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateDenom { denom } => execute::try_create_denom(env, denom),
        ExecuteMsg::CreateDenom2 { denom } => execute::try_create_denom2(env, denom),
        ExecuteMsg::MintTo { amount, mint_to } => {
            execute::try_mint_to(info, deps, env, amount, mint_to)
        }
        ExecuteMsg::MintTo2 { amount, mint_to } => {
            execute::try_mint_to2(info, deps, env, amount, mint_to)
        }
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
    pub fn try_create_denom2(env: Env, subdenom: String) -> Result<Response, ContractError> {
        let sender = env.contract.address.into();

        // construct message and convert them into cosmos message
        // (notice `CosmosMsg` type and `.into()`)
        let msg_create_denom = CosmosMsg::Stargate {
            type_url: "/osmosis.tokenfactory.v1beta1.Msg/CreateDenom".to_string(),
            value: encode_msg_create_denom(sender, subdenom).into(),
        };

        Ok(Response::new()
            .add_message(msg_create_denom)
            .add_attribute("method", "try_create_denom2"))
    }

    pub fn try_mint_to(
        info: MessageInfo,
        deps: DepsMut,
        env: Env,
        amount: Coin,
        mint_to: String,
    ) -> Result<Response, ContractError> {
        let admin = STATE.load(deps.storage)?.owner;
        ensure!(info.sender == admin, ContractError::Unauthorized {});
        let mint_to_adress = deps.api.addr_validate(&mint_to)?;
        let account = env.contract.address.into();

        let msg_create_denom: CosmosMsg = MsgMint {
            sender: account,
            amount: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                amount: amount.amount.to_string(),
                denom: amount.denom,
            }),
            mint_to_address: mint_to_adress.to_string(),
        }
        .into();

        Ok(Response::new()
            .add_message(msg_create_denom)
            .add_attribute("method", "mint_to"))
    }

    pub fn try_mint_to2(
        info: MessageInfo,
        deps: DepsMut,
        env: Env,
        amount: Coin,
        mint_to: String,
    ) -> Result<Response, ContractError> {
        let admin = STATE.load(deps.storage)?.owner;
        ensure!(info.sender == admin, ContractError::Unauthorized {});
        let mint_to_adress = deps.api.addr_validate(&mint_to)?;
        let account = env.contract.address.into();

        // construct message and convert them into cosmos message
        // (notice `CosmosMsg` type and `.into()`)
        let msg_mint = CosmosMsg::Stargate {
            type_url: "/osmosis.tokenfactory.v1beta1.Msg/Mint".to_string(),
            value: encode_msg_mint(account, &amount, mint_to_adress.to_string()).into(),
        };

        Ok(Response::new()
            .add_message(msg_mint)
            .add_attribute("method", "mint_to2"))
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

fn encode_msg_create_denom(sender: String, subdenom: String) -> Vec<u8> {
    Anybuf::new()
        .append_string(1, sender)
        .append_string(2, subdenom)
        .into_vec()
}

fn encode_msg_mint(sender: String, amount: &Coin, mint_to_address: String) -> Vec<u8> {
    let coin = Anybuf::new()
        .append_string(1, &amount.denom)
        .append_string(2, amount.amount.to_string());
    Anybuf::new()
        .append_string(1, sender)
        .append_message(2, &coin)
        .append_string(3, mint_to_address)
        .into_vec()
}
