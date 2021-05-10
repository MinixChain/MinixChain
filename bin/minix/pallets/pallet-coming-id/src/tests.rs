use super::Event as ComingIdEvent;
use crate::{mock::*, BondData, CidDetails, Error};
use frame_support::{assert_noop, assert_ok};

const ADMIN: u64 = 1;
const RESERVE2: u64 = 2;
const RESERVE3: u64 = 3;
const COMMUNITY_ALICE: u64 = 100000;
const COMMUNITY_BOB: u64 = 999999;
const COMMON_CHARLIE: u64 = 1000000;
const COMMON_DAVE: u64 = 999999999999;

#[test]
fn it_works_for_regular_value() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1000000,
            COMMON_CHARLIE
        ));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            BondData {
                bond_type: 1u16,
                data: vec![]
            }
        ));
        assert_ok!(ComingId::unbond(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            1u16
        ));
        assert_ok!(ComingId::transfer(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            COMMON_DAVE
        ));

        let events = vec![
            Event::pallet_coming_id(ComingIdEvent::Registered(RESERVE2, 1)),
            Event::pallet_coming_id(ComingIdEvent::Registered(COMMON_CHARLIE, 1000000)),
            Event::pallet_coming_id(ComingIdEvent::Bonded(COMMON_CHARLIE, 1000000, 1)),
            Event::pallet_coming_id(ComingIdEvent::UnBonded(COMMON_CHARLIE, 1000000, 1)),
            Event::pallet_coming_id(ComingIdEvent::Transferred(
                COMMON_CHARLIE,
                COMMON_DAVE,
                1000000,
            )),
        ];

        expect_events(events);
    });
}

#[test]
fn register_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        // (1) Error::RequireAdmin
        assert_noop!(
            ComingId::register(Origin::signed(COMMUNITY_ALICE), 1, RESERVE2),
            Error::<Test>::RequireAdmin
        );

        // (2) Error::InvalidCid
        assert_noop!(
            ComingId::register(Origin::signed(ADMIN), 0, RESERVE2),
            Error::<Test>::InvalidCid
        );
        assert_noop!(
            ComingId::register(Origin::signed(ADMIN), 1_000_000_000_000, RESERVE2),
            Error::<Test>::InvalidCid
        );

        // (3) Event::Registered
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1));

        // (4) Error::DistributedCid
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            100000,
            COMMUNITY_ALICE
        ));
        assert_noop!(
            ComingId::register(Origin::signed(ADMIN), 100000, COMMUNITY_BOB),
            Error::<Test>::DistributedCid
        );

        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_001,
            RESERVE2
        ));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1000001));
    });
}

#[test]
fn transfer_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            100000,
            COMMUNITY_ALICE
        ));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1000000,
            COMMON_CHARLIE
        ));
        expect_event(ComingIdEvent::Registered(COMMON_CHARLIE, 1000000));

        // 1. Error::OnlyCommunityAndCommonCid
        assert_noop!(
            ComingId::transfer(Origin::signed(RESERVE2), 1, RESERVE3),
            Error::<Test>::OnlyCommunityAndCommonCid,
        );

        // 2. Error::UndistributedCid
        assert_noop!(
            ComingId::transfer(Origin::signed(ADMIN), 100001, RESERVE3),
            Error::<Test>::UndistributedCid,
        );

        // 3. Error::RequireOwner
        assert_noop!(
            ComingId::transfer(Origin::signed(ADMIN), 100000, RESERVE3),
            Error::<Test>::RequireOwner,
        );

        assert_ok!(ComingId::transfer(
            Origin::signed(COMMUNITY_ALICE),
            100000,
            COMMUNITY_BOB
        ));
        expect_event(ComingIdEvent::Transferred(
            COMMUNITY_ALICE,
            COMMUNITY_BOB,
            100000,
        ));

        let bond = BondData {
            bond_type: 1u16,
            data: b"test".to_vec(),
        };
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            bond.clone()
        ));

        assert_ok!(ComingId::transfer(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            COMMON_DAVE
        ));
        expect_event(ComingIdEvent::Transferred(
            COMMON_CHARLIE,
            COMMON_DAVE,
            1000000,
        ));

        let cid_details = ComingId::get_bond_data(1000000);
        assert_eq!(
            Some(CidDetails {
                owner: COMMON_DAVE,
                bonds: vec![]
            }),
            cid_details
        );

        // transfer to self
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_DAVE),
            1000000,
            bond.clone()
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMON_DAVE,
                bonds: vec![bond]
            }),
            ComingId::get_bond_data(1000000)
        );
        assert_ok!(ComingId::transfer(
            Origin::signed(COMMON_DAVE),
            1000000,
            COMMON_DAVE
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMON_DAVE,
                bonds: vec![]
            }),
            ComingId::get_bond_data(1000000)
        );
    });
}

#[test]
fn bond_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            100000,
            COMMUNITY_ALICE
        ));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1000000,
            COMMON_CHARLIE
        ));
        expect_event(ComingIdEvent::Registered(COMMON_CHARLIE, 1000000));
        let bond = BondData {
            bond_type: 1u16,
            data: b"test".to_vec(),
        };

        // 1. Error::InvalidCid
        assert_noop!(
            ComingId::bond(Origin::signed(RESERVE2), 0, bond.clone()),
            Error::<Test>::InvalidCid,
        );
        assert_noop!(
            ComingId::bond(Origin::signed(RESERVE2), 1000000000000, bond.clone()),
            Error::<Test>::InvalidCid,
        );

        // 2. Error::RequireOwner
        assert_noop!(
            ComingId::bond(Origin::signed(ADMIN), 1, bond.clone()),
            Error::<Test>::RequireOwner,
        );

        assert_ok!(ComingId::bond(Origin::signed(RESERVE2), 1, bond.clone()));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMUNITY_ALICE),
            100000,
            bond.clone()
        ));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            bond.clone()
        ));

        let new_bond1 = BondData {
            bond_type: 1u16,
            data: b"new-test".to_vec(),
        };
        assert_ok!(ComingId::bond(
            Origin::signed(RESERVE2),
            1,
            new_bond1.clone()
        ));
        expect_event(ComingIdEvent::BondUpdated(RESERVE2, 1, 1u16));
        assert_eq!(
            Some(CidDetails {
                owner: RESERVE2,
                bonds: vec![new_bond1]
            }),
            ComingId::get_bond_data(1)
        );

        let new_bond2 = BondData {
            bond_type: 2u16,
            data: b"new-test".to_vec(),
        };
        assert_ok!(ComingId::bond(
            Origin::signed(COMMUNITY_ALICE),
            100000,
            new_bond2.clone()
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMUNITY_ALICE,
                bonds: vec![bond.clone(), new_bond2]
            }),
            ComingId::get_bond_data(100000)
        );

        let new_bond3 = BondData {
            bond_type: 3u16,
            data: b"new-test".to_vec(),
        };
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            new_bond3.clone()
        ));
        expect_event(ComingIdEvent::Bonded(COMMON_CHARLIE, 1000000, 3u16));
        assert_eq!(
            Some(CidDetails {
                owner: COMMON_CHARLIE,
                bonds: vec![bond, new_bond3]
            }),
            ComingId::get_bond_data(1000000)
        );
    })
}

#[test]
fn unbond_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            100000,
            COMMUNITY_ALICE
        ));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1000000,
            COMMON_CHARLIE
        ));
        expect_event(ComingIdEvent::Registered(COMMON_CHARLIE, 1000000));
        let bond = BondData {
            bond_type: 1u16,
            data: b"test".to_vec(),
        };

        assert_ok!(ComingId::bond(Origin::signed(RESERVE2), 1, bond.clone()));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMUNITY_ALICE),
            100000,
            bond.clone()
        ));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            bond.clone()
        ));

        // 1. Error::InvalidCid
        assert_noop!(
            ComingId::unbond(Origin::signed(RESERVE2), 0, 1u16),
            Error::<Test>::InvalidCid,
        );
        assert_noop!(
            ComingId::unbond(Origin::signed(RESERVE2), 1000000000000, 1u16),
            Error::<Test>::InvalidCid,
        );

        // 2. Error::RequireOwner
        assert_noop!(
            ComingId::unbond(Origin::signed(ADMIN), 1, 1u16),
            Error::<Test>::RequireOwner,
        );

        assert_ok!(ComingId::unbond(Origin::signed(RESERVE2), 1, 1u16));
        expect_event(ComingIdEvent::UnBonded(RESERVE2, 1, 1u16));

        let new_bond2 = BondData {
            bond_type: 2u16,
            data: b"new-test".to_vec(),
        };
        assert_ok!(ComingId::bond(
            Origin::signed(COMMUNITY_ALICE),
            100000,
            new_bond2.clone()
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMUNITY_ALICE,
                bonds: vec![bond.clone(), new_bond2.clone()]
            }),
            ComingId::get_bond_data(100000)
        );
        assert_ok!(ComingId::unbond(
            Origin::signed(COMMUNITY_ALICE),
            100000,
            1u16
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMUNITY_ALICE,
                bonds: vec![new_bond2]
            }),
            ComingId::get_bond_data(100000)
        );

        // unbond twice
        // 3. Error::NotFoundBondType
        assert_ok!(ComingId::unbond(
            Origin::signed(COMMON_CHARLIE),
            1000000,
            1u16
        ));
        expect_event(ComingIdEvent::UnBonded(COMMON_CHARLIE, 1000000, 1u16));
        assert_noop!(
            ComingId::unbond(Origin::signed(COMMON_CHARLIE), 1000000, 1u16),
            Error::<Test>::NotFoundBondType,
        );
    })
}
