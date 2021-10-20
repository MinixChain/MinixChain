pub use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
pub use pallet_threshold_signature_rpc_runtime_api::{
    Message, OpCode, Pubkey, Signature, ThresholdSignatureApi as ThresholdSignatureRuntimeApi,
};
use sp_api::{BlockId, BlockT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::AccountId32;
use sp_std::{fmt::Debug, marker::PhantomData, sync::Arc};

/// The call to runtime failed.
pub const RUNTIME_ERROR: i64 = 1;

#[rpc]
pub trait ThresholdSignatureApi<BlockHash> {
    /// Use the params to compute script hash.
    #[rpc(name = "ts_computeScriptHash")]
    fn compute_script_hash(
        &self,
        account: AccountId32,
        call: OpCode,
        amount: u128,
        time_lock: (u32, u32),
        at: Option<BlockHash>,
    ) -> Result<String>;
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
    fn compute_script_hash(
        &self,
        account: AccountId32,
        call: OpCode,
        amount: u128,
        time_lock: (u32, u32),
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<String> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));
        Ok(hex::encode(
            api.compute_script_hash(&at, account, call, amount, time_lock)
                .map_err(runtime_error_into_rpc_err)?,
        ))
    }
}

/// Converts a runtime trap into an RPC error.
pub fn runtime_error_into_rpc_err(err: impl Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime trapped".into(),
        data: Some(format!("{:?}", err).into()),
    }
}
