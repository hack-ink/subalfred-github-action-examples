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
//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_referenda
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	// Storage: Referenda ReferendumCount (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:0 w:1)
	fn submit() -> Weight {
		// Minimum execution time: 37_755 nanoseconds.
		Weight::from_ref_time(38_346_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_preparing() -> Weight {
		// Minimum execution time: 48_492 nanoseconds.
		Weight::from_ref_time(49_586_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_queued() -> Weight {
		// Minimum execution time: 50_416 nanoseconds.
		Weight::from_ref_time(51_153_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	fn place_decision_deposit_not_queued() -> Weight {
		// Minimum execution time: 49_940 nanoseconds.
		Weight::from_ref_time(50_664_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn place_decision_deposit_passing() -> Weight {
		// Minimum execution time: 63_167 nanoseconds.
		Weight::from_ref_time(64_982_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	fn place_decision_deposit_failing() -> Weight {
		// Minimum execution time: 44_569 nanoseconds.
		Weight::from_ref_time(46_163_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn refund_decision_deposit() -> Weight {
		// Minimum execution time: 31_451 nanoseconds.
		Weight::from_ref_time(32_309_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn refund_submission_deposit() -> Weight {
		// Minimum execution time: 30_259 nanoseconds.
		Weight::from_ref_time(30_856_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn cancel() -> Weight {
		// Minimum execution time: 38_179 nanoseconds.
		Weight::from_ref_time(39_326_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn kill() -> Weight {
		// Minimum execution time: 79_978 nanoseconds.
		Weight::from_ref_time(80_846_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda TrackQueue (r:1 w:0)
	// Storage: Referenda DecidingCount (r:1 w:1)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Minimum execution time: 11_591 nanoseconds.
		Weight::from_ref_time(12_103_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_failing() -> Weight {
		// Minimum execution time: 78_771 nanoseconds.
		Weight::from_ref_time(79_849_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn one_fewer_deciding_passing() -> Weight {
		// Minimum execution time: 80_373 nanoseconds.
		Weight::from_ref_time(81_121_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:0)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Minimum execution time: 53_641 nanoseconds.
		Weight::from_ref_time(54_278_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:0)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Minimum execution time: 53_505 nanoseconds.
		Weight::from_ref_time(53_954_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:0)
	fn nudge_referendum_queued() -> Weight {
		// Minimum execution time: 55_473 nanoseconds.
		Weight::from_ref_time(56_134_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:0)
	// Storage: Referenda TrackQueue (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:0)
	fn nudge_referendum_not_queued() -> Weight {
		// Minimum execution time: 55_330 nanoseconds.
		Weight::from_ref_time(56_137_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_no_deposit() -> Weight {
		// Minimum execution time: 29_234 nanoseconds.
		Weight::from_ref_time(30_016_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_preparing() -> Weight {
		// Minimum execution time: 30_320 nanoseconds.
		Weight::from_ref_time(31_207_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	fn nudge_referendum_timed_out() -> Weight {
		// Minimum execution time: 22_278 nanoseconds.
		Weight::from_ref_time(22_963_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Minimum execution time: 42_438 nanoseconds.
		Weight::from_ref_time(43_206_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Referenda DecidingCount (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Minimum execution time: 44_071 nanoseconds.
		Weight::from_ref_time(44_793_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Minimum execution time: 40_206 nanoseconds.
		Weight::from_ref_time(41_719_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_end_confirming() -> Weight {
		// Minimum execution time: 41_848 nanoseconds.
		Weight::from_ref_time(42_645_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Minimum execution time: 38_801 nanoseconds.
		Weight::from_ref_time(39_701_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Minimum execution time: 37_936 nanoseconds.
		Weight::from_ref_time(38_758_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:2 w:2)
	// Storage: Scheduler Lookup (r:1 w:1)
	fn nudge_referendum_approved() -> Weight {
		// Minimum execution time: 50_402 nanoseconds.
		Weight::from_ref_time(51_317_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Balances InactiveIssuance (r:1 w:0)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn nudge_referendum_rejected() -> Weight {
		// Minimum execution time: 40_386 nanoseconds.
		Weight::from_ref_time(41_208_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
