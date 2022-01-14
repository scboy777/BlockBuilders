use cw721::ContractInfoResponse;

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub struct Cw721Controller<'a> {
    pub collection_info: Item<'a, ContractInfoResponse>,
    pub owner: Item<'a, Addr>,
    pub uluna_price: Item<'a, u32>,
    pub mint_cap: Item<'a, u32>,
    pub nft_contract_addr: Item<'a, Addr>,
    pub allowed_token_ids: Map<'a, String, ()>,
}

impl Default for Cw721Controller<'static> {
    fn default() -> Self {
        Self::new(
            "collection_info",
            "owner",
            "uluna_price",
            "mint_cap",
            "nft_contract_addr",
            "allowed_token_ids",
        )
    }
}

impl<'a> Cw721Controller<'a> {
    fn new(
        collection_info_key: &'a str,
        owner_key: &'a str,
        uluna_price_key: &'a str,
        mint_cap_key: &'a str,
        nft_contract_addr_key: &'a str,
        allowed_token_ids_key: &'a str,
    ) -> Self {
        Self {
            collection_info: Item::new(collection_info_key),
            owner: Item::new(owner_key),
            uluna_price: Item::new(uluna_price_key),
            mint_cap: Item::new(mint_cap_key),
            nft_contract_addr: Item::new(nft_contract_addr_key),
            allowed_token_ids: Map::new(allowed_token_ids_key),
        }
    }
}
