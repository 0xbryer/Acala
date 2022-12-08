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

//! Autogenerated weights for module_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-28, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `cae23a5654f3`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_nft.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_nft::WeightInfo for WeightInfo<T> {
	// Storage: OrmlNFT NextClassId (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: OrmlNFT Classes (r:0 w:1)
	fn create_class() -> Weight {
		// Minimum execution time: 73_368 nanoseconds.
		Weight::from_ref_time(74_936_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: OrmlNFT NextTokenId (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:0 w:1)
	// Storage: OrmlNFT TokensByOwner (r:0 w:1)
	/// The range of component `i` is `[1, 1000]`.
	fn mint(i: u32, ) -> Weight {
		// Minimum execution time: 86_366 nanoseconds.
		Weight::from_ref_time(87_150_000)
			// Standard Error: 5_672
			.saturating_add(Weight::from_ref_time(18_358_563).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(i.into())))
	}
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Storage: OrmlNFT Tokens (r:1 w:1)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: OrmlNFT TokensByOwner (r:0 w:2)
	fn transfer() -> Weight {
		// Minimum execution time: 94_198 nanoseconds.
		Weight::from_ref_time(95_961_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT TokensByOwner (r:0 w:1)
	fn burn() -> Weight {
		// Minimum execution time: 68_918 nanoseconds.
		Weight::from_ref_time(70_919_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT TokensByOwner (r:0 w:1)
	/// The range of component `b` is `[0, 3670016]`.
	fn burn_with_remark(b: u32, ) -> Weight {
		// Minimum execution time: 69_550 nanoseconds.
		Weight::from_ref_time(70_361_000)
			// Standard Error: 3
			.saturating_add(Weight::from_ref_time(1_994).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: OrmlNFT NextTokenId (r:0 w:1)
	fn destroy_class() -> Weight {
		// Minimum execution time: 83_475 nanoseconds.
		Weight::from_ref_time(85_434_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	fn update_class_properties() -> Weight {
		// Minimum execution time: 18_378 nanoseconds.
		Weight::from_ref_time(18_872_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
