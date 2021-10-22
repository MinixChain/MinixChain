pub use minix_runtime::{
    AccountId, AuraConfig, BalancesConfig, ComingIdConfig, GenesisConfig,
    GrandpaConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,
    ComingAuctionConfig
};
use sc_service::{ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use hex_literal::hex;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

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
                (caller.clone(), caller.clone(), caller)
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn development_config() -> Result<ChainSpec, String> {
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
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                )
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        None,
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let mut properties = Properties::new();
    properties.insert("tokenSymbol".into(), "mini".into());
    properties.insert("tokenDecimals".into(), 8.into());

    Ok(ChainSpec::from_genesis(
        // Name
        "MiniX",
        // ID
        "MiniX",
        ChainType::Live,
        move || minix_genesis(
            // Initial PoA authorities
            vec![
                (
                    hex!("764f3c0898c003ef987dcc1289c73e5e8e6030f29747e56c8fe0c3166b04c23e").unchecked_into(),
                    hex!("9a6ddc5b7894ea44487878d5b4623ec04c280c10ab4b9a433dd114d55cb52998").unchecked_into(),
                ),
                (
                    hex!("acc024d777338da2cc7d1eab74b4f6bb6c0f6b0119a5a60c2b2911f40c0ebc71").unchecked_into(),
                    hex!("e2ca504a95fdd9bf8491b24acba441be739b144fd36a6a4667f0158eb1bab718").unchecked_into(),
                ),
            ],
            // Sudo account
            hex!("34eeb344c7e8176df0672e04e7a57cb2074f052cdbee7df02eec0c1c937cce52").into(),
            // Pre-funded accounts
            vec![
                hex!("34eeb344c7e8176df0672e04e7a57cb2074f052cdbee7df02eec0c1c937cce52").into(),
                hex!("fc4ea146bf1f19bc7b828c19be1f7d764c55108c8aaf6075d00c9fa7da1eca75").into(),
                hex!("74092de518c6394d5ec2d8915c22822d0d62cc699ce8d9177c38e812a3ed3565").into(),
                hex!("f412fd28e2835691047a49d83608c19249711b36d09c61c634566c003b3bc660").into(),
            ],
            // coming-keys
            (
                hex!("fc4ea146bf1f19bc7b828c19be1f7d764c55108c8aaf6075d00c9fa7da1eca75").into(),
                hex!("74092de518c6394d5ec2d8915c22822d0d62cc699ce8d9177c38e812a3ed3565").into(),
                hex!("f412fd28e2835691047a49d83608c19249711b36d09c61c634566c003b3bc660").into(),
            )
        ),
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties),
        // Extensions
        None,
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
    coming_keys: (AccountId, AccountId, AccountId)
) -> GenesisConfig {
    let wasm_binary = WASM_BINARY.unwrap();
    GenesisConfig {
        frame_system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        pallet_balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1_000_000_000_000u128))
                .collect(),
        },
        pallet_aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        pallet_grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        pallet_sudo: SudoConfig {
            // Assign network admin rights.
            key: root_key.clone(),
        },
        pallet_coming_id: ComingIdConfig {
            // Assign network admin rights.
            high_admin_key: coming_keys.0,
            medium_admin_key: coming_keys.1,
            low_admin_key: coming_keys.2,
        },
        pallet_coming_auction: ComingAuctionConfig {
            admin_key: Some(root_key)
        }
    }
}
