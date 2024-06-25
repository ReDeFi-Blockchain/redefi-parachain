use core::marker::PhantomData;

use pallet_evm::account::CrossAccountId;
use pallet_evm_transaction_payment::CallContext;
use up_sponsorship::SponsorshipHandler;

pub struct RedefiEthSponsorshipHandler<T>(PhantomData<T>);

impl<T> SponsorshipHandler<T::CrossAccountId, CallContext> for RedefiEthSponsorshipHandler<T>
where
	T: frame_system::Config + pallet_evm_coder_substrate::Config + pallet_evm::Config,
	T::AccountId: From<sp_runtime::AccountId32>,
{
	fn get_sponsor(_who: &T::CrossAccountId, call: &CallContext) -> Option<T::CrossAccountId> {
		let bax = &hex_literal::hex!("FFFFFFFF0000000000000000000000000000BABB") as &[u8];
		let red = &hex_literal::hex!("FFFFFFFFBABB0000000000000000000000000000") as &[u8];
		let gbp = &hex_literal::hex!("FFFFFFFFBABB0000000000000000000000000010") as &[u8];

		if ![bax, red, gbp].contains(&call.contract_address.as_bytes()) {
			return None;
		}

		let cross_chain_transfer_selector = hex_literal::hex!("ee18d38e");

		if call.input.starts_with(&cross_chain_transfer_selector) {
			Some(T::CrossAccountId::from_sub(
				crate::Treasury::account_id().into(),
			))
		} else {
			None
		}
	}
}
