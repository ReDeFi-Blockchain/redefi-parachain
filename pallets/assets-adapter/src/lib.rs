#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use core::ops::Deref;

use evm_coder::{types::*, ToLog};
use frame_support::{
	pallet_prelude::*,
	traits::tokens::{
		fungibles::{
			approvals::{Inspect as _, Mutate as _},
			metadata::Inspect as MetadataInspect,
			Inspect as _, Mutate,
		},
		Preservation,
	},
};
pub use pallet::*;
use pallet_assets::{Pallet as PalletAssets, WeightInfo as _};
use pallet_balances::WeightInfo;
use pallet_evm::{account::CrossAccountId, Pallet as PalletEvm};
use pallet_evm_coder_substrate::{SubstrateRecorder, WithRecorder};
use sp_core::{H160, U256};
pub mod eth;
pub mod handle;
use handle::*;

pub(crate) type SelfWeightOf<T, I = ()> = <T as pallet_assets::Config<I>>::WeightInfo;
pub(crate) type AssetId = u128;

#[frame_support::pallet]
pub mod pallet {
	use alloc::string::String;

	use frame_support::{ensure, storage::Key, traits::Get};

	use super::*;

	#[pallet::error]
	pub enum Error<T, I = ()> {
		// TODO Add more info.
		/// Indicates a failure with the `spender`’s `allowance`. Used in transfers.
		ERC20InsufficientAllowance,
		ERC20InvalidReceiver,
	}

	#[pallet::config]
	pub trait Config<I: 'static = ()>:
		frame_system::Config
		+ pallet_evm_coder_substrate::Config
		+ pallet_assets::Config<I, AssetId = AssetId, Balance = u128>
	{
		/// Address prefix for assets evm mirrors
		#[pallet::constant]
		type AddressPrefix: Get<&'static [u8; 4]>;

		/// Decimals of balance
		type Decimals: Get<u8>;

		/// Collection name
		type Name: Get<String>;

		/// Collection symbol
		type Symbol: Get<String>;
	}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		pub(crate) fn address_to_asset_id(address: &Address) -> Option<AssetId> {
			let (prefix, id) = address.as_fixed_bytes().split_at(4);
			if prefix != T::AddressPrefix::get() {
				return None;
			}
			Some(AssetId::from_be_bytes(<[u8; 16]>::try_from(id).ok()?))
		}

		pub(crate) fn asset_id_to_address(asset: &AssetId) -> H160 {
			let mut buff = [0; 20];
			buff[..4].copy_from_slice(T::AddressPrefix::get());
			buff[4..20].copy_from_slice(&AssetId::to_be_bytes(*asset));
			H160(buff)
		}
	}
}
