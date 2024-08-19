use x25519_dalek::StaticSecret;

pub type X25519Key = [u8; 32];
pub type SharedSecret = [u8; 32];

pub trait PrivateBalances: KeyService {}

pub trait KeyService {
	type SharedSecret;
	type Public;

	fn diffie_hellman(&self, their_public: &Self::Public) -> Self::SharedSecret;
}

pub struct EcdhKeystore {
	pub(crate) static_secret: StaticSecret,
}

impl EcdhKeystore {
	pub fn new() -> Self {
		Self {
			static_secret: StaticSecret::from([0xAA; 32]),
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
