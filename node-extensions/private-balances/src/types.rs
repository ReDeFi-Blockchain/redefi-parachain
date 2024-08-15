use crate::*;
mod key_store;
pub use key_store::*;

mod db;
pub use db::*;

pub(crate) type X25519Key = [u8; 32];
pub(crate) type SharedSecret = [u8; 32];

pub trait PrivateBalances: KeyService {}

pub trait KeyService {
	type SharedSecret;
	type Public;

	fn diffie_hellman(&self, their_public: &Self::Public) -> Self::SharedSecret;
}
