
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
        expect_event(ComingIdEvent::MintCard(1, card));
    });
}

#[test]
fn transfer_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        // (1) register
        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1_000_000, RESERVE2));
        expect_event(ComingIdEvent::Registered(RESERVE2, 1_000_000));

        assert_ok!(ComingId::register(Origin::signed(ADMIN), 1_000_001, RESERVE3));
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
        assert_ok!(ComingNFT::transfer(Origin::signed(RESERVE2), 1_000_000, RESERVE3));

        // (4) mint card failed
        assert_noop!(
            ComingNFT::mint(Origin::signed(ADMIN), 2, vec![]),
            Error::<Test>::UndistributedCid,
        );

        // (5) mint card success
        let card = br#"{"name": "testCard"}"#.to_vec();
        assert_ok!(ComingNFT::mint(Origin::signed(ADMIN), 1_000_000, card.clone()));
        expect_event(ComingIdEvent::MintCard(1_000_000, card.clone()));

        // (6) transfer ok with card
        assert_ok!(ComingNFT::transfer(Origin::signed(RESERVE3), 1_000_000, RESERVE2));

        assert_eq!(ComingNFT::cids_of_owner(RESERVE2), vec![1_000_000]);
        assert_eq!(ComingNFT::owner_of_cid(1_000_000), Some(RESERVE2));
        assert_eq!(ComingNFT::card_of_cid(1_000_000), Some(card));
    });
}
