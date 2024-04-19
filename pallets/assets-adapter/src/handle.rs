use crate::*;
pub type AssetIdOf<T, I = ()> = <T as pallet_assets::Config<I>>::AssetId;
/// Handle for native token as an ERC20 collection
pub struct FungibleAssetsHandle<T: Config<I>, I: 'static = ()> {
	asset_id: AssetIdOf<T, I>,
	recorder: SubstrateRecorder<T>,
}

impl<T: Config<I>, I: 'static> FungibleAssetsHandle<T, I> {
	/// Creates a handle
	pub fn new(asset_id: AssetIdOf<T, I>) -> FungibleAssetsHandle<T, I> {
		Self::new_with_gas_limit(asset_id, u64::MAX)
	}

	/// Creates a handle
	pub fn new_with_gas_limit(
		asset_id: AssetIdOf<T, I>,
		gas_limit: u64,
	) -> FungibleAssetsHandle<T, I> {
		Self {
			asset_id,
			recorder: SubstrateRecorder::new(gas_limit),
		}
	}

	/// Returns `AssetId` reference
	pub fn asset_id(&self) -> &AssetIdOf<T, I> {
		&self.asset_id
	}
}

impl<T: Config<I>, I: 'static> Default for FungibleAssetsHandle<T, I> {
	fn default() -> Self {
		Self::new(Default::default())
	}
}

impl<T: Config<I>, I: 'static> WithRecorder<T> for FungibleAssetsHandle<T, I> {
	fn recorder(&self) -> &pallet_evm_coder_substrate::SubstrateRecorder<T> {
		&self.recorder
	}
	fn into_recorder(self) -> pallet_evm_coder_substrate::SubstrateRecorder<T> {
		self.recorder
	}
}

impl<T: Config<I>, I: 'static> Deref for FungibleAssetsHandle<T, I> {
	type Target = SubstrateRecorder<T>;

	fn deref(&self) -> &Self::Target {
		&self.recorder
	}
}
