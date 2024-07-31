use frame_support::parameter_types;
use frame_system::EnsureRoot;

use crate::Runtime;

parameter_types! {
	pub const TrustedCollatorsPeriod: u32 = 2;
}

impl pallet_private_balances_aura_ext::Config for Runtime {
	type AuthoritiesOrigin = EnsureRoot<Self::AccountId>;
	type TrustedAuthoritiesPeriod = TrustedCollatorsPeriod;
	type WeightInfo = pallet_private_balances_aura_ext::weights::SubstrateWeight<Self>;
}

impl pallet_private_balances::Config for Runtime {
	type UpdateKeysOrigin = EnsureRoot<Self::AccountId>;
}
