// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the pallet-coming-auction module.

use std::sync::Arc;
use std::convert::TryInto;
use codec::Codec;
use sp_blockchain::HeaderBackend;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_runtime::{generic::BlockId, traits::{Block as BlockT, AtLeast32BitUnsigned, MaybeDisplay}};
use sp_api::ProvideRuntimeApi;
use sp_rpc::number::NumberOrHex;
use pallet_coming_auction_rpc_runtime_api::Cid;
pub use pallet_coming_auction_rpc_runtime_api::ComingAuctionApi as ComingAuctionRuntimeApi;



#[rpc]
pub trait ComingAuctionApi<BlockHash, Balance> {
	#[rpc(name = "get_price")]
	fn get_price(
		&self,
		cid: Cid,
		at: Option<BlockHash>
	) -> Result<NumberOrHex>;
}

/// A struct that implements the [`ComingAuctionApi`].
pub struct ComingAuction<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> ComingAuction<C, P> {
	/// Create new `ComingId` with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
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

impl<C, Block, Balance> ComingAuctionApi<<Block as BlockT>::Hash, Balance>
	for ComingAuction<C, Block>
where
	Block: BlockT,
	C: 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: ComingAuctionRuntimeApi<Block, Balance>,
	Balance: Codec + AtLeast32BitUnsigned + Copy + TryInto<NumberOrHex> + MaybeDisplay,
{
	fn get_price(
		&self,
		cid: Cid,
		at: Option<<Block as BlockT>::Hash>
	) -> Result<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash
		));

		api.get_price(&at, cid)
			.map(|price|{
				price.try_into().map_err(|_| RpcError {
					code: ErrorCode::InvalidParams,
					message: format!("{} doesn't fit in NumberOrHex representation", price),
					data: None,
				})
			})
			.map_err(|e| {
				RpcError {
					code: ErrorCode::ServerError(Error::RuntimeError.into()),
					message: "Unable to get price.".into(),
					data: Some(format!("{:?}", e).into()),
				}
			})?
	}
}
