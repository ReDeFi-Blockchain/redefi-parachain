use staging_xcm::latest::Asset;
use staging_xcm_executor::traits::{prelude::Error as XcmError, MatchesFungibles};

use crate::*;

impl<T: Config> MatchesFungibles<AssetId, Balance> for Pallet<T> {
	fn matches_fungibles(a: &Asset) -> core::result::Result<(AssetId, Balance), XcmError> {
		let XcmAssetId(Location { parents, interior }) = &a.id;
		if *parents != 1 {
			return Err(XcmError::AssetNotHandled);
		}

		let Junctions::X1(junctions) = interior else {
			return Err(XcmError::AssetNotHandled);
		};

		let [Junction::AccountKey20 {
			network: _,
			key: contract_addr,
		}] = junctions.as_ref()
		else {
			return Err(XcmError::AssetNotHandled);
		};

		let contract_addr = Address::from_slice(contract_addr);
		let asset = Self::address_to_asset_id(&contract_addr).ok_or(XcmError::AssetNotHandled)?;

		if Self::asset_exists(asset) {
			let Fungibility::Fungible(amount) = a.fun else {
				return Err(XcmError::AmountToBalanceConversionFailed);
			};
			return Ok((asset, amount));
		}

		Err(XcmError::AssetNotHandled)
	}
}

pub struct EthereumOriginToLocation<RuntimeOrigin, Network>(PhantomData<(RuntimeOrigin, Network)>)
where
	RuntimeOrigin: Into<Result<EthereumOrigin, RuntimeOrigin>>,
	Network: Get<Option<NetworkId>>;

impl<RuntimeOrigin, Network> TryConvert<RuntimeOrigin, Location>
	for EthereumOriginToLocation<RuntimeOrigin, Network>
where
	RuntimeOrigin: Into<Result<EthereumOrigin, RuntimeOrigin>>,
	Network: Get<Option<NetworkId>>,
{
	fn try_convert(o: RuntimeOrigin) -> Result<Location, RuntimeOrigin> {
		o.into().map(|eo| match eo {
			EthereumOrigin::EthereumTransaction(address) => Junction::AccountKey20 {
				network: Network::get(),
				key: address.into(),
			}
			.into(),
		})
	}
}
