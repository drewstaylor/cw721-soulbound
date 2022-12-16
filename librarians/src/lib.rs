use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Empty, Addr, Reply, SubMsgResult};
use cw2::set_contract_version;
pub use cw721_soulbound::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};
use cw721::{ContractInfoResponse};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Subdomain {
    pub name: Option<String>,
    pub resolver: Option<Addr>,
    pub minted: Option<bool>,
    pub expiry: Option<u64>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Account {
  pub username: Option<String>,
  pub profile: Option<String>,
  pub account_type: Option<String>,
  pub verfication_hash: Option<String>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Website {
  pub url: Option<String>,
  pub domain: Option<String>,
  pub verfication_hash: Option<String>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
  pub name: Option<String>,         // e.g. for interoperability with external marketplaces
  pub description: Option<String>,  // e.g. ibid.
  pub image: Option<String>,        // e.g. ibid.
  pub expiry: Option<u64>,
  pub domain: Option<String>,
  pub subdomains: Option<Vec<Subdomain>>,
  pub accounts: Option<Vec<Account>>,
  pub websites: Option<Vec<Website>>,
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

        let resolver_addr = Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7".to_string()); 

        let subdomain1 = Subdomain {
            name: Some("game".to_string()),
            resolver: Some(resolver_addr.clone()),
            minted: Some(false),
            expiry: Some(1234567),
        };
        let subdomain2 = Subdomain {
            name: Some("dapp".to_string()),
            resolver: Some(resolver_addr.clone()),
            minted: Some(false),
            expiry: Some(1234567),
        };
        let subdomain3 = Subdomain {
            name: Some("market".to_string()),
            resolver: Some(resolver_addr.clone()),
            minted: Some(false),
            expiry: Some(1234567),
        };

        let subdomains = vec![
            subdomain1, 
            subdomain2, 
            subdomain3
        ];

        let accounts = vec![
            Account {
                username: Some("drew.taylor@philabs.xyz".to_string()),
                profile: None,
                account_type: Some("email".to_string()),
                verfication_hash: None,
            },
            Account {
                username: Some("@chainofinsight".to_string()),
                profile: Some("twitter.com/chainofinsight".to_string()),
                account_type: Some("twitter".to_string()),
                verfication_hash: None,
            }
        ];
    
        let websites = vec![
            Website {
                url: Some("drewstaylor.com".to_string()),
                domain: Some("drewstaylor.arch".to_string()),
                verfication_hash: None,
            },
            Website {
                url: Some("game.drewstaylor.com".to_string()),
                domain: Some("game.drewstaylor.arch".to_string()),
                verfication_hash: None,
            },
            Website {
                url: Some("dapp.drewstaylor.com".to_string()),
                domain: Some("dapp.drewstaylor.arch".to_string()),
                verfication_hash: None,
            },
            Website {
                url: Some("market.drewstaylor.com".to_string()),
                domain: Some("market.drewstaylor.arch".to_string()),
                verfication_hash: None,
            }
        ];
    
        let metadata_extension = Some(Metadata {
            name: Some("drewstaylor.arch".into()),
            description: Some("default token description".into()),
            image: Some("ipfs://QmZdPdZzZum2jQ7jg1ekfeE3LSz1avAaa42G6mfimw9TEn".into()),
            domain: Some("drewstaylor.arch".into()),
            expiry: Some(1234567),
            subdomains: Some(subdomains),
            accounts: Some(accounts),
            websites: Some(websites),
        });

        let token_id = "drewstaylor.arch";
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
