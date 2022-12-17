use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr, Reply, SubMsgResult};

pub use crate::msg::{ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
pub use crate::species::{Species, SapienceScale};
pub use crate::state::Cw721Contract;
pub use crate::error::ContractError;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
    pub name: Option<String>,           // A human readable username (name is used for interoperability with marketplaces)
    pub description: Option<String>,    // Description is also for interoperability with marketplaces
    pub image: Option<String>,
    pub cyberdization_date: Option<u64>,
    pub dna: Option<String>,
    pub species: Option<String>,
    pub sapience: Option<SapienceScale>,
    pub home_planet: Option<Addr>,
    pub identity: Option<Addr>,         // The owner's wallet address
}

pub type Extension = Option<Metadata>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let contract = Cw721Contract::<Extension, Empty>::default();
        contract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        let contract = Cw721Contract::<Extension, Empty>::default();
        contract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.result {
            SubMsgResult::Ok(_) => Ok(Response::default()),
            SubMsgResult::Err(_) => Err(ContractError::Unauthorized {}),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        let contract = Cw721Contract::<Extension, Empty>::default();
        contract.query(deps, env, msg)
    }
}