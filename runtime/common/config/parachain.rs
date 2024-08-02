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

use cumulus_primitives_core::AggregateMessageOrigin;
use frame_support::{parameter_types, weights::Weight};
use sp_runtime::Perbill;
use staging_xcm_builder::ProcessXcmMessage;
use staging_xcm_executor::XcmExecutor;
use up_common::constants::*;


use super::{substrate::RuntimeBlockWeights, xcm::XcmConfig};
use crate::{MessageQueue, Runtime, RuntimeCall, RuntimeEvent};

parameter_types! {
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

/// Maximum number of blocks simultaneously accepted by the Runtime, not yet included
/// into the relay chain.
pub const UNINCLUDED_SEGMENT_CAPACITY: u32 = 3;
/// How many parachain blocks are processed by the relay chain per parent. Limits the
/// number of blocks authored per slot.
pub const BLOCK_PROCESSING_VELOCITY: u32 = 1;
/// Relay chain slot duration, in milliseconds.
pub const RELAY_CHAIN_SLOT_DURATION_MILLIS: u32 = 6000;

/// Aura consensus hook
pub type ConsensusHook = cumulus_pallet_aura_ext::FixedVelocityConsensusHook<
	Runtime,
	RELAY_CHAIN_SLOT_DURATION_MILLIS,
	BLOCK_PROCESSING_VELOCITY,
	UNINCLUDED_SEGMENT_CAPACITY,
>;

impl cumulus_pallet_parachain_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SelfParaId = staging_parachain_info::Pallet<Self>;
	type OnSystemEvent = ();
	type OutboundXcmpMessageSource = ();
	type ReservedDmpWeight = ReservedDmpWeight;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type XcmpMessageHandler = ();
	type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::RelayNumberMonotonicallyIncreases;
	type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
	type WeightInfo = ();
	type ConsensusHook = ConsensusHook;
}

impl staging_parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

parameter_types! {
	pub MessageQueueServiceWeight: Weight = Perbill::from_percent(35) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
		cumulus_primitives_core::AggregateMessageOrigin,
	>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MessageProcessor =
		ProcessXcmMessage<AggregateMessageOrigin, XcmExecutor<XcmConfig>, RuntimeCall>;
	type Size = u32;
	// The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
	// TODO
	type QueueChangeHandler = ();
	type QueuePausedQuery = ();
	type HeapSize = sp_core::ConstU32<{ 64 * 1024 }>;
	type MaxStale = sp_core::ConstU32<8>;
	type ServiceWeight = MessageQueueServiceWeight;
	type IdleMaxServiceWeight = ();
}
