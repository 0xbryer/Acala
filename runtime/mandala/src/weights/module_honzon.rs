// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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

//! Autogenerated weights for module_honzon
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ea4c8813bb44`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_honzon.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_honzon::WeightInfo for WeightInfo<T> {
	// Storage: Honzon Authorization (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	fn authorize() -> Weight {
		// Minimum execution time: 34_522 nanoseconds.
		Weight::from_ref_time(35_669_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Honzon Authorization (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	fn unauthorize() -> Weight {
		// Minimum execution time: 36_820 nanoseconds.
		Weight::from_ref_time(38_449_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Balances Reserves (r:1 w:0)
	// Storage: Honzon Authorization (r:0 w:1)
	/// The range of component `c` is `[0, 5]`.
	fn unauthorize_all(c: u32, ) -> Weight {
		// Minimum execution time: 19_799 nanoseconds.
		Weight::from_ref_time(27_464_984)
			// Standard Error: 147_904
			.saturating_add(Weight::from_ref_time(4_897_740).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	fn adjust_loan() -> Weight {
		// Minimum execution time: 113_255 nanoseconds.
		Weight::from_ref_time(117_181_000)
			.saturating_add(T::DbWeight::get().reads(15))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: Honzon Authorization (r:1 w:0)
	// Storage: Loans Positions (r:2 w:2)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Rewards SharesAndWithdrawnRewards (r:2 w:2)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	fn transfer_loan_from() -> Weight {
		// Minimum execution time: 95_145 nanoseconds.
		Weight::from_ref_time(97_661_000)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:3 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Tokens Accounts (r:10 w:10)
	// Storage: System Account (r:4 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: CdpTreasury DebitPool (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:3 w:1)
	// Storage: StableAsset Pools (r:2 w:1)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	fn close_loan_has_debit_by_dex() -> Weight {
		// Minimum execution time: 494_091 nanoseconds.
		Weight::from_ref_time(502_121_000)
			.saturating_add(T::DbWeight::get().reads(44))
			.saturating_add(T::DbWeight::get().writes(20))
	}
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:2 w:1)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:3 w:2)
	// Storage: StableAsset Pools (r:2 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	fn expand_position_collateral() -> Weight {
		// Minimum execution time: 232_328 nanoseconds.
		Weight::from_ref_time(238_073_000)
			.saturating_add(T::DbWeight::get().reads(27))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Loans Positions (r:1 w:1)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:3 w:1)
	// Storage: StableAsset Pools (r:2 w:1)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:3 w:2)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: Tokens Accounts (r:9 w:9)
	// Storage: System Account (r:3 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	// Storage: Rewards SharesAndWithdrawnRewards (r:1 w:1)
	// Storage: Rewards PoolInfos (r:1 w:1)
	// Storage: Loans TotalPositions (r:1 w:1)
	fn shrink_position_debit() -> Weight {
		// Minimum execution time: 373_290 nanoseconds.
		Weight::from_ref_time(377_905_000)
			.saturating_add(T::DbWeight::get().reads(34))
			.saturating_add(T::DbWeight::get().writes(18))
	}
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Tokens TotalIssuance (r:2 w:1)
	// Storage: CdpEngine CollateralParams (r:2 w:0)
	// Storage: Loans Positions (r:2 w:2)
	// Storage: Loans TotalPositions (r:2 w:2)
	// Storage: CdpEngine DebitExchangeRate (r:2 w:0)
	// Storage: Prices LockedPrice (r:3 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	fn transfer_debit() -> Weight {
		// Minimum execution time: 143_336 nanoseconds.
		Weight::from_ref_time(148_167_000)
			.saturating_add(T::DbWeight::get().reads(20))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: Loans Positions (r:1 w:0)
	// Storage: Prices LockedPrice (r:2 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:2 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: CdpEngine DebitExchangeRate (r:1 w:0)
	fn precompile_get_current_collateral_ratio() -> Weight {
		// Minimum execution time: 37_891 nanoseconds.
		Weight::from_ref_time(39_096_000)
			.saturating_add(T::DbWeight::get().reads(11))
	}
}
