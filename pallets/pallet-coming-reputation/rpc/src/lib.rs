// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the pallet-coming-id module.

use codec::Codec;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
pub use pallet_coming_reputation_rpc_runtime_api::ComingReputationApi as ComingReputationRuntimeApi;
use pallet_coming_reputation_rpc_runtime_api::{Cid, ReputationGrade};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait ComingReputationApi<BlockHash, AccountId> {
    #[rpc(name = "get_reputation_grade")]
    fn get_reputation_grade(
        &self,
        cid: Cid,
        at: Option<BlockHash>,
    ) -> Result<Option<ReputationGrade>>;
}

/// A struct that implements the [`ComingIdApi`].
pub struct ComingReputation<C, P> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<P>,
}

impl<C, P> ComingReputation<C, P> {
    /// Create new `ComingId` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

/// Error type of this RPC api.
pub enum Error {
    /// The transaction was not decodable.
    DecodeError,
    /// The call to runtime failed.
    RuntimeError,
}

impl From<Error> for i64 {
    fn from(e: Error) -> i64 {
        match e {
            Error::RuntimeError => 1,
            Error::DecodeError => 2,
        }
    }
}

impl<C, Block, AccountId> ComingReputationApi<<Block as BlockT>::Hash, AccountId>
    for ComingReputation<C, Block>
where
    Block: BlockT,
    AccountId: Codec,
    C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ComingReputationRuntimeApi<Block, AccountId>,
{
    fn get_reputation_grade(
        &self,
        cid: Cid,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<ReputationGrade>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        api.get_reputation_grade(&at, cid).map_err(|e| RpcError {
            code: ErrorCode::ServerError(Error::RuntimeError.into()),
            message: "Unable to get account id.".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}
