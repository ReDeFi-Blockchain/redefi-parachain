#![feature(once_cell_try)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

#[cfg(feature = "std")]
mod extension;

#[cfg(feature = "std")]
pub mod db;

pub mod keystore;

#[cfg(feature = "std")]
pub mod service;

mod interface;

#[cfg(feature = "std")]
pub use extension::PrivateBalancesExt;
pub use interface::*;
pub use keystore::X25519Key;
use sp_core::H160;

pub trait TrustProvider {
	fn is_trusted() -> bool;

	fn get_public_key() -> X25519Key;

	fn get_treasury_address() -> H160;

	fn decrypt(
		encrypted_tx: Vec<u8>,
		ephemeral_key: Vec<u8>,
		nonce: Vec<u8>,
	) -> Result<Vec<u8>, String>;
}

pub trait BalancesProvider {
	type Account;
	type Asset;
	type Balance;

	fn get(asset: Option<Self::Asset>, account: Self::Account) -> Option<Self::Balance>;

	fn mint(
		asset: Option<Self::Asset>,
		account: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String>;

	fn burn(
		asset: Option<Self::Asset>,
		account: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String>;

	fn transfer(
		asset: Option<Self::Asset>,
		from: Self::Account,
		to: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String>;
}
