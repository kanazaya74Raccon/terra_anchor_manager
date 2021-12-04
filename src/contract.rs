#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, CosmosMsg, BankMsg, Coin, WasmMsg, wasm_execute, QueryRequest,
    QuerierWrapper, AllBalanceResponse, BankQuery,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg };
use crate::state::{Config, CONFIG};
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};
use crate::market::{ExecuteMsg as AnchorMarket};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ProjectContract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .admin
        .and_then(|s| deps.api.addr_validate(s.as_str()).ok())
        .unwrap_or(info.sender);
    let config = Config {
        owner: owner.clone(),
        share_addr: deps.api.addr_validate(msg.share_addr.as_str())?,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit{} => try_deposit(deps, info),
        ExecuteMsg::ClaimRewards{} => try_claimrewards(),
        ExecuteMsg::SetShareAddr{share} => try_setshare(deps, share),
    }
}
pub fn try_deposit(deps:DepsMut, info:MessageInfo) -> Result<Response, ContractError>
{
    // let config = CONFIG.load(deps.storage)?;
    // if config.owner != info.sender {
        // return Err(ContractError::Unauthorized {});
    // }
    let funds = info.funds.clone();
    for mut coin in funds{
        coin.amount = Uint128::from(
        (coin.amount.u128() as f32 * 0.8) as u128);
    }

    let anchormarket = "terra15dwd5mj8v59wpj0wvt233mf5efdff808c5tkal";
    Ok(Response::new()
    .add_messages(vec![CosmosMsg::Wasm(
        WasmMsg::Execute {
            contract_addr: String::from(anchormarket),
            msg: to_binary(&AnchorMarket::DepositStable {}).unwrap(),
            funds: info.funds
    })])
    .add_attribute("action", "deposit to project"))
}
pub fn try_claimrewards() -> Result<Response, ContractError>
{
    Ok(Response::new())
}
pub fn try_setshare(deps:DepsMut, share:String) -> Result<Response, ContractError>
{
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance{} => to_binary(&query_balance(deps, _env)?),
    }
}
fn query_balance(deps:Deps, _env:Env) -> StdResult<Vec<Coin>>{

    let denom = String::from("ucosm");
    let balance: AllBalanceResponse = deps.querier.query(
        &QueryRequest::Bank(BankQuery::AllBalances {
            address: _env.contract.address.to_string(),
        }
    ))?;

    Ok(balance.amount)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, 
        MOCK_CONTRACT_ADDR, MockQuerier};
    use cosmwasm_std::{from_binary, Addr, CosmosMsg, WasmMsg,
        BankQuery, BalanceResponse, };
    #[test]
    fn add_project(){
        let mut deps = mock_dependencies(&[]);
        
        let msg = InstantiateMsg{
            admin:None,
            share_addr: String::from(MOCK_CONTRACT_ADDR),
        };

        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
//add contract
        let msg = ExecuteMsg::Deposit{};
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
println!("{:?}", res);
        // let msg = ExecuteMsg::AddContract{
        //     contract:String::from("wersome1"),
        // };
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        // assert_eq!(res.messages.len(), 0);


//add project        
        // let msg = ExecuteMsg::AddProject{
        //     project_id: Uint128::new(100),
        //     project_wallet: String::from("some"),
        // };
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        // assert_eq!(res.messages.len(), 0);

//balance

//back 2 projct
//         let msg = ExecuteMsg::Back2Project{
//             project_id: Uint128::new(100),
//             backer_wallet: String::from("some"),
//         };
//         let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
// println!("before balance:{:?}", res.messages);
        // assert_eq!(res.messages.len(), 0);
        
//Get Project
        // let msg = QueryMsg::GetProject{id:Uint128::new(101)};
        // let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        
        // let prj:ProjectResponse = from_binary(&res).unwrap();
        // println!("project {:?}:{:?}", prj.project_id, prj.project_wallet );
        // assert_eq!(
        //     prj,
        //     ProjectResponse{
        //         project_id: Uint128::new(98),
        //         project_wallet: String::from("some"),                
        //     }
        // )

        // let msg = QueryMsg::GetBacker{id:Uint128::new(100)};
        // let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        // let nlen:usize = from_binary(&res).unwrap();
        // println!("backer count = {:?}", nlen);
    }
}
