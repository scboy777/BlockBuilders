use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdError, StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use cw721::ContractInfoResponse;
pub use cw721_metadata_onchain::{ExecuteMsg, Extension, MintMsg};

use crate::msg::{InstantiateMsg, MyExecuteMsg, OwnerResponse, QueryMsg};
use crate::state::Cw721Controller;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-first-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> Cw721Controller<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        let info = ContractInfoResponse {
            name: msg.collection_name,
            symbol: msg.collection_symbol,
        };
        self.collection_info.save(deps.storage, &info)?;
        self.owner.save(deps.storage, &_info.sender.clone())?;
        self.uluna_price.save(deps.storage, &msg.uluna_price)?;
        self.mint_cap.save(deps.storage, &msg.mint_cap)?;
        let nft_contract_addr = deps.api.addr_validate(&msg.nft_contract_addr)?;
        self.nft_contract_addr
            .save(deps.storage, &nft_contract_addr)?;
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: MyExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            MyExecuteMsg::UpdateAllowedTokenIds { add, remove } => {
                self.update_allowed_token_ids(deps, env, info, add, remove)
            }
            MyExecuteMsg::ChangeMinter { new_minter } => {
                self.change_minter(deps, env, info, new_minter)
            }
            MyExecuteMsg::ChangePrice { new_price } => {
                self.change_price(deps, env, info, new_price)
            }
            MyExecuteMsg::Withdraw { to_address } => self.withdraw(deps, env, info, to_address),
            MyExecuteMsg::MintNft { owner, how_many } => {
                self.mint_nft(deps, env, info, owner, how_many)
            }
        }
    }
}

impl<'a> Cw721Controller<'a> {
    pub fn update_allowed_token_ids(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        add: Vec<String>,
        remove: Vec<String>,
    ) -> Result<Response, ContractError> {
        // only owner of this contract can update
        let owner = self.owner.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }
        for token_id in add.iter() {
            self.allowed_token_ids
                .save(deps.storage, token_id.clone(), &())?;
        }

        for token_id in remove.iter() {
            self.allowed_token_ids.remove(deps.storage, token_id.to_owned());
        }
        Ok(Response::new().add_attribute("action", "update allowed token ids"))
    }

    pub fn mint_nft(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        owner: String,
        how_many: u32,
    ) -> Result<Response, ContractError> {
        // enough fund
        let mut amount = 0;
        for fund in info.funds.iter() {
            if fund.denom.to_string() == "uluna" {
                amount = fund.amount.u128() as u32;
                break;
            }
        }
        if amount != self.uluna_price.load(deps.storage)? * how_many {
            return Err(ContractError::Std(StdError::GenericErr {
                msg: "Not enough funds".to_string(),
            }));
        }

        // randomly choose one token from tokens
        let allowed_token_ids =
            self.allowed_token_ids
                .keys(deps.storage, None, None, Order::Ascending);
        let ks: Vec<_> = allowed_token_ids.take(how_many as usize).collect();
        let token_ids: Vec<_> = ks.iter().map(|v| String::from_utf8(v.to_vec()).unwrap()).collect();
        for token_id in token_ids.iter() {
            self.allowed_token_ids.remove(deps.storage, token_id.to_owned());
        }
        let nft_contract_addr = self.nft_contract_addr.load(deps.storage)?.to_string();
        Ok(
            Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: nft_contract_addr,
                funds: vec![],
                msg: to_binary(&ExecuteMsg::Mint { owner, token_ids })?,
            })),
        )
    }

    pub fn change_minter(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        new_minter: String,
    ) -> Result<Response, ContractError> {
        let owner = self.owner.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }
        Ok(Response::new()
            .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: self.nft_contract_addr.load(deps.storage)?.to_string(),
                funds: vec![],
                msg: to_binary(&ExecuteMsg::ChangeMinter { new_minter })?,
            }))
            .add_attribute("action", "change_minter"))
    }

    pub fn withdraw(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        to_address: String,
    ) -> Result<Response, ContractError> {
        let owner = self.owner.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }
        let amount: Vec<Coin> = deps.querier.query_all_balances(&_env.contract.address)?;
        Ok(Response::new()
            .add_message(CosmosMsg::Bank(BankMsg::Send { to_address, amount }))
            .add_attribute("action", "withdraw"))
    }

    pub fn change_price(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        new_price: u32,
    ) -> Result<Response, ContractError> {
        let owner = self.owner.load(deps.storage)?;
        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }
        self.uluna_price.save(deps.storage, &new_price)?;
        Ok(Response::new().add_attribute("action", "change_price"))
    }
}

impl<'a> Cw721Controller<'a> {
    pub fn owner(&self, deps: Deps) -> StdResult<OwnerResponse> {
        let owner = self.owner.load(deps.storage)?;
        Ok(OwnerResponse {
            owner: owner.to_string(),
        })
    }

    pub fn collection_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.collection_info.load(deps.storage)
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Owner {} => to_binary(&self.owner(deps)?),
            QueryMsg::ContractInfo {} => to_binary(&self.collection_info(deps)?),
        }
    }
}
