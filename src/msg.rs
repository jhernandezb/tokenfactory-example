use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct DenomAuthorityMetadata {
    pub admin: String,
}
#[cw_serde]
pub struct AuthorityMetadataResponse {
    pub authority_metadata: Option<DenomAuthorityMetadata>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateDenom { denom: String },
    MintTo { amount: Coin, mint_to: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(DenomAuthorityMetadata)]
    GetDenomAuthorityMetadata { denom: String },
}
