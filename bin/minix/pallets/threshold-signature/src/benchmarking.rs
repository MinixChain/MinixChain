//! Benchmarking setup for pallet-threshold-signature

use super::*;

#[allow(unused)]
use crate::Pallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_support::traits::Get;
use frame_system::RawOrigin;
use sp_runtime::traits::Saturating;

benchmarks! {
    pass_script {
        let caller: T::AccountId = whitelisted_caller();
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let control_block = vec![
            hex::decode("881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768").unwrap(),
            hex::decode("e17a23050f6f6db2f4218ce9f7c14edd21c5f24818157103c5a8524d7014c0dd").unwrap(),
            hex::decode("0bac21362eecf9223bc477d6dfbbe02066a911eba752faedb26d881c466ea80f").unwrap()];
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <T as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f").unwrap();
        let message = b"We are legion!".to_vec();
        let script_hash = Pallet::<T>::compute_script_hash(caller.clone(), OpCode::Transfer, 10u32.into(), (0u32.into(), 10u32.into()));
    }: _(RawOrigin::Signed(caller), addr.clone(), signature_ab, ab, control_block, message, script_hash.clone())
    verify {
        assert_eq!(ScriptHashToAddr::<T>::get(script_hash), addr);
    }

    exec_script {
        let caller: T::AccountId = whitelisted_caller();
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let control_block = vec![
            hex::decode("881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768").unwrap(),
            hex::decode("e17a23050f6f6db2f4218ce9f7c14edd21c5f24818157103c5a8524d7014c0dd").unwrap(),
            hex::decode("0bac21362eecf9223bc477d6dfbbe02066a911eba752faedb26d881c466ea80f").unwrap()];
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <T as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let existential_deposit = <T as pallet_balances::Config>::ExistentialDeposit::get();
        let _ = pallet_balances::Pallet::<T>::deposit_creating(&addr, existential_deposit.saturating_mul(10u32.into()));
        let signature_ab = hex::decode("7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f").unwrap();
        let message = b"We are legion!".to_vec();
        let script_hash = Pallet::<T>::compute_script_hash(caller.clone(), OpCode::Transfer, existential_deposit, (0u32.into(), 10u32.into()));
        let _ = Pallet::<T>::apply_pass_script(addr, signature_ab, ab, control_block, message, script_hash);
    }: _(RawOrigin::Signed(caller.clone()), OpCode::Transfer, existential_deposit, (0u32.into(), 10u32.into()))
    verify {
        assert_eq!(pallet_balances::Pallet::<T>::free_balance(caller), existential_deposit);
    }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
