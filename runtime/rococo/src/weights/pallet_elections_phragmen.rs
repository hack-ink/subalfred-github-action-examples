// Copyright 2017-2022 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(_v: u32, ) -> Weight {
		// Minimum execution time: 32_371 nanoseconds.
		Weight::from_ref_time(36_279_856)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		// Minimum execution time: 43_151 nanoseconds.
		Weight::from_ref_time(44_063_579)
			// Standard Error: 5_084
			.saturating_add(Weight::from_ref_time(130_645).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		// Minimum execution time: 42_148 nanoseconds.
		Weight::from_ref_time(43_777_573)
			// Standard Error: 6_625
			.saturating_add(Weight::from_ref_time(160_293).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		// Minimum execution time: 41_190 nanoseconds.
		Weight::from_ref_time(41_621_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Minimum execution time: 37_427 nanoseconds.
		Weight::from_ref_time(29_293_841)
			// Standard Error: 1_053
			.saturating_add(Weight::from_ref_time(97_420).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 33_012 nanoseconds.
		Weight::from_ref_time(24_872_179)
			// Standard Error: 1_044
			.saturating_add(Weight::from_ref_time(72_609).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		// Minimum execution time: 46_468 nanoseconds.
		Weight::from_ref_time(47_505_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		// Minimum execution time: 35_632 nanoseconds.
		Weight::from_ref_time(36_854_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		// Minimum execution time: 2_000_000_000 nanoseconds.
		Weight::from_ref_time(2_000_000_000_000)
	}
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		// Minimum execution time: 60_998 nanoseconds.
		Weight::from_ref_time(62_539_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: PhragmenElection Voting (r:5001 w:5000)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		// Minimum execution time: 285_403_894 nanoseconds.
		Weight::from_ref_time(285_860_459_000)
			// Standard Error: 248_756
			.saturating_add(Weight::from_ref_time(35_708_872).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: PhragmenElection Voting (r:10001 w:0)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: PhragmenElection ElectionRounds (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:3 w:3)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Minimum execution time: 29_683_290 nanoseconds.
		Weight::from_ref_time(29_810_601_000)
			// Standard Error: 548_695
			.saturating_add(Weight::from_ref_time(46_375_520).saturating_mul(v.into()))
			// Standard Error: 35_211
			.saturating_add(Weight::from_ref_time(2_381_861).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(265))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}
