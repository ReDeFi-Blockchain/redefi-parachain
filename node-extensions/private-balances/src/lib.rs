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

pub struct Inner {}

// TODO: Make PrivateBalancesKeystore(Arc<dyn NetworkProvider + Send + Sync>)
// TODO: Make PrivateBalancesDb(RocksDB)

#[cfg(feature = "std")]
impl PrivateBalancesExt {
	/// Create a new instance of `PrivateBalancesExt`.
	pub fn new() -> Self {
		Self(Inner {})
	}

	pub fn get_keys(&self) -> u32 {
		std::fs::write("AAAAAAAAAAAAAAAAAAAAAA", "CONTENT FROM NODE");
		123
	}
}
