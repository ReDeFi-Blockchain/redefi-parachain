// Copyright 2019-2022 Unique Network (Gibraltar) Ltd.
// This file is part of Unique Network.

// Unique Network is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Unique Network is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Unique Network. If not, see <http://www.gnu.org/licenses/>.

#[macro_export]
macro_rules! dispatch_unique_runtime {
	($collection:ident.$method:ident($($name:ident),*) $($rest:tt)*) => {{
		let collection = <Runtime as pallet_common::Config>::CollectionDispatch::dispatch($collection)?;
		let dispatch = collection.as_dyn();

		Ok::<_, DispatchError>(dispatch.$method($($name),*) $($rest)*)
	}};
}

#[macro_export]
macro_rules! impl_common_runtime_apis {
	(
		$(
			#![custom_apis]

			$($custom_apis:tt)+
		)?
	) => {
		use sp_std::prelude::*;
		use sp_api::impl_runtime_apis;
		use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H256, U256, H160};
		use sp_runtime::{
			Permill,
			traits::{Block as BlockT},
			transaction_validity::{TransactionSource, TransactionValidity},
			ApplyExtrinsicResult, ExtrinsicInclusionMode,
		};
		use frame_support::{
			pallet_prelude::Weight,
			traits::OnFinalize,
			genesis_builder_helper::{build_config, create_default_config},
		};
		use fp_rpc::TransactionStatus;
		use pallet_transaction_payment::{
			FeeDetails, RuntimeDispatchInfo,
		};
		use pallet_evm::{
			Runner, account::CrossAccountId as _,
			Account as EVMAccount, FeeCalculator,
		};
		use runtime_common::{
			config::ethereum::CrossAccountId,
		};

		impl_runtime_apis! {
			$($($custom_apis)+)?
			impl sp_api::Core<Block> for Runtime {
				fn version() -> RuntimeVersion {
					VERSION
				}

				fn execute_block(block: Block) {
					Executive::execute_block(block)
				}

				fn initialize_block(header: &<Block as BlockT>::Header) -> ExtrinsicInclusionMode {
					Executive::initialize_block(header)
				}
			}

			impl sp_api::Metadata<Block> for Runtime {
				fn metadata() -> OpaqueMetadata {
					OpaqueMetadata::new(Runtime::metadata().into())
				}

				fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
					Runtime::metadata_at_version(version)
				}

				fn metadata_versions() -> sp_std::vec::Vec<u32> {
					Runtime::metadata_versions()
				}
			}

			impl sp_block_builder::BlockBuilder<Block> for Runtime {
				fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
					Executive::apply_extrinsic(extrinsic)
				}

				fn finalize_block() -> <Block as BlockT>::Header {
					Executive::finalize_block()
				}

				fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
					data.create_extrinsics()
				}

				fn check_inherents(
					block: Block,
					data: sp_inherents::InherentData,
				) -> sp_inherents::CheckInherentsResult {
					data.check_extrinsics(&block)
				}

				// fn random_seed() -> <Block as BlockT>::Hash {
				//	   RandomnessCollectiveFlip::random_seed().0
				// }
			}

			impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
				fn validate_transaction(
					source: TransactionSource,
					tx: <Block as BlockT>::Extrinsic,
					hash: <Block as BlockT>::Hash,
				) -> TransactionValidity {
					Executive::validate_transaction(source, tx, hash)
				}
			}

			impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
				fn offchain_worker(header: &<Block as BlockT>::Header) {
					Executive::offchain_worker(header)
				}
			}

			impl fp_rpc::EthereumRuntimeRPCApi<Block> for Runtime {
				fn chain_id() -> u64 {
					<Runtime as pallet_evm::Config>::ChainId::get()
				}

				fn account_basic(address: H160) -> EVMAccount {
					let (account, _) = EVM::account_basic(&address);
					account
				}

				fn gas_price() -> U256 {
					let (price, _) = <Runtime as pallet_evm::Config>::FeeCalculator::min_gas_price();
					price
				}

				fn account_code_at(address: H160) -> Vec<u8> {
					use pallet_evm::OnMethodCall;
					<Runtime as pallet_evm::Config>::OnMethodCall::get_code(&address)
						.unwrap_or_else(|| pallet_evm::AccountCodes::<Runtime>::get(address))
				}

				fn author() -> H160 {
					<pallet_evm::Pallet<Runtime>>::find_author()
				}

				fn storage_at(address: H160, index: U256) -> H256 {
					let mut tmp = [0u8; 32];
					index.to_big_endian(&mut tmp);
					pallet_evm::AccountStorages::<Runtime>::get(address, H256::from_slice(&tmp[..]))
				}

				#[allow(clippy::redundant_closure)]
				fn call(
					from: H160,
					to: H160,
					data: Vec<u8>,
					value: U256,
					gas_limit: U256,
					max_fee_per_gas: Option<U256>,
					max_priority_fee_per_gas: Option<U256>,
					nonce: Option<U256>,
					estimate: bool,
					access_list: Option<Vec<(H160, Vec<H256>)>>,
				) -> Result<pallet_evm::CallInfo, sp_runtime::DispatchError> {
					let config = if estimate {
						let mut config = <Runtime as pallet_evm::Config>::config().clone();
						config.estimate = true;
						Some(config)
					} else {
						None
					};

					let is_transactional = false;
					let validate = false;
					<Runtime as pallet_evm::Config>::Runner::call(
						CrossAccountId::from_eth(from),
						to,
						data,
						value,
						gas_limit.low_u64(),
						max_fee_per_gas,
						max_priority_fee_per_gas,
						nonce,
						access_list.unwrap_or_default(),
						is_transactional,
						validate,
						// TODO we probably want to support external cost recording in non-transactional calls
						None,
						None,

						config.as_ref().unwrap_or_else(|| <Runtime as pallet_evm::Config>::config()),
					).map_err(|err| err.error.into())
				}

				#[allow(clippy::redundant_closure)]
				fn create(
					from: H160,
					data: Vec<u8>,
					value: U256,
					gas_limit: U256,
					max_fee_per_gas: Option<U256>,
					max_priority_fee_per_gas: Option<U256>,
					nonce: Option<U256>,
					estimate: bool,
					access_list: Option<Vec<(H160, Vec<H256>)>>,
				) -> Result<pallet_evm::CreateInfo, sp_runtime::DispatchError> {
					let config = if estimate {
						let mut config = <Runtime as pallet_evm::Config>::config().clone();
						config.estimate = true;
						Some(config)
					} else {
						None
					};

					let is_transactional = false;
					let validate = false;
					<Runtime as pallet_evm::Config>::Runner::create(
						CrossAccountId::from_eth(from),
						data,
						value,
						gas_limit.low_u64(),
						max_fee_per_gas,
						max_priority_fee_per_gas,
						nonce,
						access_list.unwrap_or_default(),
						is_transactional,
						validate,
						// TODO we probably want to support external cost recording in non-transactional calls
						None,
						None,

						config.as_ref().unwrap_or_else(|| <Runtime as pallet_evm::Config>::config()),
					).map_err(|err| err.error.into())
				}

				fn current_transaction_statuses() -> Option<Vec<TransactionStatus>> {
					pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
				}

				fn current_block() -> Option<pallet_ethereum::Block> {
					pallet_ethereum::CurrentBlock::<Runtime>::get()
				}

				fn current_receipts() -> Option<Vec<pallet_ethereum::Receipt>> {
					pallet_ethereum::CurrentReceipts::<Runtime>::get()
				}

				fn current_all() -> (
					Option<pallet_ethereum::Block>,
					Option<Vec<pallet_ethereum::Receipt>>,
					Option<Vec<TransactionStatus>>
				) {
					(
						pallet_ethereum::CurrentBlock::<Runtime>::get(),
						pallet_ethereum::CurrentReceipts::<Runtime>::get(),
						pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
					)
				}

				fn extrinsic_filter(xts: Vec<<Block as BlockT>::Extrinsic>) -> Vec<pallet_ethereum::Transaction> {
					xts.into_iter().filter_map(|xt| match xt.0.function {
						RuntimeCall::Ethereum(pallet_ethereum::Call::transact { transaction }) => Some(transaction),
						_ => None
					}).collect()
				}

				fn elasticity() -> Option<Permill> {
					None
				}

				fn gas_limit_multiplier_support() {}

				fn pending_block(
					xts: Vec<<Block as BlockT>::Extrinsic>,
				) -> (Option<pallet_ethereum::Block>, Option<Vec<TransactionStatus>>) {
					for ext in xts.into_iter() {
						let _ = Executive::apply_extrinsic(ext);
					}

					Ethereum::on_finalize(System::block_number() + 1);

					(
						pallet_ethereum::CurrentBlock::<Runtime>::get(),
						pallet_ethereum::CurrentTransactionStatuses::<Runtime>::get()
					)
				}
			}

			impl sp_genesis_builder::GenesisBuilder<Block> for Runtime {
				fn create_default_config() -> Vec<u8> {
					create_default_config::<RuntimeGenesisConfig>()
				}

				fn build_config(config: Vec<u8>) -> sp_genesis_builder::Result {
					build_config::<RuntimeGenesisConfig>(config)
				}
			}

			impl sp_session::SessionKeys<Block> for Runtime {
				fn decode_session_keys(
					encoded: Vec<u8>,
				) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
					SessionKeys::decode_into_raw_public_keys(&encoded)
				}

				fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
					SessionKeys::generate(seed)
				}
			}

			impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
				fn slot_duration() -> sp_consensus_aura::SlotDuration {
					sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
				}

				fn authorities() -> Vec<AuraId> {
					Aura::authorities().to_vec()
				}
			}

			impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
				fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
					ParachainSystem::collect_collation_info(header)
				}
			}

			impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
				fn account_nonce(account: AccountId) -> Nonce {
					System::account_nonce(account)
				}
			}

			impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
				fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
					TransactionPayment::query_info(uxt, len)
				}
				fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
					TransactionPayment::query_fee_details(uxt, len)
				}
				fn query_weight_to_fee(weight: Weight) -> Balance {
					TransactionPayment::weight_to_fee(weight)
				}
				fn query_length_to_fee(length: u32) -> Balance {
					TransactionPayment::length_to_fee(length)
				}
			}

			#[cfg(feature = "runtime-benchmarks")]
			impl frame_benchmarking::Benchmark<Block> for Runtime {
				fn benchmark_metadata(extra: bool) -> (
					Vec<frame_benchmarking::BenchmarkList>,
					Vec<frame_support::traits::StorageInfo>,
				) {
					use frame_benchmarking::{list_benchmark, Benchmarking, BenchmarkList};
					use frame_support::traits::StorageInfoTrait;

					let mut list = Vec::<BenchmarkList>::new();

					let storage_info = AllPalletsWithSystem::storage_info();

					return (list, storage_info)
				}

				fn dispatch_benchmark(
					config: frame_benchmarking::BenchmarkConfig
				) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
					use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark};
					use sp_storage::TrackedStorageKey;

					let allowlist: Vec<TrackedStorageKey> = vec![
						// Total Issuance
						hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),

						// Block Number
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
						// Execution Phase
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
						// Event Count
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
						// System Events
						hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),

						// Evm CurrentLogs
						hex_literal::hex!("1da53b775b270400e7e61ed5cbc5a146547f210cec367e9af919603343b9cb56").to_vec().into(),

						// Transactional depth
						hex_literal::hex!("3a7472616e73616374696f6e5f6c6576656c3a").to_vec().into(),
					];

					let mut batches = Vec::<BenchmarkBatch>::new();
					let params = (&config, &allowlist);

					if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
					Ok(batches)
				}
			}

			#[cfg(feature = "try-runtime")]
			impl frame_try_runtime::TryRuntime<Block> for Runtime {
				fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
					log::info!("try-runtime::on_runtime_upgrade unique-chain.");
					let weight = Executive::try_runtime_upgrade(checks).unwrap();
					(weight, $crate::config::substrate::RuntimeBlockWeights::get().max_block)
				}

				fn execute_block(
					block: Block,
					state_root_check: bool,
					signature_check: bool,
					select: frame_try_runtime::TryStateSelect
				) -> Weight {
					log::info!(
						target: "node-runtime",
						"try-runtime: executing block {:?} / root checks: {:?} / try-state-select: {:?}",
						block.header.hash(),
						state_root_check,
						select,
					);

					Executive::try_execute_block(block, state_root_check, signature_check, select).unwrap()
				}
			}

			/// Should never be used, yet still required because of https://github.com/paritytech/polkadot-sdk/issues/27
			/// Not allowed to panic, because rpc may be called using native runtime, thus causing thread panic.
			impl fp_rpc::ConvertTransactionRuntimeApi<Block> for Runtime {
				fn convert_transaction(
					transaction: pallet_ethereum::Transaction
				) -> <Block as BlockT>::Extrinsic {
					UncheckedExtrinsic::new_unsigned(
						pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
					)
				}
			}
		}
	}
}
