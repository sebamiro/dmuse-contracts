use cosmwasm_schema::write_api;

use ft::msg::{ExecuteMsg, MsgDto, QueryMsg};

fn main() {
    write_api! {
        instantiate: MsgDto,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
