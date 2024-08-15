use x25519_dalek::StaticSecret;

use super::{KeyService, SharedSecret, X25519Key};

pub struct EcdhKeystore {
	pub(crate) static_secret: StaticSecret,
}

impl EcdhKeystore {
	pub fn new() -> Self {
		Self {
			static_secret: StaticSecret::from([
				102, 50, 11, 195, 191, 25, 194, 182, 185, 92, 5, 215, 83, 59, 230, 215, 146, 149,
				8, 61, 82, 166, 152, 45, 147, 77, 112, 232, 56, 219, 61, 202,
			]),
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
