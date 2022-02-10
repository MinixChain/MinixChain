//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use minix_runtime::{opaque::Block, AccountId, Balance, Index};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(deps: FullDeps<C, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    C::Api: pallet_coming_id_rpc::ComingIdRuntimeApi<Block, AccountId>,
    P: TransactionPool + 'static,
    C::Api: pallet_threshold_signature_rpc::ThresholdSignatureRuntimeApi<Block>,
    C::Api: pallet_coming_auction_rpc::ComingAuctionRuntimeApi<Block, Balance>,
    C::Api: pallet_coming_reputation_rpc::ComingReputationRuntimeApi<Block, AccountId>,
{
    use pallet_coming_auction_rpc::{ComingAuction, ComingAuctionApi};
    use pallet_coming_reputation_rpc::{ComingReputation, ComingReputationApi};
    use pallet_coming_id_rpc::{ComingId, ComingIdApi};
    use pallet_threshold_signature_rpc::{ThresholdSignature, ThresholdSignatureApi};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
    use substrate_frame_rpc_system::{FullSystem, SystemApi};

    let mut io = jsonrpc_core::IoHandler::default();
    let FullDeps {
        client,
        pool,
        deny_unsafe,
    } = deps;

    io.extend_with(SystemApi::to_delegate(FullSystem::new(
        client.clone(),
        pool,
        deny_unsafe,
    )));

    io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
        client.clone(),
    )));

    io.extend_with(ThresholdSignatureApi::to_delegate(ThresholdSignature::new(
        client.clone(),
    )));

    io.extend_with(ComingAuctionApi::to_delegate(ComingAuction::new(
        client.clone(),
    )));

    io.extend_with(ComingReputationApi::to_delegate(ComingReputation::new(
        client.clone(),
    )));

    io.extend_with(ComingIdApi::to_delegate(ComingId::new(client)));

    io
}
