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
macro_rules! construct_runtime {
	() => {
		frame_support::construct_runtime! {

			pub enum Runtime {
				System: frame_system = 0,

				ParachainSystem: cumulus_pallet_parachain_system = 20,
				ParachainInfo: staging_parachain_info = 21,

				Aura: pallet_aura = 25,
				AuraExt: cumulus_pallet_aura_ext = 26,

				Balances: pallet_balances = 30,

				// RandomnessCollectiveFlip = 31
				Timestamp: pallet_timestamp = 32,
				TransactionPayment: pallet_transaction_payment = 33,
				Treasury: pallet_treasury = 34,
				Sudo: pallet_sudo = 35,

				// XCM

				PolkadotXcm: pallet_xcm = 41,
				CumulusXcm: cumulus_pallet_xcm = 42,
				MessageQueue: pallet_message_queue = 43,

				// Frontier
				EVM: pallet_evm = 100,
				Ethereum: pallet_ethereum = 101,

				EvmCoderSubstrate: pallet_evm_coder_substrate = 150,
				EvmContractHelpers: pallet_evm_contract_helpers = 151,
				BalancesAdapter: pallet_balances_adapter = 152,
				EvmAssets: pallet_evm_assets = 153,
				Utility: pallet_utility = 156,
			}
		}
	};
}
