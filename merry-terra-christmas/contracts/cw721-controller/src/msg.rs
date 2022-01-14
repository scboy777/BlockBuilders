use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub collection_name: String,
    pub collection_symbol: String,
    pub uluna_price: u32,
    pub mint_cap: u32,
    pub nft_contract_addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MyExecuteMsg {
    ChangeMinter {
        new_minter: String,
    },
    ChangePrice {
        new_price: u32,
    },
    Withdraw {
        to_address: String,
    },
    MintNft {
        owner: String,
        how_many: u32,
    },
    UpdateAllowedTokenIds {
        add: Vec<String>,
        remove: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    Owner {},
    ContractInfo {},
}

/// Shows who own this contract
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OwnerResponse {
    pub owner: String,
}
