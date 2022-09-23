#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn clubstore)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type ClubStore<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, BTreeSet<T::AccountId>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Member added to the club successfully
		MemberAdded(Vec<u8>, T::AccountId),
		/// Member removed from the club successfully
		MemberRemoved(Vec<u8>, T::AccountId),
		/// Member added to the club successfully
		ClubAdded(Vec<u8>),
		/// Member removed from the club successfully
		ClubRemoved(Vec<u8>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// There is no such club
		InvalidClub,
		ClubAlreadyExists,
	}

	// Our pallet's genesis configuration.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub clubs: Vec<Vec<u8>>,
		pub other: PhantomData<T>,
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { clubs: vec![], other: PhantomData }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the dna and gender to be
			// supplied.
			for club in &self.clubs {
				<ClubStore<T>>::insert(club, BTreeSet::<T::AccountId>::new());
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Adds a new club.
		///
		/// If the club already exists returns Error
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_club(origin: OriginFor<T>, club: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(!<ClubStore<T>>::contains_key(club.clone()), Error::<T>::ClubAlreadyExists);

			// Update storage.
			<ClubStore<T>>::insert(club.clone(), BTreeSet::<T::AccountId>::new());
			Self::deposit_event(Event::ClubAdded(club));

			Ok(())
		}

		/// Removes a club.
		///
		/// If the club does not exist returns Error
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_club(origin: OriginFor<T>, club: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;

			ensure!(<ClubStore<T>>::contains_key(club.clone()), Error::<T>::InvalidClub);

			// Update storage.
			<ClubStore<T>>::remove(club.clone());
			Self::deposit_event(Event::ClubRemoved(club));

			Ok(())
		}

		/// Adds a member to the club.
		///
		/// If the club does not exist returns Error
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_member(
			origin: OriginFor<T>,
			club: Vec<u8>,
			member: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Update storage.
			ensure!(<ClubStore<T>>::contains_key(club.clone()), Error::<T>::InvalidClub);

			<ClubStore<T>>::mutate(club.clone(), |members| members.insert(member.clone()));
			Self::deposit_event(Event::MemberAdded(club, member));

			Ok(())
		}

		/// Removes a member from the club.
		///
		/// If the club does not exist returns Error
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn remove_member(
			origin: OriginFor<T>,
			club: Vec<u8>,
			member: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			// Update storage.
			ensure!(<ClubStore<T>>::contains_key(club.clone()), Error::<T>::InvalidClub);

			<ClubStore<T>>::mutate(club.clone(), |members| {
				members.retain(|m| {
					let diff = *m != member;
					if !diff {
						Self::deposit_event(Event::MemberRemoved(club.clone(), member.clone()));
					}
					diff
				})
			});

			Ok(())
		}
	}
}
