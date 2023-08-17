// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! # CDP Treasury Module
//!
//! ## Overview
//!
//! CDP Treasury manages the accumulated interest and bad debts generated by
//! CDPs, and handle excessive surplus or debits timely in order to keep the
//! system healthy with low risk. It's the only entry for issuing/burning stable
//! coin for whole system.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::needless_range_loop)]

use frame_support::{log, pallet_prelude::*, transactional, PalletId};
use frame_system::pallet_prelude::*;
use nutsfinance_stable_asset::traits::StableAsset;
use nutsfinance_stable_asset::RedeemProportionResult;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use primitives::{Balance, CurrencyId};
use sp_runtime::{
	traits::{AccountIdConversion, One, Zero},
	ArithmeticError, DispatchError, DispatchResult, FixedPointNumber,
};
use sp_std::prelude::*;
use support::{AuctionManager, CDPTreasury, CDPTreasuryExtended, DEXManager, Ratio, Swap, SwapLimit};

mod mock;
mod tests;
pub mod weights;

pub use module::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The origin which may update parameters and handle
		/// surplus/collateral.
		type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// The Currency for managing assets related to CDP
		type Currency: MultiCurrencyExtended<Self::AccountId, CurrencyId = CurrencyId, Balance = Balance>;

		/// Stablecoin currency id
		#[pallet::constant]
		type GetStableCurrencyId: Get<CurrencyId>;

		/// Auction manager creates auction to handle system surplus and debit
		type AuctionManagerHandler: AuctionManager<Self::AccountId, CurrencyId = CurrencyId, Balance = Balance>;

		/// Dex manager
		type DEX: DEXManager<Self::AccountId, Balance, CurrencyId>;

		/// Swap
		type Swap: Swap<Self::AccountId, Balance, CurrencyId>;

		type StableAsset: StableAsset<
			AssetId = CurrencyId,
			AtLeast64BitUnsigned = Balance,
			Balance = Balance,
			AccountId = Self::AccountId,
			BlockNumber = BlockNumberFor<Self>,
		>;

		/// The cap of lots number when create collateral auction on a
		/// liquidation or to create debit/surplus auction on block end.
		/// If set to 0, does not work.
		#[pallet::constant]
		type MaxAuctionsCount: Get<u32>;

		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;

		/// The CDP treasury's module id, keep surplus and collateral assets
		/// from liquidation.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The collateral amount of CDP treasury is not enough
		CollateralNotEnough,
		/// The surplus pool of CDP treasury is not enough
		SurplusPoolNotEnough,
		/// The debit pool of CDP treasury is not enough
		DebitPoolNotEnough,
		/// Cannot use collateral to swap stable
		CannotSwap,
		/// The currency id is not DexShare type
		NotDexShare,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		/// The expected amount size for per lot collateral auction of specific collateral type
		/// updated.
		ExpectedCollateralAuctionSizeUpdated {
			collateral_type: CurrencyId,
			new_size: Balance,
		},
		/// The buffer amount of debit pool that will not be offset by suplus pool updated.
		DebitOffsetBufferUpdated { amount: Balance },
	}

	/// The expected amount size for per lot collateral auction of specific
	/// collateral type.
	///
	/// ExpectedCollateralAuctionSize: map CurrencyId => Balance
	#[pallet::storage]
	#[pallet::getter(fn expected_collateral_auction_size)]
	pub type ExpectedCollateralAuctionSize<T: Config> = StorageMap<_, Twox64Concat, CurrencyId, Balance, ValueQuery>;

	/// Current total debit value of system. It's not same as debit in CDP
	/// engine, it is the bad debt of the system.
	///
	/// DebitPool: Balance
	#[pallet::storage]
	#[pallet::getter(fn debit_pool)]
	pub type DebitPool<T: Config> = StorageValue<_, Balance, ValueQuery>;

	/// The buffer amount of debit pool that will not be offset by surplus pool.
	///
	/// DebitOffsetBuffer: Balance
	#[pallet::storage]
	#[pallet::getter(fn debit_offset_buffer)]
	pub type DebitOffsetBuffer<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T> {
		pub expected_collateral_auction_size: Vec<(CurrencyId, Balance)>,
		pub _phantom: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			self.expected_collateral_auction_size
				.iter()
				.for_each(|(currency_id, size)| {
					ExpectedCollateralAuctionSize::<T>::insert(currency_id, size);
				});
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Handle excessive surplus or debits of system when block end
		fn on_finalize(_now: BlockNumberFor<T>) {
			// offset the same amount between debit pool and surplus pool
			Self::offset_surplus_and_debit();
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::extract_surplus_to_treasury())]
		pub fn extract_surplus_to_treasury(origin: OriginFor<T>, #[pallet::compact] amount: Balance) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			T::Currency::transfer(
				T::GetStableCurrencyId::get(),
				&Self::account_id(),
				&T::TreasuryAccount::get(),
				amount,
			)?;
			Ok(())
		}

		/// Auction the collateral not occupied by the auction.
		///
		/// The dispatch origin of this call must be `UpdateOrigin`.
		///
		/// - `currency_id`: collateral type
		/// - `amount`: collateral amount
		/// - `target`: target amount
		/// - `splited`: split collateral to multiple auction according to the config size
		#[pallet::call_index(1)]
		#[pallet::weight(
			if *splited {
				T::WeightInfo::auction_collateral(T::MaxAuctionsCount::get())
			} else {
				T::WeightInfo::auction_collateral(1)
			}
		)]
		pub fn auction_collateral(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			#[pallet::compact] amount: Balance,
			#[pallet::compact] target: Balance,
			splited: bool,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;
			let created_auctions = <Self as CDPTreasuryExtended<T::AccountId>>::create_collateral_auctions(
				currency_id,
				amount,
				target,
				Self::account_id(),
				splited,
			)?;
			Ok(Some(T::WeightInfo::auction_collateral(created_auctions)).into())
		}

		/// Swap the collateral not occupied by the auction to stable.
		///
		/// The dispatch origin of this call must be `UpdateOrigin`.
		///
		/// - `currency_id`: collateral type
		/// - `swap_limit`: target amount
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::exchange_collateral_to_stable())]
		pub fn exchange_collateral_to_stable(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			swap_limit: SwapLimit<Balance>,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			// the supply collateral must not be occupied by the auction.
			Self::swap_collateral_to_stable(currency_id, swap_limit, false)?;
			Ok(())
		}

		/// Update parameters related to collateral auction under specific
		/// collateral type
		///
		/// The dispatch origin of this call must be `UpdateOrigin`.
		///
		/// - `currency_id`: collateral type
		/// - `amount`: expected size of per lot collateral auction
		#[pallet::call_index(3)]
		#[pallet::weight((T::WeightInfo::set_expected_collateral_auction_size(), DispatchClass::Operational))]
		pub fn set_expected_collateral_auction_size(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			#[pallet::compact] size: Balance,
		) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			ExpectedCollateralAuctionSize::<T>::insert(currency_id, size);
			Self::deposit_event(Event::ExpectedCollateralAuctionSizeUpdated {
				collateral_type: currency_id,
				new_size: size,
			});
			Ok(())
		}

		/// Update the debit offset buffer
		///
		/// The dispatch origin of this call must be `UpdateOrigin`.
		///
		/// - `amount`: the buffer amount of debit pool
		#[pallet::call_index(4)]
		#[pallet::weight((T::WeightInfo::set_expected_collateral_auction_size(), DispatchClass::Operational))]
		pub fn set_debit_offset_buffer(origin: OriginFor<T>, #[pallet::compact] amount: Balance) -> DispatchResult {
			T::UpdateOrigin::ensure_origin(origin)?;
			DebitOffsetBuffer::<T>::mutate(|v| {
				if *v != amount {
					*v = amount;
					Self::deposit_event(Event::DebitOffsetBufferUpdated { amount });
				}
			});
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Get account of cdp treasury module.
	pub fn account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}

	/// Get current total surplus of system.
	pub fn surplus_pool() -> Balance {
		T::Currency::free_balance(T::GetStableCurrencyId::get(), &Self::account_id())
	}

	/// Get total collateral amount of cdp treasury module.
	pub fn total_collaterals(currency_id: CurrencyId) -> Balance {
		T::Currency::free_balance(currency_id, &Self::account_id())
	}

	/// Get collateral amount not in auction
	pub fn total_collaterals_not_in_auction(currency_id: CurrencyId) -> Balance {
		T::Currency::free_balance(currency_id, &Self::account_id())
			.saturating_sub(T::AuctionManagerHandler::get_total_collateral_in_auction(currency_id))
	}

	fn offset_surplus_and_debit() {
		// The part of the debit pool that exceeds the debit offset buffer can be offset by the surplus
		let offset_amount = sp_std::cmp::min(
			Self::debit_pool().saturating_sub(Self::debit_offset_buffer()),
			Self::surplus_pool(),
		);

		// Burn the amount that is equal to offset amount of stable currency.
		if !offset_amount.is_zero() {
			let res = Self::burn_debit(&Self::account_id(), offset_amount);
			match res {
				Ok(_) => {
					DebitPool::<T>::mutate(|debit| {
						*debit = debit
							.checked_sub(offset_amount)
							.expect("offset = min(debit, surplus); qed")
					});
				}
				Err(e) => {
					log::warn!(
						target: "cdp-treasury",
						"get_swap_supply_amount: Attempt to burn surplus {:?} failed: {:?}, this is unexpected but should be safe",
						offset_amount, e
					);
				}
			}
		}
	}
}

impl<T: Config> CDPTreasury<T::AccountId> for Pallet<T> {
	type Balance = Balance;
	type CurrencyId = CurrencyId;

	fn get_surplus_pool() -> Self::Balance {
		Self::surplus_pool()
	}

	fn get_debit_pool() -> Self::Balance {
		Self::debit_pool()
	}

	fn get_total_collaterals(id: Self::CurrencyId) -> Self::Balance {
		Self::total_collaterals(id)
	}

	fn get_debit_proportion(amount: Self::Balance) -> Ratio {
		let stable_total_supply = T::Currency::total_issuance(T::GetStableCurrencyId::get());
		Ratio::checked_from_rational(amount, stable_total_supply).unwrap_or_default()
	}

	fn on_system_debit(amount: Self::Balance) -> DispatchResult {
		DebitPool::<T>::try_mutate(|debit_pool| -> DispatchResult {
			*debit_pool = debit_pool.checked_add(amount).ok_or(ArithmeticError::Overflow)?;
			Ok(())
		})
	}

	fn on_system_surplus(amount: Self::Balance) -> DispatchResult {
		Self::issue_debit(&Self::account_id(), amount, true)
	}

	/// This should be the only function in the system that issues stable coin
	fn issue_debit(who: &T::AccountId, debit: Self::Balance, backed: bool) -> DispatchResult {
		// increase system debit if the debit is unbacked
		if !backed {
			Self::on_system_debit(debit)?;
		}
		T::Currency::deposit(T::GetStableCurrencyId::get(), who, debit)?;

		Ok(())
	}

	/// This should be the only function in the system that burns stable coin
	fn burn_debit(who: &T::AccountId, debit: Self::Balance) -> DispatchResult {
		T::Currency::withdraw(T::GetStableCurrencyId::get(), who, debit)
	}

	fn deposit_surplus(from: &T::AccountId, surplus: Self::Balance) -> DispatchResult {
		T::Currency::transfer(T::GetStableCurrencyId::get(), from, &Self::account_id(), surplus)
	}

	fn withdraw_surplus(to: &T::AccountId, surplus: Self::Balance) -> DispatchResult {
		T::Currency::transfer(T::GetStableCurrencyId::get(), &Self::account_id(), to, surplus)
	}

	fn deposit_collateral(from: &T::AccountId, currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult {
		T::Currency::transfer(currency_id, from, &Self::account_id(), amount)
	}

	fn withdraw_collateral(to: &T::AccountId, currency_id: Self::CurrencyId, amount: Self::Balance) -> DispatchResult {
		T::Currency::transfer(currency_id, &Self::account_id(), to, amount)
	}
}

impl<T: Config> CDPTreasuryExtended<T::AccountId> for Pallet<T> {
	#[transactional]
	fn swap_collateral_to_stable(
		currency_id: CurrencyId,
		limit: SwapLimit<Balance>,
		collateral_in_auction: bool,
	) -> sp_std::result::Result<(Balance, Balance), DispatchError> {
		let supply_limit = match limit {
			SwapLimit::ExactSupply(supply_amount, _) => supply_amount,
			SwapLimit::ExactTarget(max_supply_amount, _) => max_supply_amount,
		};
		let target_limit = match limit {
			SwapLimit::ExactSupply(_, minimum_target_amount) => minimum_target_amount,
			SwapLimit::ExactTarget(_, exact_target_amount) => exact_target_amount,
		};

		if collateral_in_auction {
			ensure!(
				Self::total_collaterals(currency_id) >= supply_limit
					&& T::AuctionManagerHandler::get_total_collateral_in_auction(currency_id) >= supply_limit,
				Error::<T>::CollateralNotEnough,
			);
		} else {
			ensure!(
				Self::total_collaterals_not_in_auction(currency_id) >= supply_limit,
				Error::<T>::CollateralNotEnough,
			);
		}

		match currency_id {
			CurrencyId::StableAssetPoolToken(stable_asset_id) => {
				let pool_info = T::StableAsset::pool(stable_asset_id).ok_or(Error::<T>::CannotSwap)?;
				let updated_balance_info =
					T::StableAsset::get_balance_update_amount(&pool_info).ok_or(Error::<T>::CannotSwap)?;
				let yield_info =
					T::StableAsset::get_collect_yield_amount(&updated_balance_info).ok_or(Error::<T>::CannotSwap)?;
				ensure!(
					yield_info.total_supply >= pool_info.total_supply,
					Error::<T>::CannotSwap,
				);
				let RedeemProportionResult { amounts, .. } =
					T::StableAsset::get_redeem_proportion_amount(&yield_info, supply_limit)
						.ok_or(Error::<T>::CannotSwap)?;

				// redeem stable asset to a basket of assets
				T::StableAsset::redeem_proportion(
					&Self::account_id(),
					stable_asset_id,
					supply_limit,
					vec![0; amounts.len()],
				)?;

				let mut supply_sum: Balance = Zero::zero();
				let mut target_sum: Balance = Zero::zero();

				for i in 0..amounts.len() {
					let redemption_currency = pool_info.assets[i];
					let amount = amounts[i];

					if !amount.is_zero() {
						let swap_limit = SwapLimit::ExactSupply(amount, 0);
						let response = T::Swap::swap(
							&Self::account_id(),
							redemption_currency,
							T::GetStableCurrencyId::get(),
							swap_limit,
						)?;
						supply_sum = supply_sum.checked_add(response.0).ok_or(ArithmeticError::Overflow)?;
						target_sum = target_sum.checked_add(response.1).ok_or(ArithmeticError::Overflow)?;
					}
				}

				ensure!(target_sum >= target_limit, Error::<T>::CannotSwap);
				Ok((supply_sum, target_sum))
			}
			_ => T::Swap::swap(&Self::account_id(), currency_id, T::GetStableCurrencyId::get(), limit),
		}
	}

	fn create_collateral_auctions(
		currency_id: CurrencyId,
		amount: Balance,
		target: Balance,
		refund_receiver: T::AccountId,
		splited: bool,
	) -> Result<u32, DispatchError> {
		ensure!(
			Self::total_collaterals_not_in_auction(currency_id) >= amount,
			Error::<T>::CollateralNotEnough,
		);

		let mut unhandled_collateral_amount = amount;
		let mut unhandled_target = target;
		let expected_collateral_auction_size = Self::expected_collateral_auction_size(currency_id);
		let max_auctions_count: Balance = T::MaxAuctionsCount::get().into();
		let lots_count = if !splited
			|| max_auctions_count.is_zero()
			|| expected_collateral_auction_size.is_zero()
			|| amount <= expected_collateral_auction_size
		{
			One::one()
		} else {
			let mut count = amount
				.checked_div(expected_collateral_auction_size)
				.expect("collateral auction maximum size is not zero; qed");

			let remainder = amount
				.checked_rem(expected_collateral_auction_size)
				.expect("collateral auction maximum size is not zero; qed");
			if !remainder.is_zero() {
				count = count.saturating_add(One::one());
			}
			sp_std::cmp::min(count, max_auctions_count)
		};
		let average_amount_per_lot = amount.checked_div(lots_count).expect("lots count is at least 1; qed");
		let average_target_per_lot = target.checked_div(lots_count).expect("lots count is at least 1; qed");
		let mut created_lots: Balance = Zero::zero();

		while !unhandled_collateral_amount.is_zero() {
			created_lots = created_lots.saturating_add(One::one());
			let (lot_collateral_amount, lot_target) = if created_lots == lots_count {
				// the last lot may be have some remnant than average
				(unhandled_collateral_amount, unhandled_target)
			} else {
				(average_amount_per_lot, average_target_per_lot)
			};

			T::AuctionManagerHandler::new_collateral_auction(
				&refund_receiver,
				currency_id,
				lot_collateral_amount,
				lot_target,
			)?;

			unhandled_collateral_amount = unhandled_collateral_amount.saturating_sub(lot_collateral_amount);
			unhandled_target = unhandled_target.saturating_sub(lot_target);
		}
		let created_auctions: u32 = created_lots.try_into().map_err(|_| ArithmeticError::Overflow)?;
		Ok(created_auctions)
	}

	fn remove_liquidity_for_lp_collateral(
		lp_currency_id: CurrencyId,
		amount: Balance,
	) -> sp_std::result::Result<(Balance, Balance), DispatchError> {
		let (currency_id_0, currency_id_1) = lp_currency_id
			.split_dex_share_currency_id()
			.ok_or(Error::<T>::NotDexShare)?;
		T::DEX::remove_liquidity(
			&Self::account_id(),
			currency_id_0,
			currency_id_1,
			amount,
			Zero::zero(),
			Zero::zero(),
			false,
		)
	}

	fn max_auction() -> u32 {
		T::MaxAuctionsCount::get()
	}
}

pub struct InitializeDebitOffsetBuffer<T, GetBufferSize>(
	sp_std::marker::PhantomData<T>,
	sp_std::marker::PhantomData<GetBufferSize>,
);
impl<T: Config, GetBufferSize: Get<Balance>> frame_support::traits::OnRuntimeUpgrade
	for InitializeDebitOffsetBuffer<T, GetBufferSize>
{
	fn on_runtime_upgrade() -> Weight {
		let amount = GetBufferSize::get();
		DebitOffsetBuffer::<T>::mutate(|v| {
			if *v != amount {
				*v = amount;
				Pallet::<T>::deposit_event(Event::DebitOffsetBufferUpdated { amount });
			}
		});

		Weight::from_parts(0, 0)
	}
}
