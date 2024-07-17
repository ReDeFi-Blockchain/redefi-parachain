#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

use frame_support::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::BoundedSlice;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_aura::Config {}

	#[pallet::storage]
	pub type Authorities<T: Config> =
		StorageValue<_, BoundedVec<T::AuthorityId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::storage]
	pub type TrustedAuthorities<T: Config> =
		StorageValue<_, BoundedVec<T::AuthorityId, T::MaxAuthorities>, ValueQuery>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub trusted_authorities: Vec<T::AuthorityId>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_authorities(&pallet_aura::Pallet::<T>::authorities());
			Pallet::<T>::initialize_trusted_authorities(&self.trusted_authorities);
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		/// Initial authorities.
		///
		/// The storage will be applied immediately.
		///
		/// The authorities length must be equal or less than T::MaxAuthorities.
		fn initialize_authorities(authorities: &[T::AuthorityId]) {
			if !authorities.is_empty() {
				assert!(
					<Authorities<T>>::get().is_empty(),
					"Authorities are already initialized!"
				);

				let bounded = <BoundedSlice<'_, _, T::MaxAuthorities>>::try_from(authorities)
					.expect("Initial authority set must be less than T::MaxAuthorities");

				<Authorities<T>>::put(bounded);
			}
		}

		/// Initial trusted authorities.
		///
		/// The storage will be applied immediately.
		///
		/// The authorities length must be equal or less than T::MaxAuthorities.
		fn initialize_trusted_authorities(authorities: &[T::AuthorityId]) {
			if !authorities.is_empty() {
				assert!(
					<TrustedAuthorities<T>>::get().is_empty(),
					"Trusted authorities are already initialized!"
				);

				let bounded = <BoundedSlice<'_, _, T::MaxAuthorities>>::try_from(authorities)
					.expect("Initial trusted authority set must be less than T::MaxAuthorities");

				<TrustedAuthorities<T>>::put(bounded);
			}
		}
	}
}
