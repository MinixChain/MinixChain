use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
pub use pallet_threshold_signature_rpc_runtime_api::{
    Message, Script, Signature, ThresholdSignatureApi as ThresholdSignatureRuntimeApi,
};
use sp_api::{BlockId, BlockT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::AccountId32;
use sp_std::{marker::PhantomData, sync::Arc};

#[rpc]
pub trait ThresholdSignatureApi<BlockHash> {
    /// Use the params to verify whether the threshold_signature apply is valid.
    #[rpc(name = "threshold_signature_verify")]
    fn verify_threshold_signature(
        &self,
        addr: AccountId32,
        signature: Signature,
        script: Script,
        message: Message,
        at: Option<BlockHash>,
    ) -> Result<bool>;
}

/// A struct that implements the [`ThresholdSignatureApi`].
pub struct ThresholdSignature<C, P> {
    client: Arc<C>,
    _marker: PhantomData<P>,
}

impl<C, P> ThresholdSignature<C, P> {
    /// Create new `ThresholdSignature` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block> ThresholdSignatureApi<<Block as BlockT>::Hash> for ThresholdSignature<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ThresholdSignatureRuntimeApi<Block>,
{
    fn verify_threshold_signature(
        &self,
        addr: AccountId32,
        signature: Signature,
        script: Script,
        message: Message,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<bool> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));
        Ok(api
            .verify_threshold_signature(&at, addr, signature, script, message)
            .is_ok())
    }
}
