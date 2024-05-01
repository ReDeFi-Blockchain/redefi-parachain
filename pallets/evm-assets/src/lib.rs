#![cfg_attr(not(feature = "std"), no_std)]

use evm_coder::ToLog;
use frame_support::{
	dispatch::DispatchResult, ensure, pallet_prelude::*, traits::OnRuntimeUpgrade,
};
pub use pallet::*;
use pallet_evm::{account::CrossAccountId, Pallet as PalletEvm};
use pallet_evm_coder_substrate::{types::String, SubstrateRecorder, WithRecorder};
use sp_core::{Get, H160, U256};
use sp_runtime::ArithmeticError;
use sp_std::{marker::PhantomData, ops::Deref, prelude::*};
pub mod types;
use types::*;

pub mod functions;

pub mod eth;

pub mod hanlde;
use hanlde::*;

pub mod migration;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::Blake2_128Concat;

	use self::migration::init_assets_with;
	use super::*;

	/// The in-code storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::error]
	pub enum Error<T> {
		ERC20InsufficientAllowance,
		ERC20InvalidReceiver,
		ERC20InvalidApprover,
		ERC20InvalidSender,
		Erc20InvalidSpender,
		ERC20InsufficientBalance,
		AssetNotFound,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_evm_coder_substrate::Config {
		/// Address prefix for assets evm mirrors
		#[pallet::constant]
		type AddressPrefix: Get<[u8; 4]>;

		/// The maximum length of a name or symbol stored on-chain.
		#[pallet::constant]
		type StringLimit: Get<u32>;
	}

	#[pallet::storage]
	/// Details of an asset.
	pub(super) type Asset<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, AssetDetails<Balance, Address>>;

	#[pallet::storage]
	pub(super) type Approvals<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, AssetId>,
			NMapKey<Blake2_128Concat, Address>, // owner
			NMapKey<Blake2_128Concat, Address>, // spender
		),
		Balance,
		ValueQuery,
	>;

	#[pallet::storage]
	/// Balances
	pub(super) type Balances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AssetId,
		Blake2_128Concat,
		Address,
		Balance,
		ValueQuery,
	>;

	#[pallet::storage]
	/// Metadata of an asset.
	pub(super) type Metadata<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetId, AssetMetadata<BoundedVec<u8, T::StringLimit>>>;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		accounts: Vec<T::AccountId>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			init_assets_with::<T>(&self.accounts[..])
		}
	}
}
