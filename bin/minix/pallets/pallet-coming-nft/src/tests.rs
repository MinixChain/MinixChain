use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

const ADMIN: u64 = 1;
const RESERVE2: u64 = 2;
const RESERVE3: u64 = 3;

#[test]
fn mint_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        // (1) register
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1));

        // (2) mint card failed
        assert_noop!(
            ComingNFT::mint(Origin::signed(ADMIN), 2, vec![]),
            Error::<Test>::UndistributedCid,
        );

        // (3) mint card success
        let card = br#"{"name": "testCard"}"#.to_vec();
        assert_ok!(ComingNFT::mint(Origin::signed(ADMIN), 1, card.clone()));
        expect_event(ComingIdEvent::MintCard(1, card.clone()));

        assert_noop!(
            ComingNFT::mint(Origin::signed(ADMIN), 1, card.clone()),
            Error::<Test>::BanMint
        );
    });
}

#[test]
fn burn_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        // (1) register
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1));

        assert_eq!(ComingNFT::cids_of_owner(RESERVE2), vec![1]);
        assert_eq!(ComingNFT::owner_of_cid(1), Some(RESERVE2));
        assert_eq!(ComingNFT::card_of_cid(1), None);

        // (2) burn
        assert_ok!(ComingNFT::burn(Origin::signed(ADMIN), 1));
        expect_event(ComingIdEvent::Burned(1));

        assert!(ComingNFT::cids_of_owner(RESERVE2).is_empty());
        assert_eq!(ComingNFT::owner_of_cid(1), None);
        assert_eq!(ComingNFT::card_of_cid(1), None);

        assert_noop!(
            ComingNFT::burn(Origin::signed(ADMIN), 1),
            Error::<Test>::UndistributedCid,
        );
    })
}

#[test]
fn burn_should_not_work() {
    new_test_ext(ADMIN).execute_with(|| {
        // (1) register
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 100_000, RESERVE2));
        expect_event(ComingIdEvent::Registered(RESERVE2, 100_000));

        assert_eq!(ComingNFT::cids_of_owner(RESERVE2), vec![100_000]);
        assert_eq!(ComingNFT::owner_of_cid(100_000), Some(RESERVE2));
        assert_eq!(ComingNFT::card_of_cid(100_000), None);

        // (2) burn
        assert_noop!(
            ComingNFT::burn(Origin::signed(ADMIN), 100_000),
            Error::<Test>::BanBurn,
        );

        assert_noop!(
            ComingNFT::burn(Origin::signed(RESERVE2), 1),
            Error::<Test>::RequireHighAuthority,
        );

        assert_noop!(
            ComingNFT::burn(Origin::signed(ADMIN), 1),
            Error::<Test>::UndistributedCid,
        );
    })
}

#[test]
fn transfer_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        // (1) register
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_000,
            RESERVE2
        ));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1_000_000));

        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_001,
            RESERVE3
        ));
        expect_event(ComingIdEvent::Registered(RESERVE3, 1_000_001));

        // (2) transfer card failed
        assert_noop!(
            ComingNFT::transfer(Origin::signed(RESERVE2), 1_000_001, RESERVE3),
            Error::<Test>::RequireOwner,
        );

        assert_noop!(
            ComingNFT::transfer(Origin::signed(RESERVE2), 1_000_002, RESERVE3),
            Error::<Test>::UndistributedCid,
        );

        // (3) transfer ok without card
        assert_ok!(ComingNFT::transfer(
            Origin::signed(RESERVE2),
            1_000_000,
            RESERVE3
        ));

        // (4) mint card failed
        assert_noop!(
            ComingNFT::mint(Origin::signed(ADMIN), 2, vec![]),
            Error::<Test>::UndistributedCid,
        );

        assert_noop!(
            ComingNFT::mint(
                Origin::signed(ADMIN),
                1_000_000,
                vec![1; 1048576 + 1 as usize]
            ),
            Error::<Test>::TooBigCardSize,
        );

        // (5) mint card success
        let card = br#"{"name": "testCard"}"#.to_vec();
        assert_ok!(ComingNFT::mint(
            Origin::signed(ADMIN),
            1_000_000,
            card.clone()
        ));
        expect_event(ComingIdEvent::MintCard(1_000_000, card.clone()));

        // (6) transfer ok with card
        assert_ok!(ComingNFT::transfer(
            Origin::signed(RESERVE3),
            1_000_000,
            RESERVE2
        ));

        assert_eq!(ComingNFT::cids_of_owner(RESERVE2), vec![1_000_000]);
        assert_eq!(ComingNFT::owner_of_cid(1_000_000), Some(RESERVE2));
        assert_eq!(ComingNFT::card_of_cid(1_000_000), Some(card.into()));

        assert_eq!(ComingNFT::cids_of_owner(RESERVE3), vec![1_000_001]);
    });
}

#[test]
fn transfer_to_self_should_do_nothing() {
    new_test_ext(ADMIN).execute_with(|| {
        // (1) register
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            1_000_000,
            RESERVE2
        ));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1_000_000));

        // (2) mint
        let card = br#"{"name": "testCard"}"#.to_vec();
        assert_ok!(ComingNFT::mint(
            Origin::signed(ADMIN),
            1_000_000,
            card.clone()
        ));
        expect_event(ComingIdEvent::MintCard(1_000_000, card.clone()));

        // (3) bond
        let bond = BondData {
            bond_type: 1u16,
            data: b"testbond".to_vec().into(),
        };

        assert_ok!(ComingId::bond(Origin::signed(RESERVE2), 1_000_000, bond.clone()));
        expect_event(ComingIdEvent::Bonded(RESERVE2, 1_000_000, 1u16));

        // (3) transfer to self
        assert_ok!(ComingNFT::transfer(
            Origin::signed(RESERVE2),
            1_000_000,
            RESERVE2
        ));
        expect_event(ComingIdEvent::Transferred(RESERVE2, RESERVE2, 1_000_000));

        assert_eq!(ComingNFT::cids_of_owner(RESERVE2), vec![1_000_000]);
        assert_eq!(ComingNFT::owner_of_cid(1_000_000), Some(RESERVE2));
        assert_eq!(ComingNFT::card_of_cid(1_000_000), Some(card.clone().into()));

        assert_eq!(
            Some(CidDetails {
                owner: RESERVE2,
                bonds: vec![bond],
                card: card.into()
            }),
            ComingId::get_bond_data(1_000_000)
        );
    });
}
