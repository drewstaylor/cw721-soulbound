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

pub type Cw721MetadataContract<'a> = Cw721Contract<'a, Extension, Empty>;

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
        Cw721MetadataContract::default().instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
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
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::{Cw721Query};

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw721MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "archid token".to_string(),
            symbol: "AID".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let species = Species {
            name: "Cyborg type 3 (Human)".to_string(),
            sapience_level: SapienceScale::High,
        };

        let metadata_extension = Some(Metadata {
            name: Some("Traveler Name".into()),
            description: Some("Ever since you became a Cyborg, you’ve been feeling pretty weird...".into()),
            image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
            cyberdization_date: Some(1671221764),
            dna: Some("DNA String".into()), // XXX TODO (drew): Re-work the way DNA strings are built and parsed in Potion contract
            species: Some(species.name),
            sapience: Some(species.sapience_level),
            home_planet: Some(Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7")),
            identity: Some(Addr::unchecked("archway1f395p0gg67mmfd5zcqvpnp9cxnu0hg6r9hfczq")),
        });

        let token_id = "1";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: CREATOR.to_string(),
            token_uri: None,
            extension: metadata_extension,
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();

        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }
}