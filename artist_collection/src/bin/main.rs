use cosmwasm_schema::write_api;

use nft::msg::{ExecuteMsg, MsgDto};
use coreum_wasm_sdk::assetnft::Query;

fn main() {
    write_api! {
        instantiate: MsgDto,
        execute: ExecuteMsg,
        query: Query
    }
}
