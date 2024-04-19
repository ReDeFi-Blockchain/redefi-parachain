use evm_coder::{abi::AbiType, generate_stubgen, solidity_interface};
use pallet_evm::{OnMethodCall, PrecompileHandle, PrecompileResult};
use pallet_evm_coder_substrate::{
	dispatch_to_evm,
	execution::{PreDispatch, Result},
	frontier_contract,
};

use super::*;

frontier_contract! {
	macro_rules! FungibleAssetsHandle_result {...}
	impl<@INSTANCE(IN), T: Config<IN>> Contract for FungibleAssetsHandle<T, IN> {...}
}

#[derive(ToLog)]
pub enum ERC20Events {
	Transfer {
		#[indexed]
		from: Address,
		#[indexed]
		to: Address,
		value: U256,
	},
	Approval {
		#[indexed]
		owner: Address,
		#[indexed]
		spender: Address,
		value: U256,
	},
}

#[solidity_interface(name = ERC20, events(ERC20Events), enum(derive(PreDispatch)), enum_attr(weight))]
impl<T: Config<I>, I: 'static> FungibleAssetsHandle<T, I> {
	fn allowance(&self, owner: Address, spender: Address) -> Result<U256> {
		self.consume_store_reads(1)?;
		let owner = T::CrossAccountId::from_eth(owner);
		let spender = T::CrossAccountId::from_eth(spender);

		Ok(
			<PalletAssets<T, I>>::allowance(*self.asset_id(), owner.as_sub(), spender.as_sub())
				.into(),
		)
	}
	#[weight(<SelfWeightOf<T>>::approve_transfer())]
	fn approve(&mut self, caller: Caller, spender: Address, amount: U256) -> Result<bool> {
		self.consume_store_writes(1)?;
		let owner = T::CrossAccountId::from_eth(caller);
		let spender = T::CrossAccountId::from_eth(spender);
		let amount = amount.try_into().map_err(|_| "amount overflow")?;
		<PalletAssets<T, I>>::approve(*self.asset_id(), owner.as_sub(), spender.as_sub(), amount)
			.map_err(dispatch_to_evm::<T>)?;

		Ok(true)
	}

	fn balance_of(&self, owner: Address) -> Result<U256> {
		self.consume_store_reads(1)?;
		let owner = T::CrossAccountId::from_eth(owner);
		let balance = <PalletAssets<T, I>>::balance(*self.asset_id(), owner.as_sub());
		Ok(balance.into())
	}

	fn decimals(&self) -> Result<u8> {
		self.consume_store_reads(1)?;
		Ok(<PalletAssets<T, I>>::decimals(*self.asset_id()))
	}

	fn name(&self) -> Result<String> {
		self.consume_store_reads(1)?;
		let raw = <PalletAssets<T, I> as MetadataInspect<_>>::name(*self.asset_id());
		Ok(String::from_utf8_lossy(&raw[..]).into())
	}

	fn symbol(&self) -> Result<String> {
		self.consume_store_reads(1)?;
		let raw = <PalletAssets<T, I> as MetadataInspect<_>>::symbol(*self.asset_id());
		Ok(String::from_utf8_lossy(&raw[..]).into())
	}

	fn total_supply(&self) -> Result<U256> {
		self.consume_store_reads(1)?;
		Ok(<PalletAssets<T, I>>::total_issuance(*self.asset_id()).into())
	}

	#[weight(<SelfWeightOf<T>>::transfer())]
	fn transfer(&mut self, caller: Caller, to: Address, amount: U256) -> Result<bool> {
		let caller = T::CrossAccountId::from_eth(caller);
		let to = T::CrossAccountId::from_eth(to);
		let amount = amount.try_into().map_err(|_| "amount overflow")?;

		<PalletAssets<T, I> as Mutate<_>>::transfer(
			*self.asset_id(),
			caller.as_sub(),
			to.as_sub(),
			amount,
			Preservation::Expendable,
		)
		.map_err(dispatch_to_evm::<T>)?;
		Ok(true)
	}

	#[weight(<SelfWeightOf<T>>::transfer_approved())]
	fn transfer_from(
		&mut self,
		caller: Caller,
		from: Address,
		to: Address,
		amount: U256,
	) -> Result<bool> {
		let caller = T::CrossAccountId::from_eth(caller);
		let from = T::CrossAccountId::from_eth(from);
		let to = T::CrossAccountId::from_eth(to);
		let amount = amount.try_into().map_err(|_| "amount overflow")?;

		<PalletAssets<T, I>>::transfer_from(
			*self.asset_id(),
			from.as_sub(),
			caller.as_sub(),
			to.as_sub(),
			amount,
		)
		.map_err(dispatch_to_evm::<T>)?;

		Ok(true)
	}
}

/// Implements [`OnMethodCall`], which delegates call to [`NativeFungibleHandle`]
pub struct AdapterOnMethodCall<T: Config<I>, I: 'static = ()>(PhantomData<*const (T, I)>);
impl<T: Config<I> + Config, I: 'static> OnMethodCall<T> for AdapterOnMethodCall<T, I>
where
	T::AccountId: AsRef<[u8; 32]>,
{
	fn is_reserved(contract: &H160) -> bool {
		<Pallet<T, I>>::address_to_asset_id(contract).is_some()
	}

	fn is_used(contract: &H160) -> bool {
		<Pallet<T, I>>::address_to_asset_id(contract)
			.map(<PalletAssets<T, I>>::asset_exists)
			.unwrap_or_default()
	}

	fn call(handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		let asset_id = <Pallet<T, I>>::address_to_asset_id(&handle.code_address());

		let adapter_handle =
			<FungibleAssetsHandle<T, I>>::new_with_gas_limit(asset_id?, handle.remaining_gas());
		pallet_evm_coder_substrate::call(handle, adapter_handle)
	}

	fn get_code(contract: &H160) -> Option<Vec<u8>> {
		Self::is_used(contract).then(|| include_bytes!("./stubs/NativeFungible.raw").to_vec())
	}
}

#[solidity_interface(
	name = NativeFungibleAssets,
	is(ERC20),
	enum(derive(PreDispatch))
)]
impl<T: Config<I>, I> FungibleAssetsHandle<T, I>
where
	T::AccountId: From<[u8; 32]> + AsRef<[u8; 32]>,
	I: 'static,
{
}

generate_stubgen!(gen_impl, NativeFungibleAssetsCall<()>, true);
generate_stubgen!(gen_iface, NativeFungibleAssetsCall<()>, false);
