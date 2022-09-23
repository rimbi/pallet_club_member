//! Benchmarking setup for pallet-club-member

use super::*;

#[allow(unused)]
use crate::Pallet as ClubMember;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_club {
		let s in 0 .. 100;
		let club = format!("club{}", s).as_bytes().to_vec();
	}: _(RawOrigin::Root, club.clone())
	verify {
		assert!(ClubStore::<T>::contains_key(club));
	}

	remove_club {
		let s in 0 .. 100;
		let club = format!("club{}", s).as_bytes().to_vec();
		let origin = <T as frame_system::Config>::Origin::from(RawOrigin::Root);
		ClubMember::<T>::add_club(origin, club.clone()).expect("Add club should succeed!");
	}: _(RawOrigin::Root, club.clone())
	verify {
		assert!(!ClubStore::<T>::contains_key(club));
	}

	add_member {
		let s in 0 .. 100;
		let member: T::AccountId = whitelisted_caller();
		let club = "club".as_bytes().to_vec();
		let origin = <T as frame_system::Config>::Origin::from(RawOrigin::Root);
		ClubMember::<T>::add_club(origin, club.clone()).expect("Add club should succeed!");
	}: _(RawOrigin::Root, club.clone(), member.clone())
	verify {
		let members = ClubStore::<T>::get(club);
		assert!(members.contains(&member));
	}

	remove_member {
		let s in 0 .. 100;
		let member: T::AccountId = whitelisted_caller();
		let club = "club".as_bytes().to_vec();
		let origin = <T as frame_system::Config>::Origin::from(RawOrigin::Root);
		ClubMember::<T>::add_club(origin.clone(), club.clone()).expect("Add club should succeed!");
		ClubMember::<T>::add_member(origin, club.clone(), member.clone()).expect("Add club should succeed!");
	}: _(RawOrigin::Root, club.clone(), member.clone())
	verify {
		let members = ClubStore::<T>::get(club);
		assert!(!members.contains(&member));
	}
}

impl_benchmark_test_suite!(ClubMember, crate::mock::new_test_ext(), crate::mock::Test);
