pub mod contract;
mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;
pub use crate::msg::{MyExecuteMsg, InstantiateMsg, QueryMsg};
pub use crate::state::Cw721Controller;
pub use cw721_metadata_onchain::{ExecuteMsg, Extension, MintMsg};


// This is a simple type to let us handle empty extensions

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let tract = Cw721Controller::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: MyExecuteMsg,
    ) -> Result<Response, ContractError> {
        let tract = Cw721Controller::default();
        tract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        let tract = Cw721Controller::default();
        tract.query(deps, env, msg)
    }
}
