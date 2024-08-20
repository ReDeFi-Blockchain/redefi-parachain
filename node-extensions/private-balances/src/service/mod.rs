use x25519_dalek::StaticSecret;

pub struct TrustedService;

impl TrustedService {
	pub fn new() -> Self {
		Self {}
	}

	pub fn get_key(&self, _public: ()) -> Option<StaticSecret> {
		Some(StaticSecret::from([0xAA; 32]))
	}
}
