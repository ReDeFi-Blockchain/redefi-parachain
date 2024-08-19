use std::{path::Path, sync::Arc};

use sp_database::Database;

use super::{columns, DbHash};

pub struct Backend {
	database: Arc<dyn Database<DbHash>>,
}

impl Backend {
	pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
		let mut config = kvdb_rocksdb::DatabaseConfig::with_columns(columns::NUM_COLUMNS);
		config.create_if_missing = true;

		// TODO: Versioning.
		// TODO: Support ParityDB.
		let database =
			kvdb_rocksdb::Database::open(&config, path).map_err(|err| err.to_string())?;

		Ok(Self {
			database: sp_database::as_database(database),
		})
	}
}
