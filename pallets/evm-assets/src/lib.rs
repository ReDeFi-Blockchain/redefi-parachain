#![cfg_attr(not(feature = "std"), no_std)]

use evm_coder::ToLog;
use frame_support::{dispatch::DispatchResult, ensure};
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

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::{StorageDoubleMap, *},
		Blake2_128Concat,
	};

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
			const CURRENCY: Balance = 1_000_000;
			const NATIVE: Balance = 1_000_000_000_000_000_000;
			const BALANCE: Balance = 10_000;
			let gbp_id: AssetId = 1;
			let bx_id: AssetId = 0xBABBu16.into();

			let gbp_asset = AssetDetails::<Balance, Address> {
				supply: BALANCE * CURRENCY * self.accounts.len() as Balance,
				..Default::default()
			};
			<Asset<T>>::insert(gbp_id, gbp_asset);

			let bx_asset = AssetDetails::<Balance, Address> {
				supply: BALANCE * NATIVE * self.accounts.len() as Balance,
				..Default::default()
			};
			<Asset<T>>::insert(bx_id, bx_asset);

			self.accounts
				.iter()
				.map(|acc| *T::CrossAccountId::from_sub(acc.clone()).as_eth())
				.for_each(|adr| {
					<Balances<T>>::insert(gbp_id, adr, BALANCE * CURRENCY);
					<Balances<T>>::insert(bx_id, adr, BALANCE * NATIVE);
				});
			let gbp_meta = AssetMetadata::<BoundedVec<u8, T::StringLimit>> {
				name: "Great Britain Pound"
					.as_bytes()
					.to_vec()
					.try_into()
					.unwrap(),
				symbol: "GBP".as_bytes().to_vec().try_into().unwrap(),
				decimals: 6,
				is_frozen: false,
			};

			<Metadata<T>>::insert(gbp_id, gbp_meta);

			let bx_meta = AssetMetadata::<BoundedVec<u8, T::StringLimit>> {
				name: "Relaychain native token"
					.as_bytes()
					.to_vec()
					.try_into()
					.unwrap(),
				symbol: "BAX".as_bytes().to_vec().try_into().unwrap(),
				decimals: 18,
				is_frozen: false,
			};

			<Metadata<T>>::insert(bx_id, bx_meta);
		}
	}
}
