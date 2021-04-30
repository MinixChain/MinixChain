use crate::{Error, mock::*, BondData};
use frame_support::{assert_ok, assert_noop};

const ADMIN: u64 = 1;
const RESERVE: u64 = 2;
const COMMUNITY_ALICE: u64 = 100000;
const COMMUNITY_BOB:   u64 = 999999;
const COMMON_CHARLIE:  u64 = 1000000;
const COMMON_DAVE:     u64 = 999999999999;

#[test]
fn it_works_for_regular_value() {
	new_test_ext(ADMIN).execute_with(|| {
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE));
		assert_ok!(ComingId::claim(Origin::signed(COMMON_CHARLIE), COMMON_CHARLIE));
		assert_ok!(ComingId::approve(Origin::signed(ADMIN), 1000000, 1000001));
		assert_ok!(ComingId::claim(Origin::signed(COMMON_DAVE), COMMON_DAVE));
		assert_ok!(ComingId::disapprove(Origin::signed(ADMIN), 1000001, 1000001+5));
		assert_ok!(ComingId::bond(Origin::signed(COMMON_CHARLIE), 1000000, BondData{bond_type:1u16, data:vec![]}));
		assert_ok!(ComingId::unbond(Origin::signed(COMMON_CHARLIE), 1000000, 1u16));
	});
}

#[test]
fn register_should_work_by_valid_cid() {

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
