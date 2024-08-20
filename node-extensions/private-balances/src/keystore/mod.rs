#[cfg(feature = "std")]
use x25519_dalek::StaticSecret;

#[cfg(feature = "std")]
use crate::service::TrustedService;

pub type X25519Key = [u8; 32];
pub type SharedSecret = [u8; 32];

#[cfg(feature = "std")]
pub trait PrivateBalances: KeyService {}

#[cfg(feature = "std")]
pub trait KeyService {
	type SharedSecret;
	type Public;

	fn has_key(&self) -> bool;

	fn diffie_hellman(&self, their_public: &Self::Public) -> Option<Self::SharedSecret>;
}

#[cfg(feature = "std")]
pub struct EcdhKeystore {
	trusted_service: TrustedService,
	static_secret: std::cell::RefCell<Option<StaticSecret>>,
}

#[cfg(feature = "std")]
impl EcdhKeystore {
	pub fn new() -> Self {
		Self {
			trusted_service: TrustedService::new(),
			static_secret: Default::default(),
		}
	}

	fn get_static_secret(&self) -> Option<StaticSecret> {
		let secret = self.trusted_service.get_key(());

		self.static_secret.borrow_mut().clone_from(&secret);

		secret
	}
}

#[cfg(feature = "std")]
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
