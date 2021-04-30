use crate::{Error, mock::*, BondData, CidDetails};
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

		let events = vec![
			Event::pallet_coming_id(ComingIdEvent::Registered(RESERVE2, 1)),
			Event::pallet_coming_id(ComingIdEvent::Claiming(COMMON_CHARLIE, 1000000, 11)),
			Event::pallet_coming_id(ComingIdEvent::Approved(COMMON_CHARLIE, 1000001)),
			Event::pallet_coming_id(ComingIdEvent::Claiming(COMMON_DAVE, 1000001, 11)),
			Event::pallet_coming_id(ComingIdEvent::DisApproved(1000001, 1000006)),
			Event::pallet_coming_id(ComingIdEvent::Bonded(COMMON_CHARLIE, 1000000, 1)),
			Event::pallet_coming_id(ComingIdEvent::UnBonded(COMMON_CHARLIE, 1000000, 1)),
			Event::pallet_coming_id(ComingIdEvent::Transferred(COMMON_CHARLIE, COMMON_DAVE, 1000000))
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

		// (2) Error::OnlyReservedAndCommunityCid
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

		// (5) Error::DistributedCid
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 100000, COMMUNITY_ALICE));
		assert_noop!(
			ComingId::register(Origin::signed(ADMIN), 100000, COMMUNITY_BOB),
			Error::<Test>::DistributedCid
		);
	});
}

#[test]
fn claim_should_work() {
	new_test_ext(ADMIN).execute_with(||{

	})
}

#[test]
fn approve_should_work() {
	new_test_ext(ADMIN).execute_with(||{

	})
}

#[test]
fn disapprove_should_work() {
	new_test_ext(ADMIN).execute_with(||{

	})
}

#[test]
fn transfer_should_work() {
	new_test_ext(ADMIN).execute_with(||{
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 100000, COMMUNITY_ALICE));
		assert_ok!(ComingId::claim(Origin::signed(ADMIN), COMMON_CHARLIE));
		expect_event(ComingIdEvent::ForceClaimed(COMMON_CHARLIE, 1000000));

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

		assert_ok!(ComingId::transfer(Origin::signed(COMMUNITY_ALICE), 100000, COMMUNITY_BOB));
		expect_event(ComingIdEvent::Transferred(COMMUNITY_ALICE, COMMUNITY_BOB, 100000));

		let bond = BondData{
			bond_type:1u16,
			data:b"test".to_vec(),
		};
		assert_ok!(ComingId::bond(Origin::signed(COMMON_CHARLIE), 1000000, bond));

		assert_ok!(ComingId::transfer(Origin::signed(COMMON_CHARLIE), 1000000, COMMON_DAVE));
		expect_event(ComingIdEvent::Transferred(COMMON_CHARLIE, COMMON_DAVE, 1000000));

		let cid_details = ComingId::get_bond(1000000);
		assert_eq!(Some(CidDetails{owner: COMMON_DAVE, bonds: vec![]}), cid_details);
	});
}

#[test]
fn bond_should_work() {
	new_test_ext(ADMIN).execute_with(||{
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 1, RESERVE2));
		assert_ok!(ComingId::register(Origin::signed(ADMIN), 100000, COMMUNITY_ALICE));
		assert_ok!(ComingId::claim(Origin::signed(ADMIN), COMMON_CHARLIE));
		expect_event(ComingIdEvent::ForceClaimed(COMMON_CHARLIE, 1000000));
		let bond = BondData{
			bond_type:1u16,
			data:b"test".to_vec(),
		};

		assert_ok!(ComingId::bond(Origin::signed(RESERVE2), 1, bond.clone()));
		assert_ok!(ComingId::bond(Origin::signed(COMMUNITY_ALICE), 100000, bond.clone()));
		assert_ok!(ComingId::bond(Origin::signed(COMMON_CHARLIE), 1000000, bond.clone()));

		let new_bond1 = BondData{
			bond_type:1u16,
			data:b"new-test".to_vec(),
		};
		assert_ok!(ComingId::bond(Origin::signed(RESERVE2), 1, new_bond1.clone()));
		assert_eq!(
			Some(CidDetails{ owner: RESERVE2, bonds: vec![new_bond1]}),
			ComingId::get_bond(1)
		);

		let new_bond2 = BondData{
			bond_type:2u16,
			data:b"new-test".to_vec(),
		};
		assert_ok!(ComingId::bond(Origin::signed(COMMUNITY_ALICE), 100000, new_bond2.clone()));
		assert_eq!(
			Some(CidDetails{owner: COMMUNITY_ALICE, bonds: vec![bond.clone(), new_bond2]}),
			ComingId::get_bond(100000)
		);

		let new_bond3 = BondData{
			bond_type:3u16,
			data:b"new-test".to_vec(),
		};
		assert_ok!(ComingId::bond(Origin::signed(COMMON_CHARLIE), 1000000, new_bond3.clone()));
		assert_eq!(
			Some(CidDetails{owner: COMMON_CHARLIE, bonds: vec![bond, new_bond3]}),
			ComingId::get_bond(1000000)
		);
	})
}

#[test]
fn unbond_should_work() {
	new_test_ext(ADMIN).execute_with(||{

	})
}
