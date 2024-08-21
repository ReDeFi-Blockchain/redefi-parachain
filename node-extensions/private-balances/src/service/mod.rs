use x25519_dalek::StaticSecret;
/// To interract with `CryptoKeyService` node.
pub struct OffChainKeyService;

impl OffChainKeyService {
	pub fn new() -> Self {
		Self {}
	}

	pub fn get_key(&self, _public: ()) -> Option<StaticSecret> {
		Some(StaticSecret::from([
			102, 50, 11, 195, 191, 25, 194, 182, 185, 92, 5, 215, 83, 59, 230, 215, 146, 149, 8,
			61, 82, 166, 152, 45, 147, 77, 112, 232, 56, 219, 61, 202,
		]))
	}
}
