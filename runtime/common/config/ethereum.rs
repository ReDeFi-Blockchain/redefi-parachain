use frame_support::{
	parameter_types,
	traits::FindAuthor,
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
	ConsensusEngineId,
};
use pallet_ethereum::PostLogContent;
use pallet_evm::{EVMCurrencyAdapter, EnsureAddressTruncated, HashedAddressMapping};
use sp_core::{H160, U256};
use sp_runtime::{traits::ConstU32, Perbill, RuntimeAppPublic};
use up_common::constants::*;

use crate::{
	runtime_common::{ethereum::precompiles::UniquePrecompiles, DealWithFees},
	Aura, Balances, ChainId, Runtime, RuntimeEvent,
};

pub type CrossAccountId = pallet_evm::account::BasicCrossAccountId<Runtime>;

// Assuming PoV size per read is 96 bytes: 16 for twox128(Evm), 16 for twox128(Storage), 32 for storage key, and 32 for storage value
const EVM_SLOAD_PROOF_SIZE: u64 = 96;

// ~~Assuming slowest ethereum opcode is SSTORE, with gas price of 20000 as our worst case~~
// ~~(contract, which only writes a lot of data),~~
// ~~approximating on top of our real store write weight~~
//
// The above approach is very wrong, and the reason is described there:
// https://forum.polkadot.network/t/frontier-support-for-evm-weight-v2/2470/5#problem-2
parameter_types! {
	pub const ReadsPerSecond: u64 = WEIGHT_REF_TIME_PER_SECOND / <Runtime as frame_system::Config>::DbWeight::get().read;
	pub const GasPerSecond: u64 = ReadsPerSecond::get() * 2100;
	pub const WeightTimePerGas: u64 = WEIGHT_REF_TIME_PER_SECOND / GasPerSecond::get();

	pub const BytesReadPerSecond: u64 = ReadsPerSecond::get() * EVM_SLOAD_PROOF_SIZE;
	pub const ProofSizePerGas: u64 = 0; //WEIGHT_REF_TIME_PER_SECOND / GasPerSecond::get();

	pub const WeightPerGas: Weight = Weight::from_parts(WeightTimePerGas::get(), ProofSizePerGas::get());
}

/// Limiting EVM execution to 50% of block for substrate users and management tasks
/// EVM transaction consumes more weight than substrate's, so we can't rely on them being
/// scheduled fairly
const EVM_DISPATCH_RATIO: Perbill = Perbill::from_percent(50);
parameter_types! {
	pub BlockGasLimit: U256 = U256::from((NORMAL_DISPATCH_RATIO * EVM_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT / WeightTimePerGas::get()).ref_time());
	pub PrecompilesValue: UniquePrecompiles<Runtime> = UniquePrecompiles::<_>::new();
}

pub struct EthereumFindAuthor<F>(core::marker::PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for EthereumFindAuthor<F> {
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(author_index) = F::find_author(digests) {
			let authority_id = Aura::authorities()[author_index as usize].clone();
			return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]));
		}
		None
	}
}

pub struct EvmFeeCalculator;
impl fp_evm::FeeCalculator for EvmFeeCalculator {
	fn min_gas_price() -> (sp_core::U256, Weight) {
		(
			up_common::constants::MIN_GAS_PRICE.into(),
			Default::default(),
		)
	}
}

impl pallet_evm::Config for Runtime {
	type CrossAccountId = CrossAccountId;
	type AddressMapping = HashedAddressMapping<Self::Hashing>;
	type BackwardsAddressMapping = HashedAddressMapping<Self::Hashing>;
	type BlockGasLimit = BlockGasLimit;
	type FeeCalculator = EvmFeeCalculator;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressTruncated<Self>;
	type WithdrawOrigin = EnsureAddressTruncated<Self>;
	type PrecompilesType = UniquePrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type OnMethodCall = (pallet_evm_contract_helpers::HelpersOnMethodCall<Self>,);
	type OnCreate = pallet_evm_contract_helpers::HelpersOnCreate<Self>;
	type ChainId = ChainId;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = EVMCurrencyAdapter<Balances, DealWithFees>;
	type FindAuthor = EthereumFindAuthor<Aura>;
	type Timestamp = crate::Timestamp;
	type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
	type GasLimitPovSizeRatio = ProofSizePerGas;
	type OnCheckEvmTransaction = ();
	type SuicideQuickClearLimit = ConstU32<0>;
}

parameter_types! {
	pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
	type PostLogContent = PostBlockAndTxnHashes;
	// Space for revert reason. Ethereum transactions are not cheap, and overall size is much less
	// than the substrate tx size, so we can afford this
	type ExtraDataLength = ConstU32<32>;
}

parameter_types! {
	// 0x842899ECF380553E8a4de75bF534cdf6fBF64049
	pub const HelpersContractAddress: H160 = H160([
		0x84, 0x28, 0x99, 0xec, 0xf3, 0x80, 0x55, 0x3e, 0x8a, 0x4d, 0xe7, 0x5b, 0xf5, 0x34, 0xcd, 0xf6, 0xfb, 0xf6, 0x40, 0x49,
	]);
}

impl pallet_evm_contract_helpers::Config for Runtime {
	type ContractAddress = HelpersContractAddress;
}

impl pallet_evm_coder_substrate::Config for Runtime {}
