use frame_support::parameter_types;
use frame_system::EnsureRoot;
use sp_core::{hex2array, H160};

use crate::Runtime;

parameter_types! {
	pub const TrustedCollatorsPeriod: u32 = 2;

	/// B1ind F0g
	pub TreasuryAddress: H160 = H160(hex2array!("000000000000000000000000000000000000B1F0"));
}

impl pallet_private_balances_aura_ext::Config for Runtime {
	type AuthoritiesOrigin = EnsureRoot<Self::AccountId>;
	type TrustedAuthoritiesPeriod = TrustedCollatorsPeriod;
	type WeightInfo = pallet_private_balances_aura_ext::weights::SubstrateWeight<Self>;
}

impl pallet_private_balances::Config for Runtime {
	type UpdateKeysOrigin = EnsureRoot<Self::AccountId>;
	type TreasuryAddress = TreasuryAddress;
}
