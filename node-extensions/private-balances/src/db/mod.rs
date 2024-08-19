#![cfg(feature = "std")]

use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use parity_scale_codec::Encode;
use sp_core::{H160, U256};
use sp_database::Database;

const DB_HASH_LEN: usize = 32;
/// Hash type that this backend uses for the database.
pub type DbHash = [u8; DB_HASH_LEN];

mod columns {
	pub const NUM_COLUMNS: u32 = 2;

	// pub const META: u32 = 0;
	pub const BALANCES: u32 = 1;
}

#[derive(Clone, Copy, Debug, parity_scale_codec::Encode)]
struct Key {
	asset: Option<u128>,
	account: H160,
}

pub struct PrivateBalancesDb {
	db: Arc<dyn Database<DbHash>>,
}

impl PrivateBalancesDb {
	pub fn new(db_config_dir: impl AsRef<Path>) -> Result<Self, String> {
		let path = Self::get_file_path(db_config_dir, "private_balances_rkdb");
		let db = Self::open_database(path)?;

		Ok(Self { db })
	}

	fn get_file_path(db_config_dir: impl AsRef<Path>, db_name: &str) -> PathBuf {
		db_config_dir.as_ref().join("redefi").join(db_name)
	}

	fn open_database(path: impl AsRef<Path>) -> Result<Arc<dyn Database<DbHash>>, String> {
		let mut config = kvdb_rocksdb::DatabaseConfig::with_columns(columns::NUM_COLUMNS);
		config.create_if_missing = true;

		let database =
			kvdb_rocksdb::Database::open(&config, path).map_err(|err| err.to_string())?;

		Ok(sp_database::as_database(database))
	}

	pub fn get_balance(&self, asset: Option<u128>, account: H160) -> Option<U256> {
		let key = Key { asset, account }.encode();
		let value = self.db.get(columns::BALANCES, &key)?;

		Some(U256::from_little_endian(value.as_slice()))
	}

	pub fn increase_balance(
		&self,
		asset: Option<u128>,
		account: H160,
		amount: U256,
	) -> Result<(), String> {
		let current_balance = self.get_balance(asset, account).unwrap_or_default();

		let balance = current_balance
			.checked_add(amount)
			.ok_or("balance overflow")?;

		let mut transaction = sp_database::Transaction::new();

		let key = Key { asset, account }.encode();
		let value = u256bytes(balance);

		transaction.set(columns::BALANCES, &key, &value);

		self.db
			.commit(transaction)
			.map_err(|err| format!("{err:?}"))
	}

	pub fn decrease_balance(
		&self,
		asset: Option<u128>,
		account: H160,
		amount: U256,
	) -> Result<(), String> {
		let current_balance = self.get_balance(asset, account).unwrap_or_default();

		let balance = current_balance
			.checked_sub(amount)
			.ok_or("not enough balance on account")?;

		let mut transaction = sp_database::Transaction::new();

		let key = Key { asset, account }.encode();
		let value = u256bytes(balance);

		transaction.set(columns::BALANCES, &key, &value);

		self.db
			.commit(transaction)
			.map_err(|err| format!("{err:?}"))
	}

	pub fn transfer(
		&self,
		asset: Option<u128>,
		from: H160,
		to: H160,
		amount: U256,
	) -> Result<(), String> {
		let current_sender_balance = self.get_balance(asset, from).unwrap_or_default();

		let sender_balance = current_sender_balance
			.checked_sub(amount)
			.ok_or("not enough balance on sender account")?;

		let current_recipient_balance = self.get_balance(asset, to).unwrap_or_default();

		let recipient_balance = current_recipient_balance
			.checked_add(amount)
			.ok_or("recipient balance overflow")?;

		let mut transaction = sp_database::Transaction::new();

		{
			let key = Key {
				asset,
				account: from,
			}
			.encode();
			let value = u256bytes(sender_balance);
			transaction.set(columns::BALANCES, &key, &value);
		}

		{
			let key = Key { asset, account: to }.encode();
			let value = u256bytes(recipient_balance);
			transaction.set(columns::BALANCES, &key, &value);
		}

		self.db
			.commit(transaction)
			.map_err(|err| format!("{err:?}"))
	}
}

#[inline(always)]
fn u256bytes(value: U256) -> [u8; 32] {
	let mut raw = [0u8; 32];
	value.to_little_endian(&mut raw);
	raw
}
