use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub enum ExecuteMsg {
    Deposit { amount: Uint128 },
}

#[cw_serde]
pub struct InstantiateMsg {
    pub max_mint_amount: Uint128,
}
