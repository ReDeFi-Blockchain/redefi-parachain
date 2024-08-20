#![cfg_attr(not(feature = "std"), no_std)]

pub mod migration;

extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade};
pub use pallet::*;
use sp_core::{H160, U256};

const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Who can call `updateKeys` extrinsic.
		type UpdateKeysOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Address of account where all private balances tokens will be stored
		/// to ensure correct total supply.
		#[pallet::constant]
		type TreasuryAddress: Get<H160>;
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
			unimplemented!()
		}
	}
}

impl<T: Config> redefi_private_balances_runtime_ext::TrustProvider for Pallet<T> {
	fn is_trusted() -> bool {
		redefi_private_balances_runtime_ext::is_trusted()
	}

	fn get_public_key() -> redefi_private_balances_runtime_ext::X25519Key {
		<X25519Key<T>>::get()
	}

	fn get_treasury_address() -> H160 {
		T::TreasuryAddress::get()
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
	type Account = H160;
	type Asset = u128;
	type Balance = U256;

	fn get(asset: Option<Self::Asset>, account: Self::Account) -> Option<Self::Balance> {
		redefi_private_balances_runtime_ext::get_balance(asset, account.into()).map(Into::into)
	}

	fn mint(
		asset: Option<Self::Asset>,
		account: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String> {
		redefi_private_balances_runtime_ext::mint(asset, account.into(), amount.into())
	}

	fn burn(
		asset: Option<Self::Asset>,
		account: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String> {
		redefi_private_balances_runtime_ext::burn(asset, account.into(), amount.into())
	}

	fn transfer(
		asset: Option<Self::Asset>,
		from: Self::Account,
		to: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String> {
		redefi_private_balances_runtime_ext::transfer(asset, from.into(), to.into(), amount.into())
	}
}
