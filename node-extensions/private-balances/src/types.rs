use crate::*;
mod keystore;
pub use keystore::*;

mod db;
pub use db::*;

pub trait PrivateBalances: KeyService {}

pub trait KeyService {
	type SharedSecret;
	type Public;

	fn diffie_hellman(&self, their_public: &Self::Public) -> Self::SharedSecret;
}
