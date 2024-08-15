#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade};
pub use pallet::*;

pub mod migration;

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Who can call `updateKeys` extrinsic.
		type UpdateKeysOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::storage]
	pub(super) type X25519Key<T: Config> =
		StorageValue<_, redefi_private_balances_runtime_ext::X25519Key, ValueQuery>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub key: Option<redefi_private_balances_runtime_ext::X25519Key>,
		_marker: PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if let Some(key) = self.key {
				<X25519Key<T>>::set(key);
			}
		}
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({0})]
		pub fn update_keys(origin: T::RuntimeOrigin) -> DispatchResult {
			redefi_private_balances_runtime_ext::get_keys();
			Ok(())
		}
	}
}
