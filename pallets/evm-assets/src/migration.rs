use sp_io::storage::get;

use crate::*;
pub(crate) const SUDO_STORAGE_KEY: [u8; 32] =
	hex_literal::hex!("5c0d1176a568c1f92944340dbfed9e9c530ebca703c85910e7164cb7d1c9e47b");

pub(crate) fn init_assets_with<T: Config>(accounts: &[T::AccountId]) {
	init_bax_with::<T>(accounts);
	init_gbp_with::<T>(accounts);
	<SupportedAssets<T>>::set(Assets::BAX | Assets::GBP);
}

pub(crate) fn init_bax_with<T: Config>(accounts: &[T::AccountId]) {
	let bx_asset = AssetDetails::<Balance, Address> {
		supply: BALANCE * NATIVE * accounts.len() as Balance,
		..Default::default()
	};
	<Asset<T>>::insert(BAX_ID, bx_asset);

	accounts
		.iter()
		.map(|acc| *T::CrossAccountId::from_sub(acc.clone()).as_eth())
		.for_each(|adr| {
			<Balances<T>>::insert(BAX_ID, adr, BALANCE * NATIVE);
		});

	let bx_meta = AssetMetadata::<BoundedVec<u8, T::StringLimit>> {
		name: "Relaychain native token"
			.as_bytes()
			.to_vec()
			.try_into()
			.unwrap(),
		symbol: "BAX".as_bytes().to_vec().try_into().unwrap(),
		decimals: 18,
		is_frozen: false,
	};

	<Metadata<T>>::insert(BAX_ID, bx_meta);
}

pub(crate) fn init_gbp_with<T: Config>(accounts: &[T::AccountId]) {
	let gbp_asset = AssetDetails::<Balance, Address> {
		supply: BALANCE * CURRENCY * accounts.len() as Balance,
		..Default::default()
	};
	<Asset<T>>::insert(GBP_ID, gbp_asset);

	accounts
		.iter()
		.map(|acc| *T::CrossAccountId::from_sub(acc.clone()).as_eth())
		.for_each(|adr| {
			<Balances<T>>::insert(GBP_ID, adr, BALANCE * CURRENCY);
		});

	let gbp_meta = AssetMetadata::<BoundedVec<u8, T::StringLimit>> {
		name: "Onchain GBP".as_bytes().to_vec().try_into().unwrap(),
		symbol: "GBP".as_bytes().to_vec().try_into().unwrap(),
		decimals: 6,
		is_frozen: false,
	};

	<Metadata<T>>::insert(GBP_ID, gbp_meta);
}

pub struct InitializationWithSudoAsHolder<T: Config>(PhantomData<T>);

impl<T: Config> OnRuntimeUpgrade for InitializationWithSudoAsHolder<T>
where
	T::AccountId: for<'a> TryFrom<&'a [u8]>,
{
	fn on_runtime_upgrade() -> Weight {
		let sudoer_raw_key = get(&SUDO_STORAGE_KEY);
		if sudoer_raw_key.is_none() {
			log::error!(
			target: LOG_TARGET,
					"Sudo key not found - migration incomplete"
				);
			return T::DbWeight::get().reads(1);
		}
		let sudoer_raw_key = sudoer_raw_key.unwrap();
		let sudoer_key = T::AccountId::try_from(sudoer_raw_key.as_ref());

		if sudoer_key.is_err() {
			log::error!(
			target: LOG_TARGET,
					"Failed to deserialize sudo key. Value: {:?}. Migration Failed",
					sudoer_raw_key
				);
			return T::DbWeight::get().reads(1);
		}

		let sudoer_key = sudoer_key.ok().unwrap();
		let accs = [sudoer_key];

		let mut supported_assets = <SupportedAssets<T>>::get();

		if !supported_assets.contains(Assets::BAX) {
			init_bax_with::<T>(&accs);
			supported_assets |= Assets::BAX;
		}

		if !supported_assets.contains(Assets::GBP) {
			init_gbp_with::<T>(&accs);
			supported_assets |= Assets::GBP;
		}

		init_assets_with::<T>(&accs);

		<SupportedAssets<T>>::set(supported_assets);

		T::DbWeight::get().reads_writes(2, 8)
	}
}
