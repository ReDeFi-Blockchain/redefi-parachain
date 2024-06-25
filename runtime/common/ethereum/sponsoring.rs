use core::marker::PhantomData;

use pallet_evm::account::CrossAccountId;
use pallet_evm_transaction_payment::CallContext;
use up_sponsorship::SponsorshipHandler;

use crate::Runtime;

pub type EvmSponsorshipHandler = (
	RedefiEthSponsorshipHandler<Runtime>,
	pallet_evm_contract_helpers::HelpersContractSponsoring<Runtime>,
);

pub struct RedefiEthSponsorshipHandler<T>(PhantomData<T>);

impl<T> SponsorshipHandler<T::CrossAccountId, CallContext> for RedefiEthSponsorshipHandler<T>
where
	T: frame_system::Config + pallet_evm_coder_substrate::Config + pallet_evm::Config,
{
	fn get_sponsor(_who: &T::CrossAccountId, _call: &CallContext) -> Option<T::CrossAccountId> {
		panic!("GET SPONSOR IN RedefiEthSponsorshipHandler");
		None
	}
}
