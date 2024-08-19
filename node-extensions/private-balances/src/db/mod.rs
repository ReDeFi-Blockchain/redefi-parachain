#![cfg(feature = "std")]

mod backend;

use std::path::{Path, PathBuf};

use backend::Backend;

const DB_HASH_LEN: usize = 32;
/// Hash type that this backend uses for the database.
pub type DbHash = [u8; DB_HASH_LEN];

mod columns {
	pub const NUM_COLUMNS: u32 = 2;

	pub const META: u32 = 0;
	pub const NATIVE_BALANCES: u32 = 1;
}

pub struct PrivateBalancesDb {
	backend: Backend,
}

impl PrivateBalancesDb {
	pub fn new(db_config_dir: impl AsRef<Path>) -> Result<Self, String> {
		let path = Self::get_file_path(db_config_dir, "private_balances_rkdb");

		Ok(Self {
			backend: Backend::open(path)?,
		})
	}

	fn get_file_path(db_config_dir: impl AsRef<Path>, db_name: &str) -> PathBuf {
		db_config_dir.as_ref().join("redefi").join(db_name)
	}
}
