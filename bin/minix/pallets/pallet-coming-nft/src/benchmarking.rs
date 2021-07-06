//! Benchmarking setup for pallet-coming-nft

use super::*;

#[allow(unused)]
use crate::Pallet as ComingNFT;
use pallet_coming_id as ComingId;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_std::vec;

// Alice
fn admin_account<AccountId: Decode + Default>() -> AccountId {
    let alice =
        hex_literal::hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
    AccountId::decode(&mut &alice[..]).unwrap_or_default()
}

benchmarks! {
    mint {
        let admin: T::AccountId = admin_account();
        let claim_cid: Cid = 1000000;
        let recipient: T::AccountId = account("recipient", 0, 0);
        let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());
        let b in 0 .. *T::BlockLength::get().max.get(DispatchClass::Normal) as u32;
        let card = vec![1; b as usize];

        assert!(
            ComingId::Pallet::<T>::register(
                RawOrigin::Signed(admin.clone()).into(),
                claim_cid,
                recipient_lookup,
            )
            .is_ok()
        );

    }: mint(RawOrigin::Signed(admin), claim_cid, card.clone())
    verify {
        assert_eq!(ComingNFT::<T>::card_of_cid(claim_cid), Some(card));
        assert_eq!(ComingNFT::<T>::owner_of_cid(claim_cid), Some(recipient.clone()));
        assert_eq!(ComingNFT::<T>::cids_of_owner(recipient), vec![claim_cid]);
    }

    transfer {
        let admin: T::AccountId = admin_account();
        let claim_cid: Cid = 1000000;
        let owner: T::AccountId = account("recipient", 0, 0);
        let owner_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(owner.clone());
        let recipient: T::AccountId = account("recipient", 0, 1);
        let recipient_lookup: <T::Lookup as StaticLookup>::Source = T::Lookup::unlookup(recipient.clone());
        let card = br#"{"name":"testcard"}"#.to_vec();

        assert!(
            ComingId::Pallet::<T>::register(
                RawOrigin::Signed(admin.clone()).into(),
                claim_cid,
                owner_lookup,
            )
            .is_ok()
        );

        assert!(
            ComingNFT::<T>::mint(
                RawOrigin::Signed(admin).into(),
                claim_cid,
                card.clone()
            )
            .is_ok()
        );

        assert_eq!(ComingNFT::<T>::cids_of_owner(owner.clone()), vec![claim_cid]);

    }: transfer(RawOrigin::Signed(owner.clone()), claim_cid, recipient_lookup)
    verify {
        assert_eq!(ComingNFT::<T>::card_of_cid(claim_cid), Some(card));
        assert_eq!(ComingNFT::<T>::owner_of_cid(claim_cid), Some(recipient.clone()));
        assert_eq!(ComingNFT::<T>::cids_of_owner(recipient), vec![claim_cid]);

        assert!(ComingNFT::<T>::cids_of_owner(owner).is_empty());
    }
}

impl_benchmark_test_suite!(
    ComingNFT,
    crate::mock::new_test_ext(super::admin_account()),
    crate::mock::Test,
);
