use x25519_dalek::StaticSecret;

pub type X25519Key = [u8; 32];
pub type SharedSecret = [u8; 32];

pub trait PrivateBalances: KeyService {}

pub trait KeyService {
	type SharedSecret;
	type Public;

	fn has_key(&self) -> bool;

	fn diffie_hellman(&self, their_public: &Self::Public) -> Option<Self::SharedSecret>;
}

pub struct EcdhKeystore {
	pub(crate) static_secret: Option<StaticSecret>,
}

impl EcdhKeystore {
	pub fn new() -> Self {
		Self {
			static_secret: Some(StaticSecret::from([0xAA; 32])),
		}
	}

	fn get_static_secret(&self) -> Option<StaticSecret> {
		// TODO(vklachkov): Request key from service
		self.static_secret.clone()
	}
}

impl KeyService for EcdhKeystore {
	type SharedSecret = SharedSecret;

	type Public = X25519Key;

	fn has_key(&self) -> bool {
		self.get_static_secret().is_some()
	}

	fn diffie_hellman(&self, their_public: &Self::Public) -> Option<Self::SharedSecret> {
		Some(
			*self
				.get_static_secret()?
				.diffie_hellman(&(*their_public).into())
				.as_bytes(),
		)
	}
}
