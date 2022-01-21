use super::Event as ComingIdEvent;
use crate::{mock::*, AccountIdCids, AdminType, BondData, CidDetails, Distributed, Error};
use frame_support::{assert_noop, assert_ok};

const ADMIN: u64 = 1;
const RESERVE2: u64 = 2;
const RESERVE3: u64 = 3;
const COMMUNITY_ALICE: u64 = 100_000;
const COMMUNITY_BOB: u64 = 999_999;
const COMMON_CHARLIE: u64 = 1_000_000;

#[test]
fn it_works_for_regular_value() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_000,
            COMMON_CHARLIE
        ));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1_000_000,
            BondData {
                bond_type: 1u16,
                data: vec![]
            }
        ));
        assert_ok!(ComingId::unbond(
            Origin::signed(COMMON_CHARLIE),
            1_000_000,
            1u16
        ));

        let events = vec![
            Event::ComingId(ComingIdEvent::Registered(RESERVE2, 1)),
            Event::ComingId(ComingIdEvent::Registered(COMMON_CHARLIE, 1_000_000)),
            Event::ComingId(ComingIdEvent::Bonded(COMMON_CHARLIE, 1_000_000, 1)),
            Event::ComingId(ComingIdEvent::UnBonded(COMMON_CHARLIE, 1_000_000, 1)),
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
            Error::<Test>::RequireHighAuthority
        );

        // (2) Error::InvalidCid
        assert_noop!(
            ComingId::register(Origin::signed(ADMIN), 1_000_000_000_000, RESERVE2),
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
            100_000,
            COMMUNITY_ALICE
        ));
        assert_noop!(
            ComingId::register(Origin::signed(ADMIN), 100_000, COMMUNITY_BOB),
            Error::<Test>::DistributedCid
        );

        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_001,
            RESERVE2
        ));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1_000_001));
    });
}

#[test]
fn bond_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            100_000,
            COMMUNITY_ALICE
        ));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_000,
            COMMON_CHARLIE
        ));
        expect_event(ComingIdEvent::Registered(COMMON_CHARLIE, 1_000_000));
        let bond = BondData {
            bond_type: 1u16,
            data: b"test".to_vec(),
        };

        assert_noop!(
            ComingId::bond(Origin::signed(RESERVE2), 1_000_000_000_000, bond.clone()),
            Error::<Test>::InvalidCid,
        );
        // 1. Error::InvalidCid
        assert_noop!(
            ComingId::bond(Origin::signed(RESERVE2), 1_000_000_000_000, bond.clone()),
            Error::<Test>::InvalidCid,
        );

        // 2. Error::RequireOwner
        assert_noop!(
            ComingId::bond(Origin::signed(RESERVE3), 1, bond.clone()),
            Error::<Test>::RequireOwner,
        );

        assert_ok!(ComingId::bond(Origin::signed(RESERVE2), 1, bond.clone()));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMUNITY_ALICE),
            100_000,
            bond.clone()
        ));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1_000_000,
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
                bonds: vec![new_bond1],
                card: vec![],
            }),
            ComingId::get_bond_data(1)
        );

        let new_bond2 = BondData {
            bond_type: 2u16,
            data: b"new-test".to_vec(),
        };
        assert_ok!(ComingId::bond(
            Origin::signed(COMMUNITY_ALICE),
            100_000,
            new_bond2.clone()
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMUNITY_ALICE,
                bonds: vec![bond.clone(), new_bond2],
                card: vec![],
            }),
            ComingId::get_bond_data(100_000)
        );

        let new_bond3 = BondData {
            bond_type: 3u16,
            data: b"new-test".to_vec(),
        };
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1_000_000,
            new_bond3.clone()
        ));
        expect_event(ComingIdEvent::Bonded(COMMON_CHARLIE, 1_000_000, 3u16));
        assert_eq!(
            Some(CidDetails {
                owner: COMMON_CHARLIE,
                bonds: vec![bond, new_bond3],
                card: vec![],
            }),
            ComingId::get_bond_data(1_000_000)
        );
    })
}

#[test]
fn unbond_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            100_000,
            COMMUNITY_ALICE
        ));
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_000,
            COMMON_CHARLIE
        ));
        expect_event(ComingIdEvent::Registered(COMMON_CHARLIE, 1_000_000));
        let bond = BondData {
            bond_type: 1u16,
            data: b"test".to_vec(),
        };

        assert_ok!(ComingId::bond(Origin::signed(RESERVE2), 1, bond.clone()));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMUNITY_ALICE),
            100_000,
            bond.clone()
        ));
        assert_ok!(ComingId::bond(
            Origin::signed(COMMON_CHARLIE),
            1_000_000,
            bond.clone()
        ));

        // 1. Error::InvalidCid
        assert_noop!(
            ComingId::unbond(Origin::signed(RESERVE2), 1_000_000_000_000, 1u16),
            Error::<Test>::InvalidCid,
        );
        assert_noop!(
            ComingId::unbond(Origin::signed(RESERVE2), 1_000_000_000_000, 1u16),
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
            100_000,
            new_bond2.clone()
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMUNITY_ALICE,
                bonds: vec![bond, new_bond2.clone()],
                card: vec![],
            }),
            ComingId::get_bond_data(100_000)
        );
        assert_ok!(ComingId::unbond(
            Origin::signed(COMMUNITY_ALICE),
            100_000,
            1u16
        ));
        assert_eq!(
            Some(CidDetails {
                owner: COMMUNITY_ALICE,
                bonds: vec![new_bond2],
                card: vec![],
            }),
            ComingId::get_bond_data(100_000)
        );

        // unbond twice
        // 3. Error::NotFoundBondType
        assert_ok!(ComingId::unbond(
            Origin::signed(COMMON_CHARLIE),
            1_000_000,
            1u16
        ));
        expect_event(ComingIdEvent::UnBonded(COMMON_CHARLIE, 1_000_000, 1u16));
        assert_noop!(
            ComingId::unbond(Origin::signed(COMMON_CHARLIE), 1_000_000, 1u16),
            Error::<Test>::NotFoundBondType,
        );
    })
}

fn clear_cids(cids: &[u64]) {
    for cid in cids {
        Distributed::<Test>::remove(cid);
        AccountIdCids::<Test>::remove(cid);
    }
}

#[test]
fn register_more_tests() {
    let (high, medium, medium2, medium3, low) = (1u64, 2u64, 3u64, 4u64, 5u64);
    let admins = [high, medium, medium2, medium3, low];
    let (reserve, community, common7, common8, common9, invalid) = (
        1u64,
        100_000u64,
        1_000_000u64,
        10_000_000u64,
        100_000_000u64,
        1_000_000_000_000u64,
    );

    new_test_ext2(admins, high).execute_with(|| {
        // 1. all admins test
        assert_ok!(ComingId::register(Origin::signed(high), reserve, high));
        expect_event(ComingIdEvent::Registered(high, reserve));
        assert_ok!(ComingId::register(
            Origin::signed(medium3),
            community,
            medium3
        ));
        expect_event(ComingIdEvent::Registered(medium3, community));
        assert_ok!(ComingId::register(
            Origin::signed(medium2),
            common7,
            medium2
        ));
        expect_event(ComingIdEvent::Registered(medium2, common7));
        assert_ok!(ComingId::register(Origin::signed(medium), common8, medium));
        expect_event(ComingIdEvent::Registered(medium, common8));
        assert_ok!(ComingId::register(Origin::signed(low), common9, low));
        expect_event(ComingIdEvent::Registered(low, common9));

        assert_ok!(ComingId::check_admin(&high, reserve));
        assert_ok!(ComingId::check_admin(&medium3, community));
        assert_ok!(ComingId::check_admin(&medium2, common7));
        assert_ok!(ComingId::check_admin(&medium, common8));
        assert_ok!(ComingId::check_admin(&low, common9));

        clear_cids(&[reserve, community, common7, common8, common9]);

        // 2. high admin test
        assert_ok!(ComingId::register(Origin::signed(high), reserve, high));
        expect_event(ComingIdEvent::Registered(high, reserve));
        assert_ok!(ComingId::register(Origin::signed(high), community, medium3));
        expect_event(ComingIdEvent::Registered(medium3, community));
        assert_ok!(ComingId::register(Origin::signed(high), common7, medium2));
        expect_event(ComingIdEvent::Registered(medium2, common7));
        assert_ok!(ComingId::register(Origin::signed(high), common8, medium));
        expect_event(ComingIdEvent::Registered(medium, common8));
        assert_ok!(ComingId::register(Origin::signed(high), common9, low));
        expect_event(ComingIdEvent::Registered(low, common9));

        assert_noop!(
            ComingId::register(Origin::signed(high), invalid, high),
            Error::<Test>::InvalidCid
        );

        assert_ok!(ComingId::check_admin(&high, reserve));
        assert_ok!(ComingId::check_admin(&medium3, community));
        assert_ok!(ComingId::check_admin(&medium2, common7));
        assert_ok!(ComingId::check_admin(&medium, common8));
        assert_ok!(ComingId::check_admin(&low, common9));

        clear_cids(&[reserve, community, common7, common8, common9]);

        // 2. medium3 admin test
        assert_noop!(
            ComingId::register(Origin::signed(medium3), reserve, high),
            Error::<Test>::RequireHighAuthority
        );
        assert_ok!(ComingId::register(
            Origin::signed(medium3),
            community,
            medium3
        ));
        assert_noop!(
            ComingId::register(Origin::signed(medium3), common7, medium2),
            Error::<Test>::RequireMediumAuthority2
        );
        assert_noop!(
            ComingId::register(Origin::signed(medium3), common8, medium),
            Error::<Test>::RequireMediumAuthority
        );
        assert_noop!(
            ComingId::register(Origin::signed(medium3), common9, low),
            Error::<Test>::RequireLowAuthority
        );
        assert_noop!(
            ComingId::register(Origin::signed(medium3), invalid, medium3),
            Error::<Test>::InvalidCid
        );

        assert_ok!(ComingId::check_admin(&medium3, community));

        clear_cids(&[reserve, community, common7, common8, common9]);

        // 3. medium2 admin test
        assert_noop!(
            ComingId::register(Origin::signed(medium2), community, medium3),
            Error::<Test>::RequireMediumAuthority3
        );
        assert_ok!(ComingId::register(
            Origin::signed(medium2),
            common7,
            medium2
        ));
        assert_noop!(
            ComingId::register(Origin::signed(medium2), common8, medium),
            Error::<Test>::RequireMediumAuthority
        );

        assert_ok!(ComingId::check_admin(&medium2, common7));

        clear_cids(&[reserve, community, common7, common8, common9]);

        // 4. medium admin test
        assert_noop!(
            ComingId::register(Origin::signed(medium), community, medium3),
            Error::<Test>::RequireMediumAuthority3
        );
        assert_noop!(
            ComingId::register(Origin::signed(medium), common7, medium2),
            Error::<Test>::RequireMediumAuthority2
        );
        assert_ok!(ComingId::register(Origin::signed(medium), common8, medium));
        assert_noop!(
            ComingId::register(Origin::signed(medium), invalid, medium),
            Error::<Test>::InvalidCid
        );

        assert_ok!(ComingId::check_admin(&medium, common8));

        clear_cids(&[reserve, community, common7, common8, common9]);

        // 5. low admin test
        assert_noop!(
            ComingId::register(Origin::signed(low), high, reserve),
            Error::<Test>::RequireHighAuthority
        );
        assert_noop!(
            ComingId::register(Origin::signed(low), community, medium3),
            Error::<Test>::RequireMediumAuthority3
        );
        assert_noop!(
            ComingId::register(Origin::signed(low), common7, medium2),
            Error::<Test>::RequireMediumAuthority2
        );
        assert_noop!(
            ComingId::register(Origin::signed(low), common8, medium),
            Error::<Test>::RequireMediumAuthority
        );
        assert_ok!(ComingId::register(Origin::signed(low), common9, low));
        assert_noop!(
            ComingId::register(Origin::signed(low), invalid, low),
            Error::<Test>::InvalidCid
        );

        assert_ok!(ComingId::check_admin(&low, common9));
    })
}

#[test]
fn set_admin() {
    let (high, medium, medium2, medium3, low) = (1u64, 2u64, 3u64, 4u64, 5u64);
    let admins = [high, medium, medium2, medium3, low];
    let sudo = 6u64;

    new_test_ext2(admins, sudo).execute_with(|| {
        assert_eq!(high, ComingId::high_admin_key());
        assert_eq!(medium, ComingId::medium_admin_key());
        assert_eq!(medium2, ComingId::medium_admin_key2());
        assert_eq!(medium3, ComingId::medium_admin_key3());
        assert_eq!(low, ComingId::low_admin_key());

        assert_ok!(ComingId::set_admin(Origin::root(), sudo, AdminType::High));
        assert_ok!(ComingId::set_admin(Origin::root(), sudo, AdminType::Medium));
        assert_ok!(ComingId::set_admin(
            Origin::root(),
            sudo,
            AdminType::Medium2
        ));
        assert_ok!(ComingId::set_admin(
            Origin::root(),
            sudo,
            AdminType::Medium3
        ));
        assert_ok!(ComingId::set_admin(Origin::root(), sudo, AdminType::Low));

        assert_eq!(sudo, ComingId::high_admin_key());
        assert_eq!(sudo, ComingId::medium_admin_key());
        assert_eq!(sudo, ComingId::medium_admin_key2());
        assert_eq!(sudo, ComingId::medium_admin_key3());
        assert_eq!(sudo, ComingId::low_admin_key());
    })
}
