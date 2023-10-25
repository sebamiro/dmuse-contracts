use cosmwasm_std::Binary;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct MsgDto {
    pub artist_name: String,
    pub symbol: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {
        id: String,
        uri: Option<String>,
        uri_hash: Option<String>,
        data: Option<Binary>
    },
    Burn {
        id: String
    },
    InstantiateCollection {
        artist: String
    }
}

