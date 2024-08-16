#[cfg(feature = "std")]
pub use private_balances::HostFunctions;
pub use private_balances::*;
#[cfg(feature = "std")]
use sp_externalities::ExternalitiesExt;
use sp_runtime_interface::runtime_interface;

#[cfg(feature = "std")]
use crate::PrivateBalancesExt;
use crate::{SharedSecret, X25519Key};

#[runtime_interface]
pub trait PrivateBalances {
	fn get_keys(&mut self) -> u32 {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.get_keys()
	}

	fn diffie_hellman(&mut self, their_public: &X25519Key) -> SharedSecret {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.diffie_hellman(their_public)
	}
}
