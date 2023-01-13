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
//! Autogenerated weights for `frame_benchmarking::baseline`
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
// --pallet=frame_benchmarking::baseline
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/frame_benchmarking_baseline.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_benchmarking::baseline`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_benchmarking::baseline::WeightInfo for WeightInfo<T> {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		// Minimum execution time: 136 nanoseconds.
		Weight::from_ref_time(166_415)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		// Minimum execution time: 134 nanoseconds.
		Weight::from_ref_time(157_476)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		// Minimum execution time: 146 nanoseconds.
		Weight::from_ref_time(181_850)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		// Minimum execution time: 138 nanoseconds.
		Weight::from_ref_time(160_267)
	}
	/// The range of component `i` is `[0, 100]`.
	fn hashing(i: u32, ) -> Weight {
		// Minimum execution time: 20_402_851 nanoseconds.
		Weight::from_ref_time(20_598_144_604)
			// Standard Error: 170_002
			.saturating_add(Weight::from_ref_time(1_774_395).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 100]`.
	fn sr25519_verification(i: u32, ) -> Weight {
		// Minimum execution time: 172 nanoseconds.
		Weight::from_ref_time(184_000)
			// Standard Error: 15_734
			.saturating_add(Weight::from_ref_time(47_361_013).saturating_mul(i.into()))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_read(i: u32, ) -> Weight {
		// Minimum execution time: 153 nanoseconds.
		Weight::from_ref_time(158_000)
			// Standard Error: 3_447
			.saturating_add(Weight::from_ref_time(1_943_306).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_write(i: u32, ) -> Weight {
		// Minimum execution time: 156 nanoseconds.
		Weight::from_ref_time(165_000)
			// Standard Error: 826
			.saturating_add(Weight::from_ref_time(339_692).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
	}
}