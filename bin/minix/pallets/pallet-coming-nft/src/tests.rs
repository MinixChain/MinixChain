
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

const ADMIN: u64 = 1;
const RESERVE2: u64 = 2;

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
