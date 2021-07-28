//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use minix_runtime::{opaque::Block, AccountId, Balance, Index};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
use sp_block_builder::BlockBuilder;
pub use sc_rpc_api::DenyUnsafe;
use sp_transaction_pool::TransactionPool;

// EVM
use std::collections::BTreeMap;
use minix_runtime::Hash;
use sc_rpc::SubscriptionTaskExecutor;
use sp_runtime::traits::BlakeTwo256;
use sc_network::NetworkService;
use sc_client_api::{
	backend::{StorageProvider, Backend, StateBackend, AuxStore},
	client::BlockchainEvents
};
use fc_rpc_core::types::{PendingTransactions, FilterPool};
use jsonrpc_pubsub::manager::SubscriptionManager;
use pallet_ethereum::EthereumStorageSchema;
use fc_rpc::{StorageOverride, SchemaV1Override, OverrideHandle, RuntimeApiStorageOverride};

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The Node authority flag
	pub is_authority: bool,
	/// Whether to enable dev signer
	pub enable_dev_signer: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// Ethereum pending transactions.
	pub pending_transactions: PendingTransactions,
	/// EthFilterApi pool.
	pub filter_pool: Option<FilterPool>,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
	/// Maximum number of logs in a query.
	pub max_past_logs: u32,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, BE>(
	deps: FullDeps<C, P>,
	subscription_task_executor: SubscriptionTaskExecutor
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> where
	BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: BlockchainEvents<Block>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: pallet_coming_id_rpc::ComingIdRuntimeApi<Block, AccountId>,
	P: TransactionPool<Block=Block> + 'static,
{
	use substrate_frame_rpc_system::{FullSystem, SystemApi};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
	use pallet_coming_id_rpc::{ComingId, ComingIdApi};
	use fc_rpc::{
		EthApi, EthApiServer, EthFilterApi, EthFilterApiServer, NetApi, NetApiServer,
		EthPubSubApi, EthPubSubApiServer, Web3Api, Web3ApiServer, EthDevSigner, EthSigner,
		HexEncodedIdProvider,
	};

	let mut io = jsonrpc_core::IoHandler::default();
	let FullDeps {
		client,
		pool,
		deny_unsafe,
		is_authority,
		network,
		pending_transactions,
		filter_pool,
		backend,
		max_past_logs,
		enable_dev_signer,
	} = deps;

	io.extend_with(
		SystemApi::to_delegate(FullSystem::new(client.clone(), pool.clone(), deny_unsafe))
	);

	io.extend_with(
		TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone()))
	);

	io.extend_with(
		ComingIdApi::to_delegate(ComingId::new(client.clone()))
	);

	let mut signers = Vec::new();
	if enable_dev_signer {
		signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
	}
	let mut overrides_map = BTreeMap::new();
	overrides_map.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone())) as Box<dyn StorageOverride<_> + Send + Sync>
	);

	let overrides = Arc::new(OverrideHandle {
		schemas: overrides_map,
		fallback: Box::new(RuntimeApiStorageOverride::new(client.clone())),
	});

	io.extend_with(
		EthApiServer::to_delegate(EthApi::new(
			client.clone(),
			pool.clone(),
			minix_runtime::TransactionConverter,
			network.clone(),
			pending_transactions.clone(),
			signers,
			overrides.clone(),
			backend,
			is_authority,
			max_past_logs,
		))
	);

	if let Some(filter_pool) = filter_pool {
		io.extend_with(
			EthFilterApiServer::to_delegate(EthFilterApi::new(
				client.clone(),
				filter_pool.clone(),
				500 as usize, // max stored filters
				overrides.clone(),
				max_past_logs,
			))
		);
	}

	io.extend_with(
		NetApiServer::to_delegate(NetApi::new(
			client.clone(),
			network.clone(),
			// Whether to format the `peer_count` response as Hex (default) or not.
			true,
		))
	);

	io.extend_with(
		Web3ApiServer::to_delegate(Web3Api::new(
			client.clone(),
		))
	);

	io.extend_with(
		EthPubSubApiServer::to_delegate(EthPubSubApi::new(
			pool,
			client,
			network,
			SubscriptionManager::<HexEncodedIdProvider>::with_id_provider(
				HexEncodedIdProvider::default(),
				Arc::new(subscription_task_executor)
			),
			overrides
		))
	);

	io
}
