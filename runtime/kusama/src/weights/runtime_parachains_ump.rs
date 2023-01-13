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
//! Autogenerated weights for `runtime_parachains::ump`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::ump
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_parachains_ump.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::ump`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::ump::WeightInfo for WeightInfo<T> {
	/// The range of component `s` is `[0, 51200]`.
	fn process_upward_message(s: u32, ) -> Weight {
		// Minimum execution time: 10_991 nanoseconds.
		Weight::from_ref_time(5_407_947)
			// Standard Error: 13
			.saturating_add(Weight::from_ref_time(1_937).saturating_mul(s.into()))
	}
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: Ump RelayDispatchQueues (r:0 w:1)
	// Storage: Ump RelayDispatchQueueSize (r:0 w:1)
	fn clean_ump_after_outgoing() -> Weight {
		// Minimum execution time: 10_134 nanoseconds.
		Weight::from_ref_time(10_487_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Ump Overweight (r:1 w:1)
	fn service_overweight() -> Weight {
		// Minimum execution time: 27_101 nanoseconds.
		Weight::from_ref_time(27_760_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}