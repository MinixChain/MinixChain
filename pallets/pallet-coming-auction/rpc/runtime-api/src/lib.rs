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

//! Runtime API definition for pallet-coming-auction module.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]

use codec::Codec;
pub use pallet_coming_auction::Cid;
use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeDisplay};

sp_api::decl_runtime_apis! {
    pub trait ComingAuctionApi<Balance> where
        Balance: Codec + AtLeast32BitUnsigned + MaybeDisplay,
    {
        fn get_price(cid: Cid) -> Balance;
        fn get_remint_fee(cid: Cid) -> Balance;
    }
}
