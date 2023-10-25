use coreum_wasm_sdk::assetnft::{
    self, ClassResponse,
    ClassesResponse, Query
};
use coreum_wasm_sdk::core::{ CoreumMsg, CoreumQueries, CoreumResult};
use coreum_wasm_sdk::pagination::PageRequest;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdResult
};
use cw2::set_contract_version;
use cw_ownable::{ assert_owner, initialize_owner };

use crate::error::ContractError;
use crate::state::CLASS_ID;
use crate::msg::{ ExecuteMsg, MsgDto };

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: MsgDto
    ) -> CoreumResult<ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    initialize_owner(deps.storage, deps.api, Some(info.sender.as_ref()))?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
    ) -> CoreumResult<ContractError> {
    match msg {
        ExecuteMsg::Mint {
            id,
            uri,
            uri_hash,
            data,
        } => mint(deps, info, id, uri, uri_hash, data),
        ExecuteMsg::Burn { id } => burn(deps, info, id),
        ExecuteMsg::InstantiateCollection { artist } => instantiate_collection(deps, info, _env, artist),
    }
}

fn mint(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
    uri: Option<String>,
    uri_hash: Option<String>,
    data: Option<Binary>,
    ) -> CoreumResult<ContractError> {
    assert_owner(deps.storage, &info.sender)?;
    let class_id = CLASS_ID.load(deps.storage)?;

    let msg = CoreumMsg::AssetNFT(assetnft::Msg::Mint {
        class_id: class_id.clone(),
        id: id.clone(),
        uri,
        uri_hash,
        data
    });

    Ok(Response::new()
       .add_attribute("method", "mint")
       .add_attribute("class_id", class_id)
       .add_attribute("id", id)
       .add_message(msg))
}


fn burn(
    deps: DepsMut,
    info: MessageInfo,
    id: String
    ) -> CoreumResult<ContractError> {
    assert_owner(deps.storage, &info.sender)?;
    let class_id = CLASS_ID.load(deps.storage)?;

    let msg = CoreumMsg::AssetNFT(assetnft::Msg::Burn {
        class_id: class_id.clone(),
        id: id.clone(),
    });

    Ok(Response::new()
       .add_attribute("method", "burn")
       .add_attribute("class_id", class_id)
       .add_attribute("id", id)
       .add_message(msg))
}

fn instantiate_collection(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    artist: String,
    ) -> CoreumResult<ContractError> {

    let issue_msg = CoreumMsg::AssetNFT(assetnft::Msg::IssueClass {
        name: artist.clone(),
        symbol: artist[..3].to_string(),
        uri: None,
        description: None,
        uri_hash: None,
        data: None,
        features: None,
        royalty_rate: None
    });

    let class_id = format!("{}-{}", artist[..3].to_string(), env.contract.address)
        .to_lowercase();
    CLASS_ID.save(deps.storage, &class_id)?;

    Ok(Response::new()
       .add_attribute("owner", info.sender)
       .add_attribute("class_id", class_id)
       .add_message(issue_msg))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<CoreumQueries>, _env: Env, msg: Query) -> StdResult<Binary> {
    match msg {
        Query::Params {  } => to_binary(&query_params(deps)?),
        Query::Class { id : _} => to_binary(&query_class(deps)?),
        Query::Classes { pagination: _, issuer } => to_binary(&query_classes(deps, issuer)?),
        Query::Frozen { id: _, class_id: _ } => to_binary(&query_params(deps)?),
        Query::BurntNFT { class_id: _, nft_id : _} => to_binary(&query_params(deps)?),
        Query::Whitelisted { id: _, class_id: _, account: _} => to_binary(&query_params(deps)?),
        Query::BurntNFTsInClass { pagination: _, class_id : _ } => to_binary(&query_params(deps)?),
        Query::WhitelistedAccountsForNFT { pagination : _, id : _, class_id : _ } => to_binary(&query_params(deps)?),
    }
}

fn query_params(deps: Deps<CoreumQueries>) -> StdResult<Binary> {
    let class_id = CLASS_ID.load(deps.storage)?;
    let request: QueryRequest<CoreumQueries> =
        CoreumQueries::AssetNFT(assetnft::Query::Class { id: class_id}).into();
    let res = deps.querier.query(&request)?;
    Ok(res)
}

fn query_class(deps: Deps<CoreumQueries>) -> StdResult<ClassResponse> {
    let class_id = CLASS_ID.load(deps.storage)?;
    let request: QueryRequest<CoreumQueries> =
        CoreumQueries::AssetNFT(assetnft::Query::Class { id: class_id }).into();
    let res = deps.querier.query(&request)?;
    Ok(res)
}

fn query_classes(deps: Deps<CoreumQueries>, issuer: String) -> StdResult<ClassesResponse> {
    let mut pagination = None;
    let mut classes = vec![];
    let mut res: ClassesResponse;
    loop {
        let request = CoreumQueries::AssetNFT(assetnft::Query::Classes {
            pagination,
            issuer: issuer.clone(),
        })
        .into();
        res = deps.querier.query(&request)?;
        classes.append(&mut res.classes);
        if res.pagination.next_key.is_none() {
            break;
        } else {
            pagination = Some(PageRequest {
                key: res.pagination.next_key,
                offset: None,
                limit: None,
                count_total: None,
                reverse: None,
            })
        }
    }
    let res = ClassesResponse {
        pagination: res.pagination,
        classes,
    };
    Ok(res)
}

