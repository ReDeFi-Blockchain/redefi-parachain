#![cfg_attr(not(feature = "std"), no_std)]

mod interface;

pub use interface::get_keys;
#[cfg(feature = "std")]
pub use interface::HostFunctions;

#[cfg(feature = "std")]
sp_externalities::decl_extension! {
	/// TODO
	pub struct PrivateBalancesExt(Inner);
}

struct Inner {
	keystore: PrivateBalancesKeystore,
	db: PrivateBalancesDb,
}

// TODO: Make PrivateBalancesKeystore(Arc<dyn NetworkProvider + Send + Sync>)
// TODO: Make PrivateBalancesDb(RocksDB)

#[cfg(feature = "std")]
impl PrivateBalancesExt {
	/// Create a new instance of `PrivateBalancesExt`.
	pub fn new(keystore: PrivateBalancesKeyStore, db: PrivateBalancesDb) -> Self {
		Self(Inner { keystore, db })
	}

	pub fn do_something_useful(&self) -> u32 {
		// TODO
	}
}
