// ! Runtime API definition required by threshold_signature RPC extensions.
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments, clippy::unnecessary_mut_passed)]
pub use pallet_threshold_signature::primitive::{Message, OpCode, Pubkey, ScriptHash, Signature};
use sp_runtime::AccountId32;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait ThresholdSignatureApi
    {
        fn compute_script_hash(
            account: AccountId32,
            call: OpCode,
            amount: u128,
            time_lock: (u32, u32),
        ) -> ScriptHash;
    }
}
