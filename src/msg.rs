use cosmwasm_schema::{cw_serde, QueryResponses};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::DenomAuthorityMetadata;
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateDenom { denom: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(DenomAuthorityMetadata)]
    GetDenomAuthorityMetadata { denom: String },
}
