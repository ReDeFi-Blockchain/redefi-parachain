use x25519_dalek::StaticSecret;

use super::{KeyService, SharedSecret, X25519Key};
use crate::*;
pub struct EcdhKeystore {
	pub(crate) static_secret: StaticSecret,
}

impl EcdhKeystore {
	pub fn new() -> Self {
		Self {
			static_secret: StaticSecret::random(),
		}
	}
}

impl KeyService for EcdhKeystore {
	type SharedSecret = SharedSecret;

	type Public = X25519Key;

	fn diffie_hellman(&self, their_public: &Self::Public) -> Self::SharedSecret {
		*self
			.static_secret
			.diffie_hellman(&(*their_public).into())
			.as_bytes()
	}
}
