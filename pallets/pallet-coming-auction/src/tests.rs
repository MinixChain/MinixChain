use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

const ALICE: u128 = 1;
const BOB: u128 = 2;
const CHARLIE: u128 = 3;
const DAVE: u128 = 4;
const RESERVE: Cid = 0;
const COMMUNITY: Cid = 100_000;
const COMMON: Cid = 1_000_000;
const DURATION: u64 = MIN_DURATION as u64;

#[test]
fn remint_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        // (1) register cid
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));

        // (2) remint card failed
        assert_noop!(
            ComingAuction::remint(Origin::signed(BOB), COMMON, vec![], 2),
            ComingIdError::<Test>::RequireOwner,
        );

        // (3) remint card success
        let card = br#"{"name": "testCard1"}"#.to_vec();
        assert_ok!(ComingAuction::remint(
            Origin::signed(ALICE),
            COMMON,
            card,
            2
        ));
    });
}

#[test]
fn create_auction_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        // (1) register cid
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));

        // (2) create an auction
        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));

        expect_event(AuctionEvent::AuctionCreated(
            COMMON,
            ALICE,
            10_000_000_000,
            1_000_000_000,
            DURATION,
            1,
        ));

        if let Some(Auction {
            seller,
            start_price,
            end_price,
            duration,
            start,
        }) = ComingAuction::get_auction(COMMON)
        {
            assert_eq!(seller, ALICE);
            assert_eq!(start_price, 10_000_000_000);
            assert_eq!(end_price, 1_000_000_000);
            assert_eq!(duration, DURATION);
            assert_eq!(start, 1);
        }

        assert_eq!(ComingAuction::get_stats(), (1, 0, 0));
    });
}

#[test]
fn create_auction_should_not_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), RESERVE, ALICE));
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMUNITY, BOB));

        // 1. ComingIdError::BanTransfer
        assert_noop!(
            ComingAuction::create(
                Origin::signed(ALICE),
                RESERVE,
                10_000_000_000,
                1_000_000_000,
                DURATION
            ),
            ComingIdError::<Test>::BanTransfer
        );

        // 2. ComingIdError::RequireOwner
        assert_noop!(
            ComingAuction::create(
                Origin::signed(ALICE),
                COMMUNITY,
                10_000_000_000,
                1_000_000_000,
                DURATION
            ),
            ComingIdError::<Test>::RequireOwner
        );

        // 3. TooLittleDuration
        assert_noop!(
            ComingAuction::create(
                Origin::signed(BOB),
                COMMUNITY,
                10_000_000_000,
                1_000_000_000,
                0
            ),
            Error::<Test>::TooLittleDuration
        );

        // 4. LessThanMinBalance
        assert_noop!(
            ComingAuction::create(
                Origin::signed(BOB),
                COMMUNITY,
                ExistentialDeposit::get(),
                ExistentialDeposit::get(),
                DURATION
            ),
            Error::<Test>::LessThanMinBalance
        );

        // 5. OnAuction
        assert_ok!(ComingAuction::create(
            Origin::signed(BOB),
            COMMUNITY,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));

        assert_noop!(
            ComingAuction::create(
                Origin::signed(BOB),
                COMMUNITY,
                10_000_000_000,
                1_000_000_000,
                DURATION
            ),
            Error::<Test>::OnAuction
        );
    })
}

#[test]
fn bid_auction_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        // (1) register cid
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));
        assert_eq!(
            Some(ALICE),
            <Test as Config>::ComingNFT::owner_of_cid(COMMON)
        );

        // (2) create an auction
        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));
        let auction_account = ComingAuction::auction_account_id(COMMON);
        assert_eq!(
            Some(auction_account),
            <Test as Config>::ComingNFT::owner_of_cid(COMMON)
        );

        expect_event(AuctionEvent::AuctionCreated(
            COMMON,
            ALICE,
            10_000_000_000,
            1_000_000_000,
            DURATION,
            1,
        ));

        assert_eq!(ComingAuction::get_stats(), (1, 0, 0));

        run_to_block(DURATION / 2 + 1);

        assert_eq!(ComingAuction::get_current_price(COMMON), 5_500_000_000);
        assert_eq!(ComingAuction::balance_of(&ALICE), 10_000_000_000);

        // (3) bid an auction
        assert_ok!(ComingAuction::bid(
            Origin::signed(BOB),
            COMMON,
            5_500_000_000,
        ));

        expect_event(AuctionEvent::AuctionSuccessful(
            COMMON,
            BOB,
            5_500_000_000,
            DURATION / 2 + 1,
        ));

        assert_eq!(
            ComingAuction::balance_of(&ALICE),
            10_000_000_000 + 5_500_000_000
        );
        assert_eq!(
            ComingAuction::balance_of(&BOB),
            10_000_000_000 - 5_500_000_000
        );
        assert_eq!(Some(BOB), <Test as Config>::ComingNFT::owner_of_cid(COMMON));

        assert_eq!(ComingAuction::get_stats(), (1, 1, 0));
    });
}
#[test]
fn remint_bid_auction_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        // (1) register cid
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));
        assert_ok!(ComingAuction::remint(
            Origin::signed(ALICE),
            COMMON,
            vec![],
            30
        ));
        assert_ok!(ComingId::transfer(&ALICE, COMMON, &BOB));
        assert_eq!(Some(BOB), <Test as Config>::ComingNFT::owner_of_cid(COMMON));

        // (2) create an auction
        assert_ok!(ComingAuction::create(
            Origin::signed(BOB),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));
        let auction_account = ComingAuction::auction_account_id(COMMON);
        assert_eq!(
            Some(auction_account),
            <Test as Config>::ComingNFT::owner_of_cid(COMMON)
        );

        assert_eq!(ComingAuction::get_stats(), (1, 0, 0));

        run_to_block(DURATION / 2 + 1);

        assert_eq!(ComingAuction::get_current_price(COMMON), 5_500_000_000);
        assert_eq!(ComingAuction::balance_of(&DAVE), 10_000_000_000);

        // (3) bid an auction
        assert_ok!(ComingAuction::bid(
            Origin::signed(DAVE),
            COMMON,
            5_500_000_000,
        ));

        expect_event(AuctionEvent::AuctionSuccessful(
            COMMON,
            DAVE,
            5_500_000_000,
            DURATION / 2 + 1,
        ));

        let tax_fee = ComingAuction::calculate_fee(5_500_000_000, 30);
        assert_eq!(5_500_000_000 / 100 * 30, tax_fee);
        assert_eq!(ComingAuction::balance_of(&ALICE), 10_000_000_000 + tax_fee);
        assert_eq!(
            ComingAuction::balance_of(&BOB),
            10_000_000_000 + 5_500_000_000 - tax_fee
        );
        assert_eq!(
            Some(DAVE),
            <Test as Config>::ComingNFT::owner_of_cid(COMMON)
        );
        assert_eq!(ComingAuction::get_stats(), (1, 1, 0));
    });
}

#[test]
fn bid_auction_should_not_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));

        // 1. NotOnAuction
        assert_noop!(
            ComingAuction::bid(Origin::signed(CHARLIE), COMMON, 10_000_000_000,),
            Error::<Test>::NotOnAuction
        );

        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));

        // 2. LowBidValue
        assert_noop!(
            ComingAuction::bid(Origin::signed(BOB), COMMON, 1_000_000_000,),
            Error::<Test>::LowBidValue
        );

        // 3. BalancesError::KeepAlive
        assert_noop!(
            ComingAuction::bid(Origin::signed(BOB), COMMON, 10_000_000_000,),
            BalancesError::<Test>::KeepAlive
        );

        run_to_block(2);

        // 4. BalancesError::InsufficientBalance
        assert_noop!(
            ComingAuction::bid(Origin::signed(CHARLIE), COMMON, 10_000_000_000,),
            BalancesError::<Test>::InsufficientBalance
        );
    })
}

#[test]
fn common_cancel_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        // (1) register cid
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));
        assert_eq!(
            Some(ALICE),
            <Test as Config>::ComingNFT::owner_of_cid(COMMON)
        );

        // (2) create an auction
        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));
        let auction_account = ComingAuction::auction_account_id(COMMON);
        assert_eq!(
            Some(auction_account),
            <Test as Config>::ComingNFT::owner_of_cid(COMMON)
        );

        expect_event(AuctionEvent::AuctionCreated(
            COMMON,
            ALICE,
            10_000_000_000,
            1_000_000_000,
            DURATION,
            1,
        ));

        assert_eq!(ComingAuction::get_stats(), (1, 0, 0));

        run_to_block(DURATION / 2 + 1);

        assert_eq!(ComingAuction::get_current_price(COMMON), 5_500_000_000);
        assert_eq!(ComingAuction::balance_of(&ALICE), 10_000_000_000);

        // (3) cancel an auction
        assert_ok!(ComingAuction::cancel(Origin::signed(ALICE), COMMON));

        expect_event(AuctionEvent::AuctionCanceled(COMMON, DURATION / 2 + 1));

        assert_eq!(ComingAuction::balance_of(&ALICE), 10_000_000_000);
        assert_eq!(
            Some(ALICE),
            <Test as Config>::ComingNFT::owner_of_cid(COMMON)
        );

        assert_eq!(ComingAuction::get_stats(), (1, 0, 1));
    });
}

#[test]
fn pause_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));

        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::Paused(1));

        run_to_block(2);

        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::Paused(1));

        // 1. create
        assert_noop!(
            ComingAuction::create(
                Origin::signed(ALICE),
                COMMON,
                10_000_000_000,
                1_000_000_000,
                DURATION
            ),
            Error::<Test>::InEmergency
        );

        // 2. bid
        assert_noop!(
            ComingAuction::bid(Origin::signed(BOB), COMMON, 10_000_000_000),
            Error::<Test>::InEmergency
        );

        // 3. set_fee_point
        assert_noop!(
            ComingAuction::set_fee_point(Origin::signed(ALICE), 10u8),
            Error::<Test>::InEmergency
        );
    })
}

#[test]
fn unpause_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMUNITY, ALICE));

        // 1. create
        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::Paused(1));
        run_to_block(2);
        assert_noop!(
            ComingAuction::create(
                Origin::signed(ALICE),
                COMMON,
                10_000_000_000,
                1_000_000_000,
                DURATION
            ),
            Error::<Test>::InEmergency
        );
        assert_ok!(ComingAuction::unpause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::UnPaused(2));
        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMUNITY,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));

        // 2. bid
        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::Paused(2));
        run_to_block(3);
        assert_noop!(
            ComingAuction::bid(Origin::signed(BOB), COMMUNITY, 10_000_000_000),
            Error::<Test>::InEmergency
        );
        assert_ok!(ComingAuction::unpause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::UnPaused(3));
        assert_ok!(ComingAuction::bid(
            Origin::signed(BOB),
            COMMUNITY,
            9_990_000_000
        ));

        // 3. set_fee_point
        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::Paused(3));
        run_to_block(4);
        assert_noop!(
            ComingAuction::set_fee_point(Origin::signed(ALICE), 10u8),
            Error::<Test>::InEmergency
        );
        assert_ok!(ComingAuction::unpause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::UnPaused(4));
        assert_ok!(ComingAuction::set_fee_point(Origin::signed(ALICE), 10u8));
    })
}

#[test]
fn set_fee_point_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_eq!(0u8, ComingAuction::get_fee_point());
        assert_ok!(ComingAuction::set_fee_point(Origin::signed(ALICE), 255u8));
        assert_eq!(255u8, ComingAuction::get_fee_point());

        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        assert_noop!(
            ComingAuction::set_fee_point(Origin::signed(ALICE), 10u8),
            Error::<Test>::InEmergency
        );

        assert_ok!(ComingAuction::unpause(Origin::signed(ALICE)));
        assert_ok!(ComingAuction::set_fee_point(Origin::signed(ALICE), 10u8));

        assert_eq!(10u8, ComingAuction::get_fee_point());
    })
}

#[test]
fn admin_cancel_when_pause_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, BOB));
        assert_ok!(ComingAuction::create(
            Origin::signed(BOB),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));
        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::Paused(1));

        run_to_block(2);

        assert_ok!(ComingAuction::cancel_when_pause(
            Origin::signed(ALICE),
            COMMON
        ));
        assert_eq!(ComingAuction::get_stats(), (1, 0, 1));
    })
}

#[test]
fn admin_cancel_should_not_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, BOB));
        assert_ok!(ComingAuction::create(
            Origin::signed(BOB),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));

        run_to_block(2);

        assert_noop!(
            ComingAuction::cancel_when_pause(Origin::signed(ALICE), COMMON),
            Error::<Test>::OnlyInEmergency
        );
        assert_eq!(ComingAuction::get_stats(), (1, 0, 0));
    })
}

#[test]
fn common_cancel_when_pause_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, BOB));
        assert_ok!(ComingAuction::create(
            Origin::signed(BOB),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));
        assert_ok!(ComingAuction::pause(Origin::signed(ALICE)));
        expect_event(AuctionEvent::Paused(1));

        run_to_block(2);

        assert_ok!(ComingAuction::cancel(Origin::signed(BOB), COMMON));
        assert_eq!(ComingAuction::get_stats(), (1, 0, 1));
    })
}

#[test]
fn english_auction_should_work() {
    new_test_ext(ALICE).execute_with(|| {
        // (1) register cid
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));

        // (2) create an auction
        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMON,
            1_000_000_000,
            10_000_000_000,
            DURATION
        ));

        expect_event(AuctionEvent::AuctionCreated(
            COMMON,
            ALICE,
            1_000_000_000,
            10_000_000_000,
            DURATION,
            1,
        ));

        if let Some(Auction {
            seller,
            start_price,
            end_price,
            duration,
            start,
        }) = ComingAuction::get_auction(COMMON)
        {
            assert_eq!(seller, ALICE);
            assert_eq!(start_price, 1_000_000_000);
            assert_eq!(end_price, 10_000_000_000);
            assert_eq!(duration, DURATION);
            assert_eq!(start, 1);
        }

        assert_eq!(ComingAuction::get_stats(), (1, 0, 0));

        run_to_block(DURATION / 2 + 1);

        assert_ok!(ComingAuction::bid(
            Origin::signed(BOB),
            COMMON,
            5_500_000_000,
        ));

        expect_event(AuctionEvent::AuctionSuccessful(
            COMMON,
            BOB,
            5_500_000_000,
            DURATION / 2 + 1,
        ));

        assert_eq!(
            ComingAuction::balance_of(&ALICE),
            10_000_000_000 + 5_500_000_000
        );
        assert_eq!(
            ComingAuction::balance_of(&BOB),
            10_000_000_000 - 5_500_000_000
        );
        assert_eq!(Some(BOB), <Test as Config>::ComingNFT::owner_of_cid(COMMON));

        assert_eq!(ComingAuction::get_stats(), (1, 1, 0));
    });
}

#[test]
fn english_auction_current_price() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));
        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMON,
            1_000_000_000,
            10_000_000_000,
            DURATION
        ));

        run_to_block(DURATION / 4 + 1);
        assert_eq!(3_250_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 2 + 1);
        assert_eq!(5_500_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 3 + 1);
        assert_eq!(7_750_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 4 + 1);
        assert_eq!(10_000_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 4 + 2);
        assert_eq!(10_000_000_000, ComingAuction::get_current_price(COMMON));
    })
}

#[test]
fn dutch_auction_current_price() {
    new_test_ext(ALICE).execute_with(|| {
        assert_ok!(ComingId::register(Origin::signed(ALICE), COMMON, ALICE));
        assert_ok!(ComingAuction::create(
            Origin::signed(ALICE),
            COMMON,
            10_000_000_000,
            1_000_000_000,
            DURATION
        ));

        run_to_block(DURATION / 4 + 1);
        assert_eq!(7_750_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 2 + 1);
        assert_eq!(5_500_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 3 + 1);
        assert_eq!(3_250_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 4 + 1);
        assert_eq!(1_000_000_000, ComingAuction::get_current_price(COMMON));

        run_to_block(DURATION / 4 * 4 + 2);
        assert_eq!(1_000_000_000, ComingAuction::get_current_price(COMMON));
    })
}

#[test]
fn calculate_remint_fee() {
    new_test_ext(ALICE).execute_with(|| {
        let min = ComingAuction::calculate_remint_fee(0);
        let max = ComingAuction::calculate_remint_fee(32);

        for remint in 0..32u8 {
            let multiple = 2u32.checked_pow(remint as u32).unwrap_or(4294967295u32);

            assert_eq!(
                min.saturating_mul(multiple as u128),
                ComingAuction::calculate_remint_fee(remint)
            )
        }

        for remint in 32..64u8 {
            assert_eq!(max, ComingAuction::calculate_remint_fee(remint))
        }
    })
}
