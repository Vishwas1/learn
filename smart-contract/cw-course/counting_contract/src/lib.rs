use cosmwasm_std::{Coin, Reply};
use cw_utils::parse_reply_instantiate_data;
use cosmwasm_std::{ 
    Addr, 
    entry_point, 
    to_binary, 
    Binary, 
    DepsMut, 
    Deps, 
    Empty, 
    Env, 
    MessageInfo,
    Response, 
    StdResult 
};
// use cosmwasm_std::{Deps, StdResult};
mod contract;
pub mod msg;
mod state;
mod error;

use error::ContractError;
use msg::InstantiateMsg;
use state::{COUNTER, COUNTER_PROXY_ADDR, INSTANTIATE_TOKEN_REPLY_ID};
// Smart contract's entry potin
// But in smart contract, entiry poitns can be many not us one unlike regular rust code which is main
// The `instantiate()` is called when the smart contract is created for the first time
// This it as a constructor
// #[entry_point]
#[cfg_attr(not(feature = "library"), entry_point)]
fn instantiate(
    // utility that allows querying and updating the contract state
    // querying another contract state
    // storing state
    _deps: DepsMut, 
    // object representing the blockchains state;  chain height and id, current timestamp etc
    _env: Env,
    //  contains metainformation about the message; an address that sends the message and chain native tokens sent
    _info: MessageInfo,
    // like constructor params for default values of a states
    _msg: InstantiateMsg
) -> StdResult<Response> {
    // COUNTER.save(_deps.storage, &_msg.counter)?;
    // MINIMAL_DONATION.save(_deps.storage, &_msg.minimal_donation)?;
    contract::instantiate(_deps, _msg, _info)?;
    Ok(Response::new())
}


// #[entry_point]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: msg::ExecMsg) -> Result<Response, ContractError> {
    use msg::ExecMsg::*;
    use contract::exec;
    use contract::exec_donation;
    
    match _msg {
        Poke {
            proxy_contract_addr
        } => exec::poke(_deps, _info, proxy_contract_addr).map_err(ContractError::from),
        Donate {} => exec_donation::donate(_deps,_info, _env).map_err(ContractError::from),
        Withdraw {} => exec_donation::widthdraw(_deps, _env, _info),
        Deploy { 
            token_code_id
        } => exec_donation::deploy_nft_contract(_deps, _info, _env, token_code_id).map_err(ContractError::from),
    }
}

// #[entry_point]
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary>{
    use msg::QueryMsg::*;
    use contract::query;

    match msg {
        Value {} => to_binary(&query::value(deps)?),
        GetProxyMessage {} => to_binary(&query::getProxyMessage(deps)?)
    }
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {

    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidTokenId { token_id: msg.id});
    }

    let value: u64 = COUNTER.load(deps.storage)? + 1;
    COUNTER.save(deps.storage, &value)?;

    let reply = parse_reply_instantiate_data(msg).unwrap();
    let cw721_address = Addr::unchecked(reply.contract_address).into();
    
    COUNTER_PROXY_ADDR.save(deps.storage, &cw721_address)?;

    Ok(Response::new())
}


// cfg is a conditional compilation attribute
// meaning that the code it wraps would be 
// compiled-in if and only if the predicate 
// passed to it is true. 
#[cfg(any(test, feature = "tests"))]
mod test {
    use crate::msg::{ExecMsg, ValueRespProxy};

    use self::msg::{QueryMsg, ValueResp};

    use super::*;
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn query_value(){
        // App simulates blockhain
        let mut app = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("sender");
        // More sophisticated way of simulating blockhain
        // need to put fund some tokens to this sender
        let initialBalance = 10000;
        let tokenDenom =  "uHID";
        // let mut app = AppBuilder::new().build(|router, _api, storage| {
        //     router  // from router
        //     .bank // extract bank module
        //     .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account 
        //     .unwrap()
        // });

        // storing contract code on blockhain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
            sender.clone(), // simulating a blockchain address
            &InstantiateMsg{
                counter: 0,
                minimal_donation: Some(coin(10, tokenDenom))
            },
            &[],
            "Funding contract",
            None,
        ).unwrap();

        // // lets first increment the counter = 1
        let proxy_contract_addr = "abcd".to_owned();
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecMsg::Poke {
                proxy_contract_addr: proxy_contract_addr.clone()
            }, &[])
            .unwrap();

        // // lets first increment the counter one more time = 2
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecMsg::Poke {
                proxy_contract_addr: proxy_contract_addr.clone()
            }, &[])
            .unwrap();

        // lets send some fund; which will also increase the coounter = 3
        // let amount_to_be_sent_to_contract = 10;
        // app.execute_contract(
        //     sender.clone(), 
        //     contract_addr.clone(), 
        //     &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
        //     .unwrap();

        // then test is counter has been incremented
        let resp: ValueResp = app
                    .wrap()
                    .query_wasm_smart(
                        contract_addr.clone(), 
                        &QueryMsg::Value {  })
                        .unwrap();

        assert_eq!(resp, ValueResp {value: 2});

        // test if proxy contract address is set
        let resp1: ValueRespProxy = app.wrap().query_wasm_smart(contract_addr.clone(), &QueryMsg::GetProxyMessage {}).unwrap();
        assert_eq!(resp1, ValueRespProxy { proxyContractAddress: proxy_contract_addr.clone()});



        // // lets check the balane of the cotnract as well....
        // assert_eq!(app.wrap().query_all_balances(contract_addr).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
        // // check if amount was deducted from sernder account or not
        // assert_eq!(app.wrap().query_all_balances(sender).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom))
    }


    #[test]
    fn donate(){
        // App simulates blockhain
        // let mut app = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("sender");
        // More sophisticated way of simulating blockhain
        // need to put fund some tokens to this sender
        let initialBalance = 10000;
        let tokenDenom =  "uHID";
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router  // from router
            .bank // extract bank module
            .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account 
            .unwrap()
        });

        // storing contract code on blockhain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
            sender.clone(), // simulating a blockchain address
            &InstantiateMsg{
                counter: 0,
                minimal_donation: Some(coin(10, tokenDenom))
            },
            &[],
            "Funding contract",
            None,
        ).unwrap();

        // lets send some fund; which will also increase the coounter = 3
        let amount_to_be_sent_to_contract = 10;
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
            .unwrap();

        // then test is counter has been incremented
        let resp: ValueResp = app
                    .wrap()
                    .query_wasm_smart(
                        contract_addr.clone(), 
                        &QueryMsg::Value {  })
                        .unwrap();

        assert_eq!(resp, ValueResp {value: 1});

        // lets check the balane of the cotnract as well....
        assert_eq!(app.wrap().query_all_balances(contract_addr).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
        // check if amount was deducted from sernder account or not
        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom))
    }

    #[test]
    fn withdraw(){
        // App simulates blockhain
        // let mut app = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("sender");
        let sender2 = Addr::unchecked("sender"); // this guyshouuld not be able to withdraw funds from contract since he is not the owner
        // More sophisticated way of simulating blockhain
        // need to put fund some tokens to this sender
        let initialBalance = 10000;
        let tokenDenom =  "uHID";
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router  // from router
            .bank // extract bank module
            .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account 
            .unwrap();


            router  // from router
            .bank // extract bank module
            .init_balance(storage, &sender2, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account 
            .unwrap()
        });

        // storing contract code on blockhain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
            sender.clone(), // simulating a blockchain address
            &InstantiateMsg{
                counter: 0,
                minimal_donation: Some(coin(10, tokenDenom))
            },
            &[],
            "Funding contract",
            None,
        ).unwrap();

        // lets send some fund; which will also increase the coounter = 3
        let amount_to_be_sent_to_contract = 10;
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
            .unwrap();

        
        // then test is counter has been incremented
        let resp: ValueResp = app
                    .wrap()
                    .query_wasm_smart(
                        contract_addr.clone(), 
                        &QueryMsg::Value {  })
                        .unwrap();

        assert_eq!(resp, ValueResp {value: 1});

        // lets check the balane of the cotnract as well....
        assert_eq!(app.wrap().query_all_balances(contract_addr.clone()).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
        // check if amount was deducted from sernder account or not
        assert_eq!(app.wrap().query_all_balances(sender.clone()).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom));

        // taking my funds back in my account
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecMsg::Withdraw {  }, 
            &[])
        .unwrap();

        // lets check the balane of the cotnract as well....
        assert_eq!(app.wrap().query_all_balances(contract_addr).unwrap(), &[]);
        // check if amount was deducted from sernder account or not
        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), coins(initialBalance, tokenDenom));
    }


    #[test]
    fn unauthorize_withdraw(){
        // App simulates blockhain
        // let mut app = App::default();

        // Let's create a dummy account
        let sender = Addr::unchecked("sender");
        let sender2 = Addr::unchecked("sender"); // this guyshouuld not be able to withdraw funds from contract since he is not the owner
        // More sophisticated way of simulating blockhain
        // need to put fund some tokens to this sender
        let initialBalance = 10000;
        let tokenDenom =  "uHID";
        let mut app = AppBuilder::new().build(|router, _api, storage| {
            router  // from router
            .bank // extract bank module
            .init_balance(storage, &sender, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account 
            .unwrap();


            router  // from router
            .bank // extract bank module
            .init_balance(storage, &sender2, coins(initialBalance, tokenDenom)) // send some initial tokens to the sender account 
            .unwrap()
        });

        // storing contract code on blockhain
        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
            sender.clone(), // simulating a blockchain address
            &InstantiateMsg{
                counter: 0,
                minimal_donation: Some(coin(10, tokenDenom))
            },
            &[],
            "Funding contract",
            None,
        ).unwrap();

        // lets send some fund; which will also increase the coounter = 3
        let amount_to_be_sent_to_contract = 10;
        app.execute_contract(
            sender.clone(), 
            contract_addr.clone(), 
            &ExecMsg::Donate {}, &coins(amount_to_be_sent_to_contract, tokenDenom))
            .unwrap();

        
        // then test is counter has been incremented
        let resp: ValueResp = app
                    .wrap()
                    .query_wasm_smart(
                        contract_addr.clone(), 
                        &QueryMsg::Value {  })
                        .unwrap();

        assert_eq!(resp, ValueResp {value: 1});

        // lets check the balane of the cotnract as well....
        assert_eq!(app.wrap().query_all_balances(contract_addr.clone()).unwrap(), coins(amount_to_be_sent_to_contract, tokenDenom));
        // check if amount was deducted from sernder account or not
        assert_eq!(app.wrap().query_all_balances(sender.clone()).unwrap(), coins(initialBalance -  amount_to_be_sent_to_contract, tokenDenom));

        
        // this should fail becuase of unauthorized withdrawal from sender2    
        //     let err = app
        //     .execute_contract(sender2, contract_addr.clone(), &ExecMsg::Withdraw {}, &[])
        //     .unwrap_err();

        // println!("err = {:?}", err);

        //    assert_eq!(
        //         ContractError::Unauthorized {
        //             owner: sender.into()
        //         },
        //         err.downcast().unwrap()
        //     );
    }

}