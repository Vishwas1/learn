use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

use crate::{state::*, InstantiateMsg};


pub fn instantiate(deps: DepsMut, msg: InstantiateMsg, info: MessageInfo) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter)?;
    MINIMAL_DONATION.save(deps.storage, &msg.minimal_donation)?;
    // 
    OWNER.save(deps.storage, &info.sender);
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};
    
    use crate::{msg::ValueResp, state::COUNTER};
    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value: u64 = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
}

pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
    use crate::{state::COUNTER};

    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        
        
        let value: u64 = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &value)?;
        
        // we can use closure in update() of
        // COUNTER.update(deps.storage, |counter| -> StdResult<_> { Ok(counter + 1)})? ;

        // lets try to emit some event that the state has been updated
        // Events are emitted from execution using the Response::add_event function, passing the constructed Event type.
        let resp  = Response::new()
                                        .add_attribute("action","poke")
                                        .add_attribute("sender", info.sender.as_str())
                                        .add_attribute("counter", value.to_string());
        Ok(resp)
    }

}

pub mod exec_donation {
    // updating the state of our contract, so it keeps the minimal donation we expect
    use cosmwasm_std::{BankMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
    use crate::error::ContractError;

    use super::{COUNTER, MINIMAL_DONATION, OWNER};
    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        //let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        // Funds can be addressed by `info.funds` argument
        if info.funds.iter().any(|coin| {
            coin.denom == minimal_donation.denom 
            && coin.amount >= minimal_donation.amount
        }) {
            // counter += 1;
            // COUNTER.save(deps.storage, &counter)?;
            let value: u64 = COUNTER.load(deps.storage)? + 1;
            COUNTER.save(deps.storage, &value)?;
        };

        let resp  = Response::new()
        .add_attribute("action","donate")
        .add_attribute("sender", info.sender.as_str());
        // .add_attribute("counter", counter.to_string());
        Ok(resp)
    }


    pub fn widthdraw(deps: DepsMut,env: Env, info: MessageInfo) -> Result<Response, ContractError> {

        let owner = OWNER.load(deps.storage)?;
        // only owner can call this method
        if info.sender != owner {
            return Err(ContractError::Unauthorized { owner: owner.to_string() });
        }
     

        // fetch all balances of this contract
        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        // sending the message to transfer all amount to the info.sender in the blockcain
        let bank_msg = BankMsg::Send {
            to_address: owner.to_string(),
            amount: balance,
        };

        // emmiting event
        let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw")
        .add_attribute("sender", info.sender.as_str());
 
        Ok(resp)


     

    }
    
}


