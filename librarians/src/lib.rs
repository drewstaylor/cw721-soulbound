pub mod species;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Empty, Addr, Reply, SubMsgResult};
use cw2::set_contract_version;

pub use cw721_soulbound::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
pub use cw721::Expiration;

pub use crate::species::{Species, SapienceScale};

use cw721::{ContractInfoResponse};

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

pub type Cw721MetadataContract<'a> = cw721_soulbound::Cw721Contract<'a, Extension, Empty, Empty, Empty>;

pub type ExecuteMsg = cw721_soulbound::ExecuteMsg<Extension, Empty>;

const CONTRACT_NAME: &str = "crates.io:cw721-soulbound-librarians";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;

    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = ContractInfoResponse {
            name: msg.name,
            symbol: msg.symbol,
        };
        Cw721MetadataContract::default()
            .contract_info
            .save(deps.storage, &info)?;
        let minter = deps.api.addr_validate(&msg.minter)?;
        Cw721MetadataContract::default()
            .minter
            .save(deps.storage, &minter)?;
        Ok(Response::default())
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.result {
            SubMsgResult::Ok(_) => Ok(Response::default()),
            SubMsgResult::Err(_) => Err(ContractError::Unauthorized {}),
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
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
            description: Some("Ever since you became a Cyborg, you???ve been feeling pretty weird...".into()),
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
