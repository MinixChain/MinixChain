pub use minix_runtime::{
    AccountId, AuraConfig, BalancesConfig, ComingIdConfig, ComingAuctionConfig,
    GenesisConfig, GrandpaConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,
    EthereumChainIdConfig, EthereumConfig, EvmConfig, GenesisAccount,
    AssetsConfig, AssetsBridgeConfig
};
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use serde::{Deserialize, Serialize};
use sc_chain_spec::ChainSpecExtension;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// The light sync state.
    ///
    /// This value will be set by the `sync-state rpc` implementation.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}


/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

#[cfg(feature = "runtime-benchmarks")]
pub fn benchmarks_config() -> Result<ChainSpec, String> {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "mini".into());
    properties.insert("tokenDecimals".into(), 8.into());

    Ok(ChainSpec::from_genesis(
        "Benchmarks",
        "benchmarks",
        ChainType::Development,
        move || {
            let caller: AccountId = frame_benchmarking::whitelisted_caller();
            minix_genesis(
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                caller.clone(),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    caller.clone()
                ],
                (caller.clone(), caller.clone(), caller),
                vec![],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
    ))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "mini".into());
    properties.insert("tokenDecimals".into(), 8.into());

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            minix_genesis(
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                (
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice")
                ),
                vec![],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "mini".into());
    properties.insert("tokenDecimals".into(), 8.into());

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            minix_genesis(
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                (
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                ),
                vec![
                    // Alith
                    H160::from(hex_literal::hex!["f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"]),
                    // Baltathar
                    H160::from(hex_literal::hex!["3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"]),
                    // Charleth
                    H160::from(hex_literal::hex!["798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc"]),
                    // Dorothy
                    H160::from(hex_literal::hex!["773539d4Ac0e786233D90A233654ccEE26a613D9"]),
                    // Ethan
                    H160::from(hex_literal::hex!["Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB"]),
                    // Faith
                    H160::from(hex_literal::hex!["C0F0f4ab324C46e55D02D0033343B4Be8A55532d"]),
                    // Goliath
                    H160::from(hex_literal::hex!["7BF369283338E12C90514468aa3868A551AB2929"]),
                    // Heath
                    H160::from(hex_literal::hex!["931f3600a299fd9B24cEfB3BfF79388D19804BeA"]),
                    // Ida
                    H160::from(hex_literal::hex!["C41C5F1123ECCd5ce233578B2e7ebd5693869d73"]),
                    // Judith
                    H160::from(hex_literal::hex!["2898FE7a42Be376C8BC7AF536A940F7Fd5aDd423"]),
                    // Gerald
                    H160::from(hex_literal::hex!["6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b"]),
                    // Frontier pre-funded account
                    H160::from(hex_literal::hex!["19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A"]),
                ],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
    ))
}

pub fn dev_evm_config() -> Result<ChainSpec, String> {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "mini".into());
    properties.insert("tokenDecimals".into(), 8.into());

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            minix_genesis(
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                ],
                (
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                ),
                vec![
                    // Alith
                    H160::from(hex_literal::hex!["f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"]),
                    // Baltathar
                    H160::from(hex_literal::hex!["3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"]),
                    // Charleth
                    H160::from(hex_literal::hex!["798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc"]),
                    // Dorothy
                    H160::from(hex_literal::hex!["773539d4Ac0e786233D90A233654ccEE26a613D9"]),
                    // Ethan
                    H160::from(hex_literal::hex!["Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB"]),
                    // Faith
                    H160::from(hex_literal::hex!["C0F0f4ab324C46e55D02D0033343B4Be8A55532d"]),
                    // Goliath
                    H160::from(hex_literal::hex!["7BF369283338E12C90514468aa3868A551AB2929"]),
                    // Heath
                    H160::from(hex_literal::hex!["931f3600a299fd9B24cEfB3BfF79388D19804BeA"]),
                    // Ida
                    H160::from(hex_literal::hex!["C41C5F1123ECCd5ce233578B2e7ebd5693869d73"]),
                    // Judith
                    H160::from(hex_literal::hex!["2898FE7a42Be376C8BC7AF536A940F7Fd5aDd423"]),
                    // Gerald
                    H160::from(hex_literal::hex!["6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b"]),
                    // Frontier pre-funded account
                    H160::from(hex_literal::hex!["19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A"]),
                ],
                get_account_id_from_seed::<sr25519::Public>("Alice"),
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Default::default(),
    ))
}

pub fn live_testnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/testnet-raw.json")[..])
}

pub fn live_mainnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/minix-raw.json")[..])
}

/// Configure initial storage state for FRAME modules.
pub fn minix_genesis(
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    coming_keys: (AccountId, AccountId, AccountId),
    addresses: Vec<H160>,
    assets_admin: AccountId,
) -> GenesisConfig {
    let wasm_binary = WASM_BINARY.unwrap();
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 100_000_000_000_u128))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
        coming_id: ComingIdConfig {
            // Assign network admin rights.
            high_admin_key: coming_keys.0,
            medium_admin_key: coming_keys.1.clone(),
            medium_admin_key2: coming_keys.1.clone(),
            medium_admin_key3: coming_keys.1,
            low_admin_key: coming_keys.2,
        },
        coming_auction: ComingAuctionConfig {
            admin_key: None
        },
        ethereum_chain_id: EthereumChainIdConfig { chain_id: 1500u64 },
        evm: EvmConfig {
            accounts: addresses
                .into_iter()
                .map(|addr| {
                    (
                        addr,
                        GenesisAccount {
                            balance: U256::from(100_000_000_000_u128),
                            nonce: Default::default(),
                            code: Default::default(),
                            storage: Default::default(),
                        },
                    )
                })
                .collect()
        },
        ethereum: EthereumConfig {},
        assets: AssetsConfig {
            assets: vec![
                // id, owner, is_sufficient, min_balance
                (1, assets_admin, true, 1),
            ],
            metadata: vec![
                // id, name, symbol, decimals
                (1, "Kusama".into(), "KSM".into(), 12),
            ],
            accounts: vec![],
        },
        assets_bridge: AssetsBridgeConfig {
            admin_key: None
        },
    }
}
