use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext(1).execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(ComingId::claim(Origin::signed(1), 1));

		let did = ComingId::get_bond(1000000);
		println!("{:?}", did)
	});
}
