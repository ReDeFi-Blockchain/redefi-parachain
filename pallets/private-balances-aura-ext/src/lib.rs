#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;

use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade};
use frame_system::{pallet_prelude::*, RawOrigin};
pub use pallet::*;
use sp_runtime::{
	traits::{One, Zero},
	BoundedSlice,
};

pub mod migration;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_aura::Config {
		/// Who can call `setTrustedAuthorities` extrinsic.
		type TrustedCollatorsOrigin: EnsureOrigin<Self::RuntimeOrigin>;

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
		pub(crate) fn initialize_authorities(authorities: &[T::AuthorityId]) {
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

		pub(crate) fn initialize_trusted_authorities(authorities: &[T::AuthorityId]) {
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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10)]
		pub fn set_trusted_authorities(
			origin: T::RuntimeOrigin,
			authorities: BoundedVec<T::AuthorityId, T::MaxAuthorities>,
		) -> DispatchResult {
			T::TrustedCollatorsOrigin::ensure_origin(origin)?;

			<TrustedAuthorities<T>>::put(authorities);

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(current: BlockNumberFor<T>) {
			let next = current + BlockNumberFor::<T>::one();
			let zero = BlockNumberFor::<T>::zero();

			if next % T::TrustedCollatorsPeriod::get() == zero {
				// If TrustedAuthorities is empty, change_authorities does nothing.
				pallet_aura::Pallet::<T>::change_authorities(<TrustedAuthorities<T>>::get());
			} else {
				pallet_aura::Pallet::<T>::change_authorities(<Authorities<T>>::get());
			}
		}
	}
}
