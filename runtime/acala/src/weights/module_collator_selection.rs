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
//! DATE: 2022-06-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
	fn set_invulnerables(b: u32, ) -> Weight {
		(10_070_000 as Weight)
			// Standard Error: 57_000
			.saturating_add((88_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		(8_758_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		(8_971_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	fn register_as_candidate(c: u32, ) -> Weight {
		(43_366_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((276_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:0)
	fn register_candidate(c: u32, ) -> Weight {
		(26_548_000 as Weight)
			// Standard Error: 12_000
			.saturating_add((274_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: CollatorSelection NonCandidates (r:0 w:1)
	fn leave_intent(c: u32, ) -> Weight {
		(17_232_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((245_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	fn withdraw_bond() -> Weight {
		(50_871_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: CollatorSelection SessionPoints (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	fn note_author() -> Weight {
		(32_748_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	fn new_session() -> Weight {
		(17_484_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Session Validators (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: CollatorSelection SessionPoints (r:0 w:50)
	fn start_session(r: u32, c: u32, ) -> Weight {
		(7_070_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((32_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 11_000
			.saturating_add((1_046_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
	// Storage: CollatorSelection SessionPoints (r:51 w:50)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: CollatorSelection NonCandidates (r:0 w:44)
	fn end_session(_r: u32, c: u32, ) -> Weight {
		(545_573_000 as Weight)
			// Standard Error: 146_000
			.saturating_add((5_969_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes(51 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
	}
}
