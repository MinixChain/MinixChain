use super::Event as ComingReputationEvent;
use crate::{mock::*, Error, ReputationGrade};
use frame_support::{assert_noop, assert_ok};

const ADMIN: u64 = 1;
const BOB: u64 = 2;
const COMMUNITY_CID: u64 = 100_000;

const TEST_GRADE1: ReputationGrade = ReputationGrade {
    key1: 100,
    key2: 0,
    key3: 0,
};
const TEST_GRADE2: ReputationGrade = ReputationGrade {
    key1: 99,
    key2: 0,
    key3: 0,
};

#[test]
fn up_grade_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            COMMUNITY_CID,
            BOB
        ));
        assert_ok!(ComingReputation::upgrade(
            Origin::signed(ADMIN),
            COMMUNITY_CID,
            TEST_GRADE1
        ));
        match ComingReputation::get_grade(COMMUNITY_CID) {
            Some(grade) => assert_eq!(100, grade.key1),
            None => assert_eq!(true, false),
        };
        expect_event(ComingReputationEvent::UpReputationGrade(
            COMMUNITY_CID,
            TEST_GRADE1,
        ));
    });
}
#[test]
fn up_grade_should_not_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_noop!(
            ComingReputation::upgrade(Origin::signed(ADMIN), COMMUNITY_CID, TEST_GRADE1),
            Error::<Test>::UndistributedCid
        );

        assert_ok!(ComingId::register(
            Origin::signed(ADMIN),
            COMMUNITY_CID,
            BOB
        ));
        assert_ok!(ComingReputation::upgrade(
            Origin::signed(ADMIN),
            COMMUNITY_CID,
            TEST_GRADE1
        ));

        assert_noop!(
            ComingReputation::upgrade(Origin::signed(ADMIN), COMMUNITY_CID, TEST_GRADE2),
            Error::<Test>::CannotDowngrade
        );
    });
}
