#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet::*;
use sp_runtime::BoundedSlice;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_aura::Config {
		/// Once every `TrustedCollatorsPeriod` blocks will be
		/// used a vector of trusted collators.
		///
		/// For example. If `TrustedCollatorsPeriod` is equal 2 then
		/// a vector of trusted collators will be used every even block.
		type TrustedCollatorsPeriod: Get<BlockNumberFor<Self>>;
	}

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
		fn initialize_authorities(authorities: &[T::AuthorityId]) {
			if authorities.is_empty() {
				return;
			}

			assert!(
				<Authorities<T>>::get().is_empty(),
				"Authorities are already initialized!"
			);

			let bounded = <BoundedSlice<'_, _, T::MaxAuthorities>>::try_from(authorities)
				.expect("Initial authority set must be less than T::MaxAuthorities");

			<Authorities<T>>::put(bounded);
		}

		fn initialize_trusted_authorities(authorities: &[T::AuthorityId]) {
			if authorities.is_empty() {
				return;
			}

			assert!(
				<TrustedAuthorities<T>>::get().is_empty(),
				"Trusted authorities are already initialized!"
			);

			let bounded = <BoundedSlice<'_, _, T::MaxAuthorities>>::try_from(authorities)
				.expect("Initial trusted authority set must be less than T::MaxAuthorities");

			<TrustedAuthorities<T>>::put(bounded);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			let zero: BlockNumberFor<T> = BlockNumberFor::<T>::default();

			if n != zero && n % T::TrustedCollatorsPeriod::get() == zero {
				pallet_aura::Pallet::<T>::change_authorities(<TrustedAuthorities<T>>::get());
			} else {
				pallet_aura::Pallet::<T>::change_authorities(<Authorities<T>>::get());
			}

			T::DbWeight::get().reads_writes(1, 1)
		}
	}
}
