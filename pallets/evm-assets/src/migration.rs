use crate::*;

pub(crate) const CURRENCY: Balance = 1_000_000;
pub(crate) const NATIVE: Balance = 1_000_000_000_000_000_000;
pub(crate) const BALANCE: Balance = 10_000;

pub(crate) const GBP_ID: AssetId = 0xBABB0000_00000000_00000000_00000010;
pub(crate) const BAXX_ID: AssetId = 0x00000000_00000000_00000000_0000BABB;

pub(crate) fn init_assets_with<T: Config>(accounts: &[T::AccountId]) {
	let gbp_asset = AssetDetails::<Balance, Address> {
		supply: BALANCE * CURRENCY * accounts.len() as Balance,
		..Default::default()
	};
	<Asset<T>>::insert(GBP_ID, gbp_asset);

	let bx_asset = AssetDetails::<Balance, Address> {
		supply: BALANCE * NATIVE * accounts.len() as Balance,
		..Default::default()
	};
	<Asset<T>>::insert(BAXX_ID, bx_asset);

	accounts
		.iter()
		.map(|acc| *T::CrossAccountId::from_sub(acc.clone()).as_eth())
		.for_each(|adr| {
			<Balances<T>>::insert(GBP_ID, adr, BALANCE * CURRENCY);
			<Balances<T>>::insert(BAXX_ID, adr, BALANCE * NATIVE);
		});
	let gbp_meta = AssetMetadata::<BoundedVec<u8, T::StringLimit>> {
		name: "Great Britain Pound"
			.as_bytes()
			.to_vec()
			.try_into()
			.unwrap(),
		symbol: "GBP".as_bytes().to_vec().try_into().unwrap(),
		decimals: 6,
		is_frozen: false,
	};

	<Metadata<T>>::insert(GBP_ID, gbp_meta);

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

	<Metadata<T>>::insert(BAXX_ID, bx_meta);
}
