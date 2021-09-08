// Copyright 2019-2020 ChainX Project Authors. Licensed under GPL-3.0.

//! Weights for pallet_threshold_signature
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-03, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/node-threshold-signature
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_threshold_signature
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/threshold-signature/src/weights.rs
// --template=./scripts/pallet-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_threshold_signature.
pub trait WeightInfo {
    fn generate_address() -> Weight;
    fn verify_threshold_signature() -> Weight;
}

/// Weights for pallet_threshold_signature using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn generate_address() -> Weight {
        (474_714_000_u64).saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn verify_threshold_signature() -> Weight {
        (886_249_000_u64).saturating_add(T::DbWeight::get().reads(1_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn generate_address() -> Weight {
        (474_714_000_u64).saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn verify_threshold_signature() -> Weight {
        (886_249_000_u64).saturating_add(RocksDbWeight::get().reads(1_u64))
    }
}
