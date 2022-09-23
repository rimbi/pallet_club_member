use crate::mock::*;

use frame_support::assert_ok;
use sp_std::collections::btree_set::BTreeSet;

#[test]
fn it_should_add_a_club() {
	new_test_ext().execute_with(|| {
		assert_ok!(ClubMember::add_club(Origin::root(), "rust".as_bytes().to_vec()));
		assert_eq!(ClubMember::clubstore("rust".as_bytes().to_vec()), BTreeSet::new());
	});
}

#[test]
fn it_should_fail_when_adding_a_club_more_than_once() {
	new_test_ext().execute_with(|| {
		let club = "rust".as_bytes().to_vec();
		assert_ok!(ClubMember::add_club(Origin::root(), club.clone()));
		assert!(ClubMember::add_club(Origin::root(), club).is_err());
	});
}

#[test]
fn it_should_remove_a_club() {
	new_test_ext().execute_with(|| {
		let club = "rust".as_bytes().to_vec();
		let member = 256u64;
		assert_ok!(ClubMember::add_club(Origin::root(), club.clone()));
		assert_ok!(ClubMember::add_member(Origin::root(), club.clone(), member));
		assert_ok!(ClubMember::remove_club(Origin::root(), club.clone()));
		assert_eq!(ClubMember::clubstore(club), BTreeSet::new());
	});
}

#[test]
fn it_should_fail_when_removing_invalid_club() {
	new_test_ext().execute_with(|| {
		let club = "rust".as_bytes().to_vec();
		assert!(ClubMember::remove_club(Origin::root(), club.clone()).is_err());
	});
}

#[test]
fn it_should_fail_when_adding_a_member_to_invalid_club() {
	new_test_ext().execute_with(|| {
		let club = "rust".as_bytes().to_vec();
		let member = 256u64;
		assert!(ClubMember::add_member(Origin::root(), club.clone(), member).is_err());
	});
}

#[test]
fn it_should_add_a_member_to_an_existing_club() {
	new_test_ext().execute_with(|| {
		let club = "rust".as_bytes().to_vec();
		let member = 256u64;
		assert_ok!(ClubMember::add_club(Origin::root(), club.clone()));
		assert_ok!(ClubMember::add_member(Origin::root(), club.clone(), member));
		let mut members = BTreeSet::new();
		members.insert(member);
		assert_eq!(ClubMember::clubstore(club), members);
	});
}

#[test]
fn it_should_remove_an_existing_member_from_a_club() {
	new_test_ext().execute_with(|| {
		let club = "rust".as_bytes().to_vec();
		let member = 256u64;
		assert_ok!(ClubMember::add_club(Origin::root(), club.clone()));
		assert_ok!(ClubMember::add_member(Origin::root(), club.clone(), member));
		assert_ok!(ClubMember::remove_member(Origin::root(), club.clone(), member));
		let members = BTreeSet::new();
		assert_eq!(ClubMember::clubstore(club), members);
	});
}
