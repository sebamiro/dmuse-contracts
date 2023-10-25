use cosmwasm_schema::write_api;

use ft::msg::{ExecuteMsg, MsgDto};
use coreum_wasm_sdk::assetft::Query;

fn main() {
    write_api! {
        instantiate: MsgDto,
        execute: ExecuteMsg,
        query: Query
    }
}
