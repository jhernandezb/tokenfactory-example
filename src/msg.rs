use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::DenomAuthorityMetadata;
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateDenom { denom: String },
    CreateDenom2 { denom: String },
    MintTo { amount: Coin, mint_to: String },
    MintTo2 { amount: Coin, mint_to: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(DenomAuthorityMetadata)]
    GetDenomAuthorityMetadata { denom: String },
}
