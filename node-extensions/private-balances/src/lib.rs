#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
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

pub trait TrustProvider {
	fn is_trusted() -> bool;

	fn get_key() -> Option<X25519Key>;

	fn decrypt(
		encrypted_tx: Vec<u8>,
		ephemeral_key: Vec<u8>,
		nonce: Vec<u8>,
	) -> Result<Vec<u8>, String>;
}

pub trait BalancesProvider {
	type Account;
	type Balance;

	fn get(account: Self::Account) -> Option<Self::Balance>;

	fn mint(account: Self::Account, amount: Self::Balance) -> Result<(), String>;

	fn burn(account: Self::Account, amount: Self::Balance) -> Result<(), String>;

	fn transfer(
		from: Self::Account,
		to: Self::Account,
		amount: Self::Balance,
	) -> Result<(), String>;
}
