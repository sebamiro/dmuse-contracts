use cosmwasm_schema::schemars::JsonSchema;
use cosmwasm_schema::schemars;
use cosmwasm_std::Binary;
use cosmwasm_schema::cw_serde;
use cosmwasm_schema::QueryResponses;

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

#[derive(JsonSchema, QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    Param,
    #[returns(String)]
    Class,
    #[returns(String)]
    Classes { issuer: String },
    #[returns(String)]
    Balance { owner: String },
    #[returns(String)]
    Owner { id: String },
    #[returns(String)]
    Supply,
    #[returns(String)]
    Nft { id: String },
    #[returns(String)]
    Nfts { owner: Option<String> },
    #[returns(String)]
    ClassNft,
    #[returns(String)]
    ClassesNft,
    #[returns(String)]
    BurnNft { nft_id: String },
    #[returns(String)]
    BurntNftsInClass
}
