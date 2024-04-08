// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// This file is part of Unique Network.

// Unique Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Unique Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Unique Network. If not, see <http://www.gnu.org/licenses/>.

#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

pub use eth::*;
pub use pallet::*;
pub mod eth;

/// Maximum number of methods per contract that could have fee limit
pub const MAX_FEE_LIMITED_METHODS: u32 = 5;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, sp_runtime::DispatchResult};
	use sp_core::H160;

	pub use super::*;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_evm_coder_substrate::Config + pallet_evm::Config
	{
		/// Address, under which magic contract will be available
		#[pallet::constant]
		type ContractAddress: Get<H160>;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// This method is only executable by contract owner
		NoPermission,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Store owner for contract.
	///
	/// * **Key** - contract address.
	/// * **Value** - owner for contract.
	#[pallet::storage]
	pub(super) type Owner<T: Config> =
		StorageMap<Hasher = Twox128, Key = H160, Value = H160, QueryKind = ValueQuery>;

	/// Storege for contracts with [`Allowlisted`](SponsoringModeT::Allowlisted) sponsoring mode.
	///
	/// ### Usage
	/// Prefer to delete collection from storage if mode chaged to non `Allowlisted`, than set **Value** to **false**.
	///
	/// * **Key** - contract address.
	/// * **Value** - is contract in [`Allowlisted`](SponsoringModeT::Allowlisted) mode.
	#[pallet::storage]
	pub(super) type AllowlistEnabled<T: Config> =
		StorageMap<Hasher = Twox128, Key = H160, Value = bool, QueryKind = ValueQuery>;

	/// Storage for users that allowed for sponsorship.
	///
	/// ### Usage
	/// Prefer to delete record from storage if user no more allowed for sponsorship.
	///
	/// * **Key1** - contract address.
	/// * **Key2** - user that allowed for sponsorship.
	/// * **Value** - allowance for sponsorship.
	#[pallet::storage]
	pub(super) type Allowlist<T: Config> = StorageDoubleMap<
		Hasher1 = Twox128,
		Key1 = H160,
		Hasher2 = Twox128,
		Key2 = H160,
		Value = bool,
		QueryKind = ValueQuery,
	>;

	impl<T: Config> Pallet<T> {
		/// Get contract owner.
		pub fn contract_owner(contract: H160) -> H160 {
			<Owner<T>>::get(contract)
		}

		/// Is user added to allowlist, or he is owner of specified contract
		pub fn allowed(contract: H160, user: H160) -> bool {
			<Allowlist<T>>::get(contract, user) || <Owner<T>>::get(contract) == user
		}

		/// Toggle contract allowlist access
		pub fn toggle_allowlist(contract: H160, enabled: bool) {
			<AllowlistEnabled<T>>::insert(contract, enabled)
		}

		/// Toggle user presence in contract's allowlist
		pub fn toggle_allowed(contract: H160, user: H160, allowed: bool) {
			<Allowlist<T>>::insert(contract, user, allowed);
		}

		/// Throw error if user is not allowed to reconfigure target contract
		pub fn ensure_owner(contract: H160, user: H160) -> DispatchResult {
			ensure!(<Owner<T>>::get(contract) == user, Error::<T>::NoPermission);
			Ok(())
		}
	}
}
