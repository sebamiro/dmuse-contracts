use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct MsgDto {
    pub symbol: String,
    pub subunit: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint { amount: u128 },
    InstantiateArtistToken { artist: String },
}

