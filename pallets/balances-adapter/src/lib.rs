#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use core::ops::Deref;

use evm_coder::{types::*, ToLog};
use frame_support::{
	pallet_prelude::*,
	traits::{
		fungible::{Dust, Unbalanced},
		tokens::fungible::{Inspect, Mutate},
	},
};
pub use pallet::*;
use pallet_balances::WeightInfo;
use pallet_ethereum::Origin as EthereumOrigin;
use pallet_evm::{account::CrossAccountId, Pallet as PalletEvm};
use pallet_evm_coder_substrate::{SubstrateRecorder, WithRecorder};
use pallet_xcm::{Pallet as PalletXcm, WeightInfo as PalletXcmWeightInfo};
use sp_core::{H160, U256};
use sp_std::{boxed::Box, collections::btree_map::BTreeMap};
use staging_xcm::{
	latest::{Asset as XcmAsset, Fungibility, Junction, Location},
	prelude::WeightLimit,
};
pub mod eth;
pub mod handle;
use handle::*;
mod impl_fungible;
mod types;
use types::*;

pub(crate) type SelfWeightOf<T> = <T as Config>::WeightInfo;
pub(crate) type ChainId = u64;

pub(crate) const LOG_TARGET: &str = "runtime::balances-adapter";

#[frame_support::pallet]
pub mod pallet {
	use alloc::string::String;

	use frame_support::{
		ensure,
		storage::Key,
		traits::{
			tokens::{Balance, Fortitude, Precision, Preservation},
			Get,
		},
	};

	use super::*;

	#[pallet::error]
	pub enum Error<T> {
		ERC20InsufficientAllowance,
		ERC20InvalidReceiver,
		ERC20InvalidApprover,
		ERC20InvalidSender,
		Erc20InvalidSpender,
		ERC20InsufficientBalance,
		OwnableUnauthorizedAccount,
		UnauthorizedAccount,
		AssetNotFound,
	}

	#[pallet::storage]
	pub type Allowance<T: Config> = StorageNMap<
		Key = (
			Key<Blake2_128, Address>,       // Owner
			Key<Blake2_128Concat, Address>, // Spender
		),
		Value = u128,
		QueryKind = ValueQuery,
	>;

	#[pallet::storage]
	pub(super) type Permissions<T: Config> =
		StorageMap<_, Blake2_128Concat, H160, AccountPermissions, ValueQuery>;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_evm_coder_substrate::Config + pallet_xcm::Config
	{
		type SudoAccountId: for<'a> TryFrom<&'a [u8]> + Into<Self::AccountId>;

		type Balances: Mutate<Self::AccountId, Balance = Self::NativeBalance>;

		type NativeBalance: Balance + Into<U256> + TryFrom<U256> + From<u128> + Into<u128>;

		/// Address, under which magic contract will be available
		#[pallet::constant]
		type ContractAddress: Get<H160>;

		/// Decimals of balance
		type Decimals: Get<u8>;

		/// Collection name
		type Name: Get<String>;

		/// Collection symbol
		type Symbol: Get<String>;

		/// The type must contain only correct
		/// and supported 'Location', since it is used "as is"
		/// and its use does not imply deep checks
		#[pallet::constant]
		type ChainLocator: Get<BTreeMap<ChainId, Location>>;

		/// Weight information
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		pub fn balance_of(account: &T::CrossAccountId) -> u128 {
			T::Balances::balance(account.as_sub()).into()
		}

		pub fn total_balance(account: &T::CrossAccountId) -> u128 {
			T::Balances::total_balance(account.as_sub()).into()
		}

		pub fn total_issuance() -> u128 {
			T::Balances::total_issuance().into()
		}

		pub fn allowance(owner: &T::CrossAccountId, spender: &T::CrossAccountId) -> U256 {
			<Allowance<T>>::get((owner.as_eth(), spender.as_eth())).into()
		}

		pub fn approve(
			owner: &T::CrossAccountId,
			spender: &T::CrossAccountId,
			amount: u128,
			emit_event: bool,
		) -> DispatchResult {
			Self::check_receiver(spender)?;

			let owner = *owner.as_eth();
			let spender = *spender.as_eth();

			<Allowance<T>>::set((&owner, &spender), amount);

			if emit_event {
				<PalletEvm<T>>::deposit_log(
					eth::ERC20Events::Approval {
						owner,
						spender,
						value: amount.into(),
					}
					.to_log(T::ContractAddress::get()),
				);
			};

			Ok(())
		}

		/// Updates `owner` s allowance for `spender` based on spent `value`.
		pub fn spend_allowance(
			owner: &T::CrossAccountId,
			spender: &T::CrossAccountId,
			amount: u128,
		) -> DispatchResult {
			let key = (owner.as_eth(), spender.as_eth());
			let current_allowance = <Allowance<T>>::get(&key);

			ensure!(
				current_allowance >= amount,
				<Error<T>>::ERC20InsufficientAllowance
			);

			<Allowance<T>>::mutate(&key, |allowance| *allowance -= amount);
			Ok(())
		}

		/// Transfers the specified amount of tokens.
		///
		/// - `from`: Owner of tokens to transfer.
		/// - `to`: Recepient of transfered tokens.
		/// - `amount`: Amount of tokens to transfer.
		pub fn transfer(
			from: &T::CrossAccountId,
			to: &T::CrossAccountId,
			amount: u128,
		) -> DispatchResult {
			Self::check_receiver(to)?;

			{
				let amount = amount.into();
				T::Balances::transfer(
					from.as_sub(),
					to.as_sub(),
					amount,
					Preservation::Expendable,
				)?;
			}

			<PalletEvm<T>>::deposit_log(
				eth::ERC20Events::Transfer {
					from: *from.as_eth(),
					to: *to.as_eth(),
					value: amount.into(),
				}
				.to_log(T::ContractAddress::get()),
			);

			Ok(())
		}

		/// Transfer tokens from one account to another.
		///
		/// Same as the [`Self::transfer`] but the spender doesn't needs to be the direct owner of the token.
		/// The spender must be allowed to transfer token.
		///
		/// - `spender`: Account that spend the money.
		/// - `from`: Owner of tokens to transfer.
		/// - `to`: Recepient of transfered tokens.
		/// - `amount`: Amount of tokens to transfer.
		pub fn transfer_from(
			spender: &T::CrossAccountId,
			from: &T::CrossAccountId,
			to: &T::CrossAccountId,
			amount: u128,
		) -> DispatchResult {
			Self::spend_allowance(from, spender, amount)?;
			Self::transfer(from, to, amount)
		}

		fn check_receiver(receiver: &T::CrossAccountId) -> DispatchResult {
			ensure!(
				&T::CrossAccountId::from_eth(H160::zero()) != receiver,
				<Error<T>>::ERC20InvalidReceiver
			);
			Ok(())
		}

		pub fn check_root(account: &Address) -> DispatchResult {
			const SUDO_STORAGE_KEY: [u8; 32] = hex_literal::hex!(
				"5c0d1176a568c1f92944340dbfed9e9c530ebca703c85910e7164cb7d1c9e47b"
			);

			let Some(sudoer_raw_key) = sp_io::storage::get(&SUDO_STORAGE_KEY) else {
				log::error!(target: LOG_TARGET, "Sudo key not found");
				return Err(<Error<T>>::OwnableUnauthorizedAccount.into());
			};

			let Ok(sudoer_key) = T::SudoAccountId::try_from(&sudoer_raw_key) else {
				log::error!(target: LOG_TARGET, "Failed to deserialize sudo key {sudoer_raw_key:?}");
				return Err(<Error<T>>::OwnableUnauthorizedAccount.into());
			};

			let cross_sudoer_key = T::CrossAccountId::from_sub(sudoer_key.into());
			if cross_sudoer_key.as_eth() == account {
				Ok(())
			} else {
				Err(<Error<T>>::OwnableUnauthorizedAccount.into())
			}
		}

		pub fn set_account_permissions(account: &Address, permissions: AccountPermissions) {
			if permissions.is_empty() {
				<Permissions<T>>::remove(account);
			} else {
				<Permissions<T>>::insert(account, permissions);
			}
		}

		pub fn check_account_permissions(
			account: &Address,
			permissions: AccountPermissions,
		) -> DispatchResult {
			if Self::check_root(account).is_ok() {
				return Ok(());
			}

			let account_permissions =
				<Permissions<T>>::try_get(account).map_err(|_| <Error<T>>::UnauthorizedAccount)?;

			if account_permissions.contains(permissions) {
				Ok(())
			} else {
				Err(<Error<T>>::UnauthorizedAccount.into())
			}
		}

		pub fn mint(to: &Address, amount: u128) -> DispatchResult {
			ensure!(to != &Address::zero(), <Error<T>>::ERC20InvalidSender);
			Self::mint_into(to, amount.into())?;
			Ok(())
		}

		pub fn burn(from: &Address, amount: u128) -> DispatchResult {
			ensure!(from != &Address::zero(), <Error<T>>::ERC20InvalidSender);
			Self::burn_from(from, amount.into(), Precision::Exact, Fortitude::Polite)?;
			Ok(())
		}
	}
}
