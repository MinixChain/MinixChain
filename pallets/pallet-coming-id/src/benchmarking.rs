//! Benchmarking setup for pallet-coming-id

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_std::vec;

use crate as ComingId;

fn prepare_bond<T: Config>(caller: &T::AccountId, cid: Cid) -> DispatchResult {
    assert_ok!(ComingId::Pallet::<T>::register(
        RawOrigin::Signed(caller.clone()).into(),
        cid,
        T::Lookup::unlookup(caller.clone()),
    ));

    Ok(())
}

fn prepare_unbond<T: Config>(
    caller: &T::AccountId,
    cid: Cid,
    bond_data: BondData,
) -> DispatchResult {
    assert_ok!(ComingId::Pallet::<T>::register(
        RawOrigin::Signed(caller.clone()).into(),
        cid,
        T::Lookup::unlookup(caller.clone()),
    ));

    assert_ok!(ComingId::Pallet::<T>::bond(
        RawOrigin::Signed(caller.clone()).into(),
        cid,
        bond_data,
    ));

    Ok(())
}

benchmarks! {
    register {
        let admin: T::AccountId = whitelisted_caller();
        let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(admin.clone());
        let claim_cid: Cid = 1000000;
    }: register(RawOrigin::Signed(admin), claim_cid, recipient_lookup)
    verify {
        assert!(Distributed::<T>::get(claim_cid).is_some());
    }

    bond {
        let common_user: T::AccountId = whitelisted_caller();
        let claim_cid: Cid = 1000000;

        let b in 0..(T::MaxDataSize::get() / 1024);

        let bond_data = BondData{
            bond_type: b as u16,
            data: vec![0u8; (1024 * b) as usize],
        };

        prepare_bond::<T>(&common_user, claim_cid)?;

    }: bond(RawOrigin::Signed(common_user.clone()), claim_cid, bond_data)
    verify {
        let option = Distributed::<T>::get(claim_cid);
        assert!(option.is_some());

        let cid_details = option.unwrap();
        assert_eq!(cid_details.owner, common_user);
    }

    unbond {
        let common_user: T::AccountId = whitelisted_caller();
        let claim_cid: Cid = 1000000;
        let bond_data = BondData {
            bond_type: 1u16,
            data: b"benchmark".to_vec(),
        };

        prepare_unbond::<T>(&common_user, claim_cid, bond_data)?;

    }: unbond(RawOrigin::Signed(common_user.clone()), claim_cid, 1u16)
    verify {
        let option = Distributed::<T>::get(claim_cid);
        assert!(option.is_some());

        let cid_details = option.unwrap();
        assert_eq!(cid_details.owner, common_user);
    }
}
