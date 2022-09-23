use crate::mock::*;

use frame_support::assert_ok;
use sp_std::collections::btree_set::BTreeSet;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(ClubMember::add_club(Origin::root(), "rust".as_bytes().to_vec()));
		// Read pallet storage and assert an expected result.
		assert_eq!(ClubMember::clubstore("rust".as_bytes().to_vec()), BTreeSet::new());
	});
}
