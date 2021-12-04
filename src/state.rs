use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, DepsMut, StdResult, Uint128, Coin};
use cw_storage_plus::{Item, Map, U128Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub share_addr: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
