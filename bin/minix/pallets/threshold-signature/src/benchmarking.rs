//! Benchmarking setup for pallet-threshold-signature

use super::*;

#[allow(unused)]
use crate::Pallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, vec, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    generate_address {
        let caller: T::AccountId = whitelisted_caller();
        let abc = hex::decode("881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768").unwrap();
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let ac = hex::decode("b69af178463918a181a8549d2cfbe77884852ace9d8b299bddf69bedc33f6356").unwrap();
        let bc = hex::decode("a20c839d955cb10e58c6cbc75812684ad3a1a8f24a503e1c07f5e4944d974d3b").unwrap();
        let scripts = vec![abc, ab, ac, bc];
    }: _(RawOrigin::Signed(caller), scripts)
    verify {
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <T as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        assert!(AddrToScript::<T>::contains_key(addr));
    }

    verify_threshold_signature {
        let caller: T::AccountId = whitelisted_caller();
        let abc = hex::decode("881102cd9cf2ee389137a99a2ad88447b9e8b60c350cda71aff049233574c768").unwrap();
        let ab = hex::decode("7c9a72882718402bf909b3c1693af60501c7243d79ecc8cf030fa253eb136861").unwrap();
        let ac = hex::decode("b69af178463918a181a8549d2cfbe77884852ace9d8b299bddf69bedc33f6356").unwrap();
        let bc = hex::decode("a20c839d955cb10e58c6cbc75812684ad3a1a8f24a503e1c07f5e4944d974d3b").unwrap();
        let scripts = vec![abc, ab.clone(), ac, bc];
        let _ = Pallet::<T>::apply_generate_address(scripts);
        let tweaked = &hex::decode("001604bef08d1fe4cefb2e75a2b786287821546f6acbe89570acc5d5a9bd5049").unwrap();
        let addr = <T as frame_system::Config>::AccountId::decode(&mut &tweaked[..]).unwrap();
        let signature_ab = hex::decode("7227f84f853853527488ba5b9939c56dd4ecd0ae96687e0d8d4d5da10cb4e6651cb2aca89236f3c3766d80e3b2ab37c74abb91ad6bb66677a0f1e3bd7e68118f").unwrap();
        let message = b"We are legion!".to_vec();
        let call: <T as Config>::Call = frame_system::Call::<T>::remark(vec![0; 1 as usize]).into();
    }: _(RawOrigin::Signed(caller), addr, signature_ab, ab, message, Box::new(call))
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
