use super::Event as CidGradeEvent;
use crate::{mock::*, Error, Grade ,CidGrade as CGrade};
use frame_support::{assert_noop, assert_ok};

const ADMIN: u64 = 1;
const RESERVE2: u64 = 2;
const RESERVE3: u64 = 3;
const COMMUNITY_ALICE: u64 = 100_000;
const COMMUNITY_BOB: u64 = 999_999;
const COMMON_CHARLIE: u64 = 1_000_000;

const test_grade: Grade = Grade {
    key1:1,
    key2:2,
    key3:3,
};

#[test]
fn up_grade_should_work() {
    new_test_ext(ADMIN).execute_with(|| {
        assert_ok!(CidGrade::up_grade(Origin::signed(ADMIN), 10, test_grade));
        assert_eq!(Grade {
            key1:1,
            key2:2,
            key3:3,
        }, CidGrade::get_grade(10));
    });
}