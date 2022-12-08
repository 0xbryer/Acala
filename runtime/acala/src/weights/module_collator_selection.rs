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

//! Autogenerated weights for module_collator_selection
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

/// Weight functions for module_collator_selection.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// The range of component `b` is `[1, 10]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Minimum execution time: 12_578 nanoseconds.
		Weight::from_ref_time(13_346_963)
			// Standard Error: 2_872
			.saturating_add(Weight::from_ref_time(73_633).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 11_965 nanoseconds.
		Weight::from_ref_time(12_693_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		// Minimum execution time: 12_629 nanoseconds.
		Weight::from_ref_time(13_034_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	/// The range of component `c` is `[5, 50]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 49_224 nanoseconds.
		Weight::from_ref_time(50_238_062)
			// Standard Error: 3_900
			.saturating_add(Weight::from_ref_time(451_851).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:0)
	/// The range of component `c` is `[1, 50]`.
	fn register_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 29_324 nanoseconds.
		Weight::from_ref_time(34_029_376)
			// Standard Error: 3_925
			.saturating_add(Weight::from_ref_time(410_209).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: CollatorSelection NonCandidates (r:0 w:1)
	/// The range of component `c` is `[6, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Minimum execution time: 23_819 nanoseconds.
		Weight::from_ref_time(23_794_464)
			// Standard Error: 2_538
			.saturating_add(Weight::from_ref_time(303_448).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	fn withdraw_bond() -> Weight {
		// Minimum execution time: 50_301 nanoseconds.
		Weight::from_ref_time(51_823_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: CollatorSelection SessionPoints (r:1 w:0)
	fn note_author() -> Weight {
		// Minimum execution time: 36_933 nanoseconds.
		Weight::from_ref_time(38_485_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	fn new_session() -> Weight {
		// Minimum execution time: 21_585 nanoseconds.
		Weight::from_ref_time(22_879_000)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Session Validators (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection SessionPoints (r:0 w:50)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn start_session(r: u32, c: u32, ) -> Weight {
		// Minimum execution time: 17_739 nanoseconds.
		Weight::from_ref_time(12_683_574)
			// Standard Error: 1_840
			.saturating_add(Weight::from_ref_time(12_306).saturating_mul(r.into()))
			// Standard Error: 1_840
			.saturating_add(Weight::from_ref_time(1_077_642).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	// Storage: CollatorSelection SessionPoints (r:51 w:50)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: CollatorSelection NonCandidates (r:0 w:44)
	/// The range of component `r` is `[5, 50]`.
	/// The range of component `c` is `[5, 50]`.
	fn end_session(_r: u32, c: u32, ) -> Weight {
		// Minimum execution time: 37_372 nanoseconds.
		Weight::from_ref_time(419_633_853)
			// Standard Error: 16_600
			.saturating_add(Weight::from_ref_time(6_016_948).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(50))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}
