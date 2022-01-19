//! Benchmarking setup for pallet-coming-auction

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as ComingAuction;
use frame_benchmarking::{whitelisted_caller, benchmarks};
use frame_system::RawOrigin;
use frame_support::{
    assert_ok, dispatch::DispatchResult
};
use pallet_coming_id as ComingId;
use sp_runtime::traits::StaticLookup;
use sp_std::convert::From;
use sp_std::vec;

const MAX_REMINT: u8 = 32u8;

fn create_auction<T: Config>() -> DispatchResult {
    let caller = whitelisted_caller();
    let cid: Cid = 1_000_000;
    let start_price = BalanceOf::<T>::from(1_000_000_000u32);
    let end_price  = BalanceOf::<T>::from(100_000_000u32);
    let duration = T::BlockNumber::from(100u32);
    let bid_price = BalanceOf::<T>::from(1_000_000_000u32);

    <T as crate::Config>::Currency::make_free_balance_be(
        &caller,
        bid_price
    );

    assert_ok!(
        ComingId::Pallet::<T>::register(
            RawOrigin::Signed(caller.clone()).into(),
            cid,
            T::Lookup::unlookup(caller.clone()),
        )
    );

    assert_ok!(
        ComingAuction::<T>::create(
            RawOrigin::Signed(caller.clone()).into(),
            cid,
            start_price,
            end_price,
            duration
        )
    );

    Ok(())
}

fn create_auction_after_remint<T: Config>() -> DispatchResult {
    let caller = whitelisted_caller();
    let cid: Cid = 1_000_000;
    let start_price = BalanceOf::<T>::from(1_000_000_000u32);
    let end_price  = BalanceOf::<T>::from(100_000_000u32);
    let duration = T::BlockNumber::from(100u32);
    let bid_price = BalanceOf::<T>::from(1_000_000_000u32);

    <T as crate::Config>::Currency::make_free_balance_be(
        &caller,
        bid_price
    );

    assert_ok!(
        ComingId::Pallet::<T>::register(
            RawOrigin::Signed(caller.clone()).into(),
            cid,
            T::Lookup::unlookup(caller.clone()),
        )
    );

    assert_ok!(
        ComingAuction::<T>::remint(
            RawOrigin::Signed(caller.clone()).into(),
            cid,
            vec![],
            100u8
        )
    );

    assert_ok!(
        ComingAuction::<T>::create(
            RawOrigin::Signed(caller.clone()).into(),
            cid,
            start_price,
            end_price,
            duration
        )
    );

    Ok(())
}

fn prepare_remint<T: Config>(caller: &T::AccountId, cid: Cid) -> DispatchResult {
    <T as Config>::Currency::make_free_balance_be(
        &caller,
        u128::MAX.unique_saturated_into(),
    );

    assert_ok!(
        ComingId::Pallet::<T>::register(
            RawOrigin::Signed(caller.clone()).into(),
            cid,
            T::Lookup::unlookup(caller.clone()),
        )
    );

    Ok(())
}

benchmarks! {
    create {
        let caller: T::AccountId = whitelisted_caller();
        let cid: Cid = 1_000_000;
        let start_price = BalanceOf::<T>::from(1_000_000_000u32);
        let end_price  = BalanceOf::<T>::from(100_000_000u32);
        let duration = T::BlockNumber::from(100u32);

        assert_ok!(
            ComingId::Pallet::<T>::register(
                RawOrigin::Signed(caller.clone()).into(),
                cid,
                T::Lookup::unlookup(caller.clone()),
            )
        );

    }: _(RawOrigin::Signed(caller), cid, start_price, end_price, duration)
    verify {
        assert!(ComingAuction::<T>::get_auction(cid).is_some());
    }

    bid {
        let caller: T::AccountId = whitelisted_caller();
        let cid: Cid = 1_000_000;
        let bid_price = BalanceOf::<T>::from(1_000_000_000u32);

        create_auction_after_remint::<T>()?;
    }: _(RawOrigin::Signed(caller.clone()), cid, bid_price)
    verify {
        assert_eq!(<T as Config>::ComingNFT::owner_of_cid(cid), Some(caller));
        assert!(ComingAuction::<T>::get_auction(cid).is_none());
        assert!(<T as Config>::ComingNFT::card_of_meta(cid).is_some());
    }

    remint {
        let caller: T::AccountId = whitelisted_caller();
        let cid: Cid = 1_000_000;
        let b in 0..(MAX_REMINT as u32) => ();
        let card = vec![0u8; (T::MaxDataSize::get() / 1024 * b) as usize];

        prepare_remint::<T>(&caller, cid)?;

    }: _(RawOrigin::Signed(caller.clone()), cid, card, 0u8)
    verify {
        assert!(<T as Config>::ComingNFT::card_of_meta(cid).is_some());
    }

    cancel {
        let caller: T::AccountId = whitelisted_caller();
        let cid: Cid = 1_000_000;

        create_auction::<T>()?;
    }: cancel(RawOrigin::Signed(caller), cid)
    verify {
        assert!(ComingAuction::<T>::get_auction(cid).is_none());
    }

    pause {
        let admin: T::AccountId = whitelisted_caller();
    }: pause(RawOrigin::Signed(admin))
    verify {
        assert!(ComingAuction::<T>::is_in_emergency());
    }

    unpause {
        let admin: T::AccountId = whitelisted_caller();

        assert_ok!(
            ComingAuction::<T>::pause(
                RawOrigin::Signed(admin.clone()).into(),
            )
        );
        assert!(ComingAuction::<T>::is_in_emergency());

    }: unpause(RawOrigin::Signed(admin))
    verify {
        assert!(!ComingAuction::<T>::is_in_emergency());
    }

    cancel_when_pause {
        let admin: T::AccountId = whitelisted_caller();
        let cid: Cid = 1_000_000;

        create_auction::<T>()?;

        assert_ok!(
            ComingAuction::<T>::pause(
                RawOrigin::Signed(admin.clone()).into(),
            )
        );
        assert!(ComingAuction::<T>::is_in_emergency());

    }: cancel_when_pause(RawOrigin::Signed(admin), cid)
    verify {
        assert!(ComingAuction::<T>::get_auction(cid).is_none());
    }

    set_fee_point {
        let admin: T::AccountId = whitelisted_caller();
        let max_point = 255u8;
    }: set_fee_point(RawOrigin::Signed(admin), max_point)
    verify {
        assert_eq!(ComingAuction::<T>::get_fee_point(), max_point);
    }

    set_remint_point {
        let admin: T::AccountId = whitelisted_caller();
        let max_point = 255u8;
    }: set_remint_point(RawOrigin::Signed(admin), max_point)
    verify {
        assert_eq!(ComingAuction::<T>::remint_point(), max_point);
    }

    set_admin {
       let new_admin: T::AccountId = whitelisted_caller();
       let new_admin_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(new_admin.clone());

    }: set_admin(RawOrigin::Root, new_admin_lookup)
    verify {
        assert!(ComingAuction::<T>::is_admin(new_admin));
    }
}
