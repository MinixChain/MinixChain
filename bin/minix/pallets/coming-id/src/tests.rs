use crate::{Error, mock::*, BondData};
use frame_support::{assert_ok, assert_noop};
use super::Event as ComingIdEvent;

const ADMIN: u64 = 1;
const RESERVE2: u64 = 2;
const RESERVE3: u64 = 3;
const COMMUNITY_ALICE: u64 = 100000;
const COMMUNITY_BOB:   u64 = 999999;
const COMMON_CHARLIE:  u64 = 1000000;
const COMMON_DAVE:     u64 = 999999999999;

#[test]
fn it_works_for_regular_value() {
	new_test_ext(ADMIN).execute_with(|| {
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
		assert_ok!(ComingId::claim(Origin::signed(COMMON_CHARLIE), COMMON_CHARLIE));
		assert_ok!(ComingId::approve(Origin::signed(ADMIN), 1000000, 1000001));
		assert_ok!(ComingId::claim(Origin::signed(COMMON_DAVE), COMMON_DAVE));
		assert_ok!(ComingId::disapprove(Origin::signed(ADMIN), 1000001, 1000001+5));
		assert_ok!(ComingId::bond(Origin::signed(COMMON_CHARLIE), 1000000, BondData{bond_type:1u16, data:vec![]}));
		assert_ok!(ComingId::unbond(Origin::signed(COMMON_CHARLIE), 1000000, 1u16));
		assert_ok!(ComingId::transfer(Origin::signed(COMMON_CHARLIE), 1000000, COMMON_DAVE));
		expect_event(ComingIdEvent::Transferred(COMMON_CHARLIE, COMMON_DAVE, 1000000));
	});
}

#[test]
fn register_should_work() {
	new_test_ext(ADMIN).execute_with(|| {
		// (1) Error::<T>::RequireAdmin
		assert_noop!(
			ComingId::register(Origin::signed(COMMUNITY_ALICE), 1, RESERVE2),
			Error::<Test>::RequireAdmin
		);

		// (2) Error::<T>::OnlyReservedAndCommunityCid
		assert_noop!(
			ComingId::register(Origin::signed(ADMIN), 1000000, RESERVE2),
			Error::<Test>::OnlyReservedAndCommunityCid
		);

		// (3) Event::Registered
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
		expect_event(ComingIdEvent::Registered(RESERVE2, 1));

		// (4) Event::ForceTransferred
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE3));
		expect_event(ComingIdEvent::ForceTransferred(RESERVE3, 1));

		// (5) Error::<T>::Error::<T>::DistributedCid
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 100000, COMMUNITY_ALICE));
		assert_noop!(
			ComingId::register(Origin::signed(ADMIN), 1000000, COMMUNITY_BOB),
			Error::<Test>::DistributedCid
		);
	});
}

#[test]
fn claim_should_work_by_valid_cid() {

}

#[test]
fn approve_should_work_by_valid_cid() {

}

#[test]
fn disapprove_should_work_by_valid_cid() {

}

#[test]
fn bond_should_work_by_valid_cid() {

}

#[test]
fn unbond_should_work_by_valid_cid() {

}
