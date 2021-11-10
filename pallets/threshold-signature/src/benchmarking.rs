//! Benchmarking setup for pallet-threshold-signature

use super::*;

#[allow(unused)]
use crate::Pallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use sp_runtime::traits::Saturating;

benchmarks! {
    pass_script {
        let caller: T::AccountId = whitelisted_caller();
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b").unwrap();
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let tweaked = &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719").unwrap();
        let addr = <T as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88").unwrap();
        let message = 666666u32.to_be_bytes().to_vec();
        let script_hash = Pallet::<T>::compute_script_hash(caller.clone(), OpCode::Transfer, 10u32.into(), (0u32.into(), 10u32.into()));
    }: _(RawOrigin::Signed(caller), addr.clone(), signature_ab, ab, control_block, message, script_hash.clone())
    verify {
        assert_eq!(ScriptHashToAddr::<T>::get(script_hash), addr);
    }

    exec_script {
        let caller: T::AccountId = whitelisted_caller();
        let ab = hex::decode("744ffca9bc5f2fa2373823c5510cf757fbbcda8e257eb0c7142edfda693b2f7b").unwrap();
        let control_block = hex::decode("fa87fe21ee5bd74aa18a83b3c182f021f3154f93dbb41f238b8c4e540c626140461222205b7b12a3ab413e75d91d4c385c1f018c9fb77c342409a85f50b27634").unwrap();
        let tweaked = &hex::decode("3ee8244d248f1e06f72ab7d38ee7f25024d33f555eb585e167816f03c7cde719").unwrap();
        let addr = <T as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let existential_deposit = <T as pallet_balances::Config>::ExistentialDeposit::get();
        let _ = pallet_balances::Pallet::<T>::deposit_creating(&addr, existential_deposit.saturating_mul(10u32.into()));
        let signature_ab = hex::decode("98d683074a37ac9bf3d08d81899071109d099ad4a006bb84662db241e507806f253c515d5f02216ec88ef91f322b583c49ea4c0e88eebc3bab32663df8019f88").unwrap();
        let message = 666666u32.to_be_bytes().to_vec();
        let script_hash = Pallet::<T>::compute_script_hash(caller.clone(), OpCode::Transfer, existential_deposit, (0u32.into(), 10u32.into()));
        let _ = Pallet::<T>::apply_pass_script(addr, signature_ab, ab, control_block, message, script_hash);
    }: _(RawOrigin::Signed(caller.clone()), caller.clone(), OpCode::Transfer, existential_deposit, (0u32.into(), 10u32.into()))
    verify {
        assert_eq!(pallet_balances::Pallet::<T>::free_balance(caller), existential_deposit);
    }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
