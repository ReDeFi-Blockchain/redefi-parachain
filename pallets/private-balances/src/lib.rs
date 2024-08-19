#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::{string::String, vec::Vec};

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
		pub fn update_keys(_origin: T::RuntimeOrigin) -> DispatchResult {
			// TODO
			Ok(())
		}
	}
}

// TODO(vklachkov): There are a lot of non-pallet methods and interface implementation should be out of there.
impl<T: Config> redefi_private_balances_runtime_ext::TrustProvider for Pallet<T> {
	fn is_trusted() -> bool {
		redefi_private_balances_runtime_ext::is_trusted()
	}

	fn get_key() -> Option<redefi_private_balances_runtime_ext::X25519Key> {
		<X25519Key<T>>::try_get().ok()
	}

	fn decrypt(
		encrypted_tx: Vec<u8>,
		ephemeral_key: Vec<u8>,
		nonce: Vec<u8>,
	) -> Result<Vec<u8>, String> {
		redefi_private_balances_runtime_ext::try_decrypt(encrypted_tx, ephemeral_key, nonce)
	}
}

impl<T: Config> redefi_private_balances_runtime_ext::BalancesProvider for Pallet<T> {
	type Account = sp_core::H160;
	type Balance = sp_core::U256;

	fn get(account: Self::Account) -> Option<Self::Balance> {
		redefi_private_balances_runtime_ext::get_balance(account.into()).map(Into::into)
	}

	fn mint(account: Self::Account, amount: Self::Balance) -> Result<(), String> {
		redefi_private_balances_runtime_ext::mint(account.into(), amount.into())
	}

	fn burn(account: Self::Account, amount: Self::Balance) -> Result<(), String> {
		redefi_private_balances_runtime_ext::burn(account.into(), amount.into())
	}

	fn transfer(
		from: Self::Account,
		to: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String> {
		redefi_private_balances_runtime_ext::transfer(from.into(), to.into(), amount.into())
	}
}
