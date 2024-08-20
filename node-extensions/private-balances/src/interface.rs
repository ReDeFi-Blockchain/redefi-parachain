use alloc::{string::String, vec::Vec};

#[cfg(feature = "std")]
use sp_externalities::ExternalitiesExt;
use sp_runtime_interface::runtime_interface;

#[cfg(feature = "std")]
use crate::extension::PrivateBalancesExt;

#[cfg(feature = "std")]
pub type HostFunctions = (private_balances::HostFunctions, trust::HostFunctions);

#[runtime_interface]
pub trait Trust {
	fn is_trusted(&mut self) -> bool {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.is_trusted()
	}

	fn try_decrypt(
		&mut self,
		encrypted_tx: Vec<u8>,
		ephemeral_key: Vec<u8>,
		nonce: Vec<u8>,
	) -> Result<Vec<u8>, String> {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.try_decrypt(encrypted_tx, ephemeral_key, nonce)
	}
}

pub use trust::{is_trusted, try_decrypt};

// FIXME(vklachkov): Replace [u8; 20] with H160 and [u8; 32] with U256 without contributing to polkadot-sdk.
#[runtime_interface]
pub trait PrivateBalances {
	fn get_balance(&mut self, asset: Option<u128>, account: [u8; 20]) -> Option<[u8; 32]> {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.get_balance(asset, account.into())
			.map(Into::into)
	}

	fn mint(
		&mut self,
		asset: Option<u128>,
		account: [u8; 20],
		amount: [u8; 32],
	) -> Result<(), String> {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.mint(asset, account.into(), amount.into())
	}

	fn burn(
		&mut self,
		asset: Option<u128>,
		account: [u8; 20],
		amount: [u8; 32],
	) -> Result<(), String> {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.burn(asset, account.into(), amount.into())
	}

	fn transfer(
		&mut self,
		asset: Option<u128>,
		from: [u8; 20],
		to: [u8; 20],
		amount: [u8; 32],
	) -> Result<(), String> {
		self.extension::<PrivateBalancesExt>()
			.expect("private balances runtime extension not found")
			.transfer(asset, from.into(), to.into(), amount.into())
	}
}

pub use private_balances::{burn, get_balance, mint, transfer};
