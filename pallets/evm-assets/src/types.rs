// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Various basic types for use in the assets pallet.

use frame_support::pallet_prelude::*;

use super::*;

pub(super) type AssetId = u128;
pub(super) type Balance = u128;
pub(super) type Address = H160;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct AssetDetails<Balance, Address> {
	/// Can change `owner`, `issuer`, `freezer` and `admin` accounts.
	pub(super) owner: Address,
	/// Can mint tokens.
	pub(super) issuer: Address,
	/// Can thaw tokens, force transfers and burn tokens from any account.
	pub(super) admin: Address,
	/// Can freeze tokens.
	pub(super) freezer: Address,
	/// The total supply across all accounts.
	pub(super) supply: Balance,
	// /// The ED for virtual accounts.
	// pub(super) min_balance: Balance,
	// /// If `true`, then any account with this asset is given a provider reference. Otherwise, it
	// /// requires a consumer reference.
	// pub(super) is_sufficient: bool,
	// /// The total number of accounts.
	// pub(super) accounts: u32,
	// /// The total number of accounts for which we have placed a self-sufficient reference.
	// pub(super) sufficients: u32,
	// /// The total number of approvals.
	// pub(super) approvals: u32,
	// /// The status of the asset
	// pub(super) status: AssetStatus,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct AssetMetadata<BoundedString> {
	/// The user friendly name of this asset. Limited in length by `StringLimit`.
	pub(super) name: BoundedString,
	/// The ticker symbol for this asset. Limited in length by `StringLimit`.
	pub(super) symbol: BoundedString,
	/// The number of decimals this asset uses to represent one unit.
	pub(super) decimals: u8,
	/// Whether the asset metadata may be changed by a non Force origin.
	pub(super) is_frozen: bool,
}
