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

use default_runtime::RuntimeGenesisConfig;
#[cfg(feature = "redefi-runtime")]
pub use redefi_runtime as default_runtime;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use serde_json::map::Map;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use up_common::types::opaque::*;

pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

/// PARA_ID for redefi FIXME
const PARA_ID: u32 = 2222;

pub trait RuntimeIdentification {
	fn runtime_id(&self) -> RuntimeId;
}

impl RuntimeIdentification for Box<dyn sc_service::ChainSpec> {
	fn runtime_id(&self) -> RuntimeId {
		#[cfg(feature = "redefi-runtime")]
		if self.id().starts_with("redefi") {
			return RuntimeId::Redefi;
		}

		RuntimeId::Unknown(self.id().into())
	}
}

pub enum ServiceId {
	Prod,
	Dev,
}

pub trait ServiceIdentification {
	fn service_id(&self) -> ServiceId;
}

impl ServiceIdentification for Box<dyn sc_service::ChainSpec> {
	fn service_id(&self) -> ServiceId {
		if self.id().ends_with("dev") {
			ServiceId::Dev
		} else {
			ServiceId::Prod
		}
	}
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`DefaultChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

macro_rules! testnet_genesis {
	(
		$runtime:path,
		$root_key:expr,
		$initial_invulnerables:expr,
		$initial_trusted:expr,
		$endowed_accounts:expr,
		$id:expr
	) => {{
		serde_json::json!({
			"sudo": {
				"key": Some($root_key),
			},
			"balances": {
				"balances": $endowed_accounts
					.iter()
					.cloned()
					// 1e13 UNQ
					.map(|k| (k, 2_000_000_000_000_000_000_000_000_000_000u128))
					.collect::<Vec<_>>()
			},
			"evmAssets": {
				"accounts": [$root_key],
			},
			"parachainInfo": {
				"parachainId": $id,
			},
			"aura": {
				"authorities": $initial_invulnerables
					.into_iter()
					.map(|(_, aura)| aura)
					.collect::<Vec<_>>(),
			},
			"privateBalancesAuraExt": {
				"trustedAuthorities": $initial_trusted
					.into_iter()
					.map(|(_, aura)| aura)
					.collect::<Vec<_>>(),
			},
			"privateBalances": {
				"key": Some([53, 4, 137, 17, 40, 242, 206, 104, 152, 206, 62, 230, 187, 9, 142, 128, 225, 195, 194, 6, 230, 177, 89, 73, 168, 231, 38, 228, 38, 146, 236, 116]),
			}
		})
	}};
}

pub fn development_config() -> ChainSpec {
	let mut properties = Map::new();
	properties.insert("tokenSymbol".into(), default_runtime::TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), default_runtime::DECIMALS.into());
	properties.insert(
		"ss58Format".into(),
		default_runtime::SS58Prefix::get().into(),
	);

	ChainSpec::builder(
		default_runtime::WASM_BINARY.expect("WASM not available"),
		Extensions {
			relay_chain: "rococo-dev".into(),
			para_id: PARA_ID,
		},
	)
	.with_name(
		// Name
		format!(
			"{}{}",
			default_runtime::VERSION.impl_name.to_uppercase(),
			if cfg!(feature = "redefi-runtime") {
				""
			} else {
				" by Redefi"
			}
		)
		.as_str(),
	)
	.with_id(format!("{}_dev", default_runtime::VERSION.spec_name).as_str())
	.with_chain_type(ChainType::Local)
	.with_properties(properties)
	.with_genesis_config_patch(testnet_genesis!(
		default_runtime,
		// Sudo account
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		[
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_from_seed::<AuraId>("Bob"),
			),
		],
		[(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_from_seed::<AuraId>("Alice"),
		)],
		// Pre-funded accounts
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		],
		PARA_ID
	))
	.build()
}

pub fn local_testnet_config() -> ChainSpec {
	let mut properties = Map::new();
	properties.insert("tokenSymbol".into(), default_runtime::TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), default_runtime::DECIMALS.into());
	properties.insert(
		"ss58Format".into(),
		default_runtime::SS58Prefix::get().into(),
	);

	ChainSpec::builder(
		default_runtime::WASM_BINARY.expect("WASM not available"),
		Extensions {
			relay_chain: "westend-local".into(),
			para_id: PARA_ID,
		},
	)
	.with_name(
		// Name
		format!(
			"{}{}",
			default_runtime::VERSION.impl_name.to_uppercase(),
			if cfg!(feature = "redefi-runtime") {
				""
			} else {
				" by Redefi"
			}
		)
		.as_str(),
	)
	.with_id(format!("{}_local", default_runtime::VERSION.spec_name).as_str())
	.with_chain_type(ChainType::Local)
	.with_properties(properties)
	.with_genesis_config_patch(testnet_genesis!(
		default_runtime,
		// Sudo account
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		[
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_from_seed::<AuraId>("Bob"),
			),
		],
		[(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_from_seed::<AuraId>("Alice"),
		)],
		// Pre-funded accounts
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		],
		PARA_ID
	))
	.build()
}
