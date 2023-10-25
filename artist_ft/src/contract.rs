use coreum_wasm_sdk::assetft::{
    self, BalanceResponse, TokenResponse, TokensResponse, ParamsResponse, Query
};
use coreum_wasm_sdk::core::{CoreumMsg, CoreumQueries, CoreumResult};
use coreum_wasm_sdk::pagination::PageRequest;
use cosmwasm_std::{coin, entry_point, to_binary, Binary, Deps, StdResult, Uint128};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw_ownable::initialize_owner;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, MsgDto, QueryMsg};
use crate::state::DENOM;

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// ********** Instantiate **********

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: MsgDto,
) -> CoreumResult<ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    initialize_owner(deps.storage, deps.api, Some(info.sender.as_ref()))?;

    let issue_msg = CoreumMsg::AssetFT(assetft::Msg::Issue {
        symbol: msg.symbol,
        subunit: msg.subunit.clone(),
        precision: 6,
        initial_amount: Uint128::new(0),
        description: None,
        features: Some(vec![0]),
        burn_rate: Some("0".into()),
        send_commission_rate: None,
    });

    let denom = format!("{}-{}", msg.subunit, env.contract.address).to_lowercase();

    DENOM.save(deps.storage, &denom)?;

    Ok(Response::new()
        .add_attribute("owner", info.sender)
        .add_attribute("denom", denom)
        .add_message(issue_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
    ) -> CoreumResult<ContractError> {
    match msg {
        ExecuteMsg::Mint { amount } => mint(deps, info, amount),
        ExecuteMsg::InstantiateArtistToken { artist } => instantate_artist_ft(deps, _env, info, artist),
    }
}

fn mint(
    deps: DepsMut,
    _info: MessageInfo,
    amount: u128
    ) -> CoreumResult<ContractError> {
    let denom = DENOM.load(deps.storage)?;

    let msg = CoreumMsg::AssetFT(assetft::Msg::Mint {
        coin: coin(amount, denom.clone()),
    });

    Ok(Response::new()
        .add_attribute("method", "mint")
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string())
        .add_message(msg))
}

fn instantate_artist_ft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    artist: String
    ) -> CoreumResult<ContractError> {

    let issue_msg = CoreumMsg::AssetFT(assetft::Msg::Issue {
        symbol: artist[..3].to_string(),
        subunit: artist.clone(),
        precision: 6,
        initial_amount: Uint128::new(0),
        description: None,
        features: Some(vec![0]),
        burn_rate: Some("0".into()),
        send_commission_rate: None,
    });

    let denom = format!("{}-{}", artist, env.contract.address).to_lowercase();

    DENOM.save(deps.storage, &denom)?;

    Ok(Response::new()
        .add_attribute("owner", info.sender)
        .add_attribute("denom", denom)
        .add_message(issue_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<CoreumQueries>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
        QueryMsg::Token {} => to_binary(&query_token(deps)?),
        QueryMsg::Tokens { issuer } => to_binary(&query_tokens(deps, issuer)?),
        QueryMsg::Balance { account } => to_binary(&query_balance(deps, account)?),
    }
}

fn query_params(deps: Deps<CoreumQueries>) -> StdResult<ParamsResponse> {
    let request = CoreumQueries::AssetFT(Query::Params {}).into();
    let res = deps.querier.query(&request)?;
    Ok(res)
}

fn query_token(deps: Deps<CoreumQueries>) -> StdResult<TokenResponse> {
    let denom = DENOM.load(deps.storage)?;
    let request = CoreumQueries::AssetFT(Query::Token { denom }).into();
    let res = deps.querier.query(&request)?;
    Ok(res)
}

fn query_tokens(deps: Deps<CoreumQueries>, issuer: String) -> StdResult<TokensResponse> {
    let mut pagination = None;
    let mut tokens = vec![];
    let mut res: TokensResponse;
    loop {
        let request = CoreumQueries::AssetFT(Query::Tokens {
            pagination,
            issuer: issuer.clone(),
        })
        .into();
        res = deps.querier.query(&request)?;
        tokens.append(&mut res.tokens);
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
    let res = TokensResponse {
        pagination: res.pagination,
        tokens,
    };
    Ok(res)
}

fn query_balance(deps: Deps<CoreumQueries>, account: String) -> StdResult<BalanceResponse> {
    let denom = DENOM.load(deps.storage)?;
    let request = CoreumQueries::AssetFT(Query::Balance { account, denom }).into();
    let res = deps.querier.query(&request)?;
    Ok(res)
}

