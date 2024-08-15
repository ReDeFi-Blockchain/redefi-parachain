#![cfg_attr(not(feature = "std"), no_std)]

mod interface;
#[cfg(feature = "std")]
pub mod types;
pub use interface::get_keys;
#[cfg(feature = "std")]
pub use interface::HostFunctions;
#[cfg(feature = "std")]
use types::{EcdhKeystore, KeyService, PrivateBalancesDb};

#[cfg(feature = "std")]
sp_externalities::decl_extension! {
	/// TODO
	pub struct PrivateBalancesExt(Inner);
}

pub type X25519Key = [u8; 32];
pub type SharedSecret = [u8; 32];
// TODO
pub struct HttpClientMock;

#[cfg(feature = "std")]
pub struct Inner {
	key_store: EcdhKeystore,
	db: PrivateBalancesDb,
	http_client: HttpClientMock,
}

// TODO: Make PrivateBalancesKeystore(Arc<dyn NetworkProvider + Send + Sync>)
// TODO: Make PrivateBalancesDb(RocksDB)

#[cfg(feature = "std")]
impl PrivateBalancesExt {
	/// Create a new instance of `PrivateBalancesExt`.
	pub fn new() -> Self {
		Self(Inner {
			key_store: EcdhKeystore::new(),
			db: PrivateBalancesDb,
			http_client: HttpClientMock,
		})
	}

	pub fn get_keys(&self) -> u32 {
		std::fs::write("AAAAAAAAAAAAAAAAAAAAAA", "CONTENT FROM NODE");
		123
	}

	pub fn diffie_hellman(&self, their_public: &X25519Key) -> SharedSecret {
		self.key_store.diffie_hellman(their_public)
	}
}
