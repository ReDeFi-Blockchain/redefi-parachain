#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade};
use frame_system::{pallet_prelude::*, RawOrigin};
pub use pallet::*;
use sp_runtime::traits::{One, Zero};
use weights::WeightInfo;

pub mod migration;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_aura::Config {
		/// Who can call `setAuthorities` and `setTrustedAuthorities` extrinsics.
		type AuthoritiesOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Once every `TrustedAuthoritiesPeriod` blocks will be
		/// used a vector of trusted authorities.
		///
		/// For example. If `TrustedAuthoritiesPeriod` is equal 2 then
		/// a vector of trusted authorities will be used every even block.
		type TrustedAuthoritiesPeriod: Get<BlockNumberFor<Self>>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
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
		pub trusted_authorities: BoundedVec<T::AuthorityId, T::MaxAuthorities>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize_authorities(pallet_aura::Pallet::<T>::authorities());
			Pallet::<T>::initialize_trusted_authorities(Default::default());
		}
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		/// Authorities list can not be empty.
		EmptyAuthorities,
	}

	impl<T: Config> Pallet<T> {
		pub(crate) fn initialize_authorities(
			authorities: BoundedVec<T::AuthorityId, T::MaxAuthorities>,
		) {
			assert!(
				<Authorities<T>>::get().is_empty(),
				"Authorities are already initialized!"
			);

			<Authorities<T>>::put(authorities);
		}

		pub(crate) fn initialize_trusted_authorities(
			authorities: BoundedVec<T::AuthorityId, T::MaxAuthorities>,
		) {
			assert!(
				<TrustedAuthorities<T>>::get().is_empty(),
				"Trusted authorities are already initialized!"
			);

			<TrustedAuthorities<T>>::put(authorities);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::set_authorities(authorities.len() as u32))]
		pub fn set_authorities(
			origin: T::RuntimeOrigin,
			authorities: BoundedVec<T::AuthorityId, T::MaxAuthorities>,
		) -> DispatchResult {
			T::AuthoritiesOrigin::ensure_origin(origin)?;

			ensure!(!authorities.is_empty(), <Error<T>>::EmptyAuthorities);

			<Authorities<T>>::put(authorities);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::set_trusted_authorities(authorities.len() as u32))]
		pub fn set_trusted_authorities(
			origin: T::RuntimeOrigin,
			authorities: BoundedVec<T::AuthorityId, T::MaxAuthorities>,
		) -> DispatchResult {
			T::AuthoritiesOrigin::ensure_origin(origin)?;

			<TrustedAuthorities<T>>::put(authorities);

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(current: BlockNumberFor<T>) {
			let next = current + BlockNumberFor::<T>::one();
			let zero = BlockNumberFor::<T>::zero();

			let authorities = if next % T::TrustedAuthoritiesPeriod::get() == zero {
				<TrustedAuthorities<T>>::get()
			} else {
				<Authorities<T>>::get()
			};

			// If TrustedAuthorities is empty, change_authorities does nothing.
			pallet_aura::Pallet::<T>::change_authorities(authorities);
		}
	}
}
