use cw_storage_plus::{Item};
use cosmwasm_std::{ Coin, Addr};

pub const COUNTER: Item<u64> = Item::new("counter");
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
// keep information about who created the contract
pub const OWNER: Item<Addr> = Item::new("owner");
pub const COUNTER_PROXY_ADDR: Item<String> = Item::new("counter_proxy_contract_address");
pub const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;