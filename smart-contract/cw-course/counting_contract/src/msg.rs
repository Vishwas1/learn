use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Coin, CosmosMsg, StdResult, WasmMsg};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
    pub minimal_donation: Option<Coin>,
}
 
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ValueResp)]
    Value {},
    #[returns(ValueRespProxy)]
    GetProxyMessage {},
}

#[cw_serde]
pub enum ExecMsg {
    Poke {
        proxy_contract_addr: String 
    },
    Donate {},
    Withdraw {},
}

#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}


#[cw_serde]
pub struct ValueRespProxy {
    pub proxyContractAddress: String,
}





