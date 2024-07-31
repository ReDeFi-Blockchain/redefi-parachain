pub use private_balances::get_keys;
#[cfg(feature = "std")]
pub use private_balances::HostFunctions;
#[cfg(feature = "std")]
use sp_externalities::ExternalitiesExt;
use sp_runtime_interface::runtime_interface;

#[cfg(feature = "std")]
use crate::PrivateBalancesExt;

#[runtime_interface]
pub trait PrivateBalances {
	fn get_keys(&mut self) -> u32 {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.get_keys()
	}
}
