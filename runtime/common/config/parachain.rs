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

use frame_support::{parameter_types, traits::EnqueueWithOrigin, weights::Weight};
use sp_core::ConstU8;
use up_common::constants::*;

use crate::{Runtime, RuntimeEvent};

parameter_types! {
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SelfParaId = staging_parachain_info::Pallet<Self>;
	type OnSystemEvent = ();
	type OutboundXcmpMessageSource = ();
	type ReservedDmpWeight = ReservedDmpWeight;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type XcmpMessageHandler = ();
	type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
	type DmpQueue = EnqueueWithOrigin<(), ConstU8<0>>;
	type WeightInfo = ();
}

impl staging_parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}
