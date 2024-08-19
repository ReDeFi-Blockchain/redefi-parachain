#![cfg(feature = "std")]

use std::path::Path;

use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit};
use sp_core::{H160, U256};

use crate::{
	db::PrivateBalancesDb,
	keystore::{EcdhKeystore, KeyService, SharedSecret, X25519Key},
	service::TrustedService,
};

sp_externalities::decl_extension! {
	/// TODO
	pub struct PrivateBalancesExt(Inner);
}

pub struct Inner {
	db: PrivateBalancesDb,
	keystore: EcdhKeystore,
	trusted_service: TrustedService,
}

impl PrivateBalancesExt {
	/// Create a new instance of `PrivateBalancesExt`.
	pub fn new(db_config_dir: impl AsRef<Path>) -> Result<Self, String> {
		Ok(Self(Inner {
			db: PrivateBalancesDb::new(db_config_dir)?,
			keystore: EcdhKeystore::new(),
			trusted_service: TrustedService::new(),
		}))
	}

	/// Returns true if has trusted key in keystore.
	pub fn is_trusted(&self) -> bool {
		self.keystore.has_key()
	}

	/// Trying to decrypt `encrypted_tx`.
	pub fn try_decrypt(
		&self,
		encrypted_tx: Vec<u8>,
		ephemeral_key: Vec<u8>,
		nonce: Vec<u8>,
	) -> Result<Vec<u8>, String> {
		if encrypted_tx.is_empty() {
			return Ok(Vec::new());
		}

		// TODO(vklachkov): Maybe new type?
		let Ok(ephemeral_key): Result<[u8; 32], _> = ephemeral_key.try_into() else {
			return Err("ephemeral_key should be 32 bytes".into());
		};

		// TODO(vklachkov): Maybe new type?
		let Ok(nonce): Result<[u8; 12], _> = nonce.try_into() else {
			return Err("nonce should be 12 bytes".into());
		};

		let Some(shared_secret) = self.keystore.diffie_hellman(&ephemeral_key) else {
			return Err("no trusted key found".into());
		};

		let key = Key::<Aes256Gcm>::from_slice(&shared_secret);

		Aes256Gcm::new(key)
			.decrypt(&nonce.into(), encrypted_tx.as_slice())
			.map_err(|e| format!("{e:?}"))
	}

	/// Get balance of private balance.
	pub fn get_balance(&self, account: H160) -> Option<U256> {
		todo!()
	}

	/// Increase balance of given account.
	pub fn mint(&self, account: H160, amount: U256) -> Result<(), String> {
		todo!()
	}

	/// Decrease balance of given account.
	pub fn burn(&self, account: H160, amount: U256) -> Result<(), String> {
		todo!()
	}

	/// Decrease balance of first account and increase of second atomically.
	pub fn transfer(&self, from: H160, to: H160, amount: U256) -> Result<(), String> {
		todo!()
	}
}
