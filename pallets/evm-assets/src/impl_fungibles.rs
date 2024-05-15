use frame_support::traits::{fungibles::Inspect, tokens::Precision::*};
use sp_runtime::TokenError;

use crate::*;

impl<T: Config> fungibles::Inspect<Address> for Pallet<T> {
	type AssetId = AssetId;

	type Balance = Balance;

	fn total_issuance(asset: Self::AssetId) -> Self::Balance {
		<Asset<T>>::get(asset)
			.map(|a| a.supply)
			.unwrap_or_else(Zero::zero)
	}

	fn minimum_balance(_asset: Self::AssetId) -> Self::Balance {
		Zero::zero()
	}

	fn total_balance(asset: Self::AssetId, who: &Address) -> Self::Balance {
		<Pallet<T>>::balance(&asset, who)
	}

	fn balance(asset: Self::AssetId, who: &Address) -> Self::Balance {
		<Pallet<T>>::balance(&asset, who)
	}

	fn reducible_balance(
		asset: Self::AssetId,
		who: &Address,
		_preservation: frame_support::traits::tokens::Preservation,
		_force: frame_support::traits::tokens::Fortitude,
	) -> Self::Balance {
		<Pallet<T>>::balance(&asset, who)
	}

	fn can_deposit(
		asset: Self::AssetId,
		who: &Address,
		amount: Self::Balance,
		_provenance: frame_support::traits::tokens::Provenance,
	) -> frame_support::traits::tokens::DepositConsequence {
		<Balances<T>>::get(asset, who)
			.checked_add(amount)
			.map(|_| DepositConsequence::Success)
			.unwrap_or(DepositConsequence::Overflow)
	}

	fn can_withdraw(
		asset: Self::AssetId,
		who: &Address,
		amount: Self::Balance,
	) -> frame_support::traits::tokens::WithdrawConsequence<Self::Balance> {
		<Balances<T>>::get(asset, who)
			.checked_sub(amount)
			.map(|_| WithdrawConsequence::Success)
			.unwrap_or(WithdrawConsequence::Underflow)
	}

	fn asset_exists(asset: Self::AssetId) -> bool {
		<Pallet<T>>::asset_exists(asset)
	}
}

impl<T: Config> fungibles::Unbalanced<Address> for Pallet<T> {
	fn handle_dust(_dust: fungibles::Dust<Address, Self>) {}

	fn write_balance(
		_asset: Self::AssetId,
		_who: &Address,
		_amount: Self::Balance,
	) -> Result<Option<Self::Balance>, DispatchError> {
		Err(DispatchError::Unavailable)
	}

	fn set_total_issuance(asset: Self::AssetId, amount: Self::Balance) {
		<Asset<T>>::mutate_exists(asset, |a| {
			if let Some(asset_details) = a {
				asset_details.supply = amount
			}
		})
	}

	fn decrease_balance(
		asset: Self::AssetId,
		who: &Address,
		mut amount: Self::Balance,
		precision: frame_support::traits::tokens::Precision,
		preservation: frame_support::traits::tokens::Preservation,
		force: frame_support::traits::tokens::Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		let old_balance = Self::balance(&asset, who);
		ensure!(old_balance > 0, TokenError::FundsUnavailable);
		let reducible = Self::reducible_balance(asset, who, preservation, force);
		match precision {
			BestEffort => amount = amount.min(reducible),
			Exact => ensure!(reducible >= amount, TokenError::FundsUnavailable),
		}
		let new_balance = old_balance
			.checked_sub(amount)
			.ok_or(sp_runtime::TokenError::FundsUnavailable)?;
		<Balances<T>>::set(asset, who, new_balance);

		Ok(amount)
	}

	fn increase_balance(
		asset: Self::AssetId,
		who: &Address,
		amount: Self::Balance,
		precision: frame_support::traits::tokens::Precision,
	) -> Result<Self::Balance, DispatchError> {
		let old_balance = Self::balance(&asset, who);
		let new_balance = if let BestEffort = precision {
			old_balance.saturating_add(amount)
		} else {
			old_balance
				.checked_add(amount)
				.ok_or(ArithmeticError::Overflow)?
		};
		if new_balance < Self::minimum_balance(asset) {
			// Attempt to increase from 0 to below minimum -> stays at zero.
			if let BestEffort = precision {
				Ok(Self::Balance::default())
			} else {
				Err(sp_runtime::TokenError::BelowMinimum.into())
			}
		} else if new_balance == old_balance {
			Ok(Self::Balance::default())
		} else {
			<Balances<T>>::set(asset, who, new_balance);
			Ok(new_balance.saturating_sub(old_balance))
		}
	}
}

impl<T: Config> fungibles::Mutate<Address> for Pallet<T> {
	fn mint_into(
		asset: Self::AssetId,
		who: &Address,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T>>::mint(&asset, who, amount).map(|_| amount)
	}

	fn burn_from(
		asset: Self::AssetId,
		who: &Address,
		amount: Self::Balance,
		_precision: frame_support::traits::tokens::Precision,
		_force: frame_support::traits::tokens::Fortitude,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T>>::burn(&asset, who, amount).map(|_| amount)
	}

	fn shelve(
		asset: Self::AssetId,
		who: &Address,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T>>::burn(&asset, who, amount).map(|_| amount)
	}

	fn restore(
		asset: Self::AssetId,
		who: &Address,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T>>::mint(&asset, who, amount).map(|_| amount)
	}

	fn transfer(
		asset: Self::AssetId,
		source: &Address,
		dest: &Address,
		amount: Self::Balance,
		_preservation: frame_support::traits::tokens::Preservation,
	) -> Result<Self::Balance, DispatchError> {
		<Pallet<T>>::transfer(&asset, source, dest, amount).map(|_| amount)
	}
}
