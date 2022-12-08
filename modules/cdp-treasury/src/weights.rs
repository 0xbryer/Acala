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

//! Autogenerated weights for module_cdp_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_cdp_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/cdp-treasury/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_cdp_treasury.
pub trait WeightInfo {
	fn extract_surplus_to_treasury() -> Weight;
	fn auction_collateral(b: u32) -> Weight;
	fn exchange_collateral_to_stable() -> Weight;
	fn set_expected_collateral_auction_size() -> Weight;
}

/// Weights for module_cdp_treasury using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn auction_collateral(b: u32, ) -> Weight {
		Weight::from_ref_time(2_672_000)
			// Standard Error: 326_000
			.saturating_add(Weight::from_ref_time(32_334_000).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(b as u64)))
	}
	fn exchange_collateral_to_stable() -> Weight {
		Weight::from_ref_time(176_000_000)
			.saturating_add(T::DbWeight::get().reads(9 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	fn set_expected_collateral_auction_size() -> Weight {
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn extract_surplus_to_treasury() -> Weight {
		Weight::from_ref_time(75_000_000)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn auction_collateral(b: u32, ) -> Weight {
		Weight::from_ref_time(2_672_000)
			.saturating_add(Weight::from_ref_time(32_334_000).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(b as u64)))
	}
	fn exchange_collateral_to_stable() -> Weight {
		Weight::from_ref_time(176_000_000)
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	fn set_expected_collateral_auction_size() -> Weight {
		Weight::from_ref_time(25_000_000)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn extract_surplus_to_treasury() -> Weight {
		Weight::from_ref_time(75_000_000)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
}
