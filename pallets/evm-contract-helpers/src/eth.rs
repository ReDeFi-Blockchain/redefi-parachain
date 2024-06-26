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

//! Implementation of magic contract

extern crate alloc;
use core::marker::PhantomData;

use evm_coder::{
	abi::{AbiEncode, AbiType},
	generate_stubgen, solidity_interface,
	types::*,
	ToLog,
};
use frame_support::traits::Get;
use pallet_evm::{
	ExitRevert, OnCreate, OnMethodCall, PrecompileFailure, PrecompileHandle, PrecompileResult,
};
use pallet_evm_coder_substrate::{
	dispatch_to_evm,
	execution::{PreDispatch, Result},
	frontier_contract, SubstrateRecorder, WithRecorder,
};
use sp_core::H160;
use up_sponsorship::SponsorshipHandler;

use crate::{AllowlistEnabled, Config, Owner, Pallet};

frontier_contract! {
	macro_rules! ContractHelpers_result {...}
	impl<T: Config> Contract for ContractHelpers<T> {...}
}

/// Pallet events.
#[derive(ToLog)]
pub enum ContractHelpersEvents {
	/// Contract sponsor was set.
	ContractSponsorSet {
		/// Contract address of the affected collection.
		#[indexed]
		contract_address: Address,
		/// New sponsor address.
		sponsor: Address,
	},

	/// New sponsor was confirm.
	ContractSponsorshipConfirmed {
		/// Contract address of the affected collection.
		#[indexed]
		contract_address: Address,
		/// New sponsor address.
		sponsor: Address,
	},

	/// Collection sponsor was removed.
	ContractSponsorRemoved {
		/// Contract address of the affected collection.
		#[indexed]
		contract_address: Address,
	},
}

/// See [`ContractHelpersCall`]
pub struct ContractHelpers<T: Config>(SubstrateRecorder<T>);
impl<T: Config> WithRecorder<T> for ContractHelpers<T> {
	fn recorder(&self) -> &SubstrateRecorder<T> {
		&self.0
	}

	fn into_recorder(self) -> SubstrateRecorder<T> {
		self.0
	}
}

/// @title Magic contract, which allows users to reconfigure other contracts
#[solidity_interface(name = ContractHelpers, events(ContractHelpersEvents), enum(derive(PreDispatch)))]
impl<T: Config> ContractHelpers<T>
where
	T::AccountId: AsRef<[u8; 32]>,
{
	/// Get user, which deployed specified contract
	/// @dev May return zero address in case if contract is deployed
	///  using uniquenetwork evm-migration pallet, or using other terms not
	///  intended by pallet-evm
	/// @dev Returns zero address if contract does not exists
	/// @param contractAddress Contract to get owner of
	/// @return address Owner of contract
	fn contract_owner(&self, contract_address: Address) -> Result<Address> {
		Ok(<Owner<T>>::get(contract_address))
	}

	/// Is specified user present in contract allow list
	/// @dev Contract owner always implicitly included
	/// @param contractAddress Contract to check allowlist of
	/// @param user User to check
	/// @return bool Is specified users exists in contract allowlist
	fn allowed(&self, contract_address: Address, user: Address) -> Result<bool> {
		self.0.consume_sload()?;
		Ok(<Pallet<T>>::allowed(contract_address, user))
	}

	/// Toggle user presence in contract allowlist
	/// @param contractAddress Contract to change allowlist of
	/// @param user Which user presence should be toggled
	/// @param isAllowed `true` if user should be allowed to be sponsored
	///  or call this contract, `false` otherwise
	/// @dev Only contract owner can change this setting
	fn toggle_allowed(
		&mut self,
		caller: Caller,
		contract_address: Address,
		user: Address,
		is_allowed: bool,
	) -> Result<()> {
		self.recorder().consume_sload()?;
		self.recorder().consume_sstore()?;

		<Pallet<T>>::ensure_owner(contract_address, caller).map_err(dispatch_to_evm::<T>)?;
		<Pallet<T>>::toggle_allowed(contract_address, user, is_allowed);

		Ok(())
	}

	/// Is this contract has allowlist access enabled
	/// @dev Allowlist always can have users, and it is used for two purposes:
	///  in case of allowlist sponsoring mode, users will be sponsored if they exist in allowlist
	///  in case of allowlist access enabled, only users from allowlist may call this contract
	/// @param contractAddress Contract to get allowlist access of
	/// @return bool Is specified contract has allowlist access enabled
	fn allowlist_enabled(&self, contract_address: Address) -> Result<bool> {
		Ok(<AllowlistEnabled<T>>::get(contract_address))
	}

	/// Toggle contract allowlist access
	/// @param contractAddress Contract to change allowlist access of
	/// @param enabled Should allowlist access to be enabled?
	fn toggle_allowlist(
		&mut self,
		caller: Caller,
		contract_address: Address,
		enabled: bool,
	) -> Result<()> {
		self.recorder().consume_sload()?;
		self.recorder().consume_sstore()?;

		<Pallet<T>>::ensure_owner(contract_address, caller).map_err(dispatch_to_evm::<T>)?;
		<Pallet<T>>::toggle_allowlist(contract_address, enabled);
		Ok(())
	}
}

/// Implements [`OnMethodCall`], which delegates call to [`ContractHelpers`]
pub struct HelpersOnMethodCall<T: Config>(PhantomData<*const T>);
impl<T: Config> OnMethodCall<T> for HelpersOnMethodCall<T>
where
	T::AccountId: AsRef<[u8; 32]>,
{
	fn is_reserved(contract: &sp_core::H160) -> bool {
		contract == &T::ContractAddress::get()
	}

	fn is_used(contract: &sp_core::H160) -> bool {
		contract == &T::ContractAddress::get()
	}

	fn call(handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		// TODO: Extract to another OnMethodCall handler
		if <AllowlistEnabled<T>>::get(handle.code_address())
			&& !<Pallet<T>>::allowed(handle.code_address(), handle.context().caller)
		{
			return Some(Err(PrecompileFailure::Revert {
				exit_status: ExitRevert::Reverted,
				output: ("target contract is allowlisted",)
					.abi_encode_call(evm_coder::fn_selector!(Error(string))),
			}));
		}

		if handle.code_address() != T::ContractAddress::get() {
			return None;
		}

		let helpers = ContractHelpers::<T>(SubstrateRecorder::<T>::new(handle.remaining_gas()));
		pallet_evm_coder_substrate::call(handle, helpers)
	}

	fn get_code(contract: &sp_core::H160) -> Option<Vec<u8>> {
		(contract == &T::ContractAddress::get())
			.then(|| include_bytes!("./stubs/ContractHelpers.raw").to_vec())
	}
}

/// Hooks into contract creation, storing owner of newly deployed contract
pub struct HelpersOnCreate<T: Config>(PhantomData<*const T>);
impl<T: Config> OnCreate<T> for HelpersOnCreate<T> {
	fn on_create(owner: H160, contract: H160) {
		<Owner<T>>::insert(contract, owner);
	}
}

generate_stubgen!(contract_helpers_impl, ContractHelpersCall<()>, true);
generate_stubgen!(contract_helpers_iface, ContractHelpersCall<()>, false);
