//! Benchmarking setup for pallet-coming-id

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_std::vec;

benchmarks! {
    up_grade {
        let admin: T::AccountId = whitelisted_caller();
        let cid: Cid = 1000000;
        let grade: Grade = Grade {key1:1,key2:2,key3:3};
    }: up_grade(RawOrigin::Signed(admin), cid, grade)
    verify {
        assert_eq!(CidGrade::<T>::get(cid),Grade {key1:1,key2:2,key3:3});
    }
}
