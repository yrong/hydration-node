// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for orml_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=orml-vesting
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// vesting.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use orml_vesting::WeightInfo;

/// Weights for orml_vesting using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(2850), added: 5325, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
<<<<<<< HEAD
	fn vested_transfer() -> Weight {
		// Minimum execution time: 82_391 nanoseconds.
		Weight::from_ref_time(83_367_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
=======
    fn vested_transfer() -> Weight {
        // Minimum execution time: 84_347 nanoseconds.
        Weight::from_ref_time(85_089_000 as u64)            .saturating_add(T::DbWeight::get().reads(4 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
>>>>>>> 8caba3a7 (new weights)
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(2850), added: 5325, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 100]`.
<<<<<<< HEAD
	fn claim(i: u32) -> Weight {
		// Minimum execution time: 53_257 nanoseconds.
		Weight::from_ref_time(55_024_947 as u64) // Standard Error: 4_032
			.saturating_add(Weight::from_ref_time(82_308 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
=======
    fn claim(i: u32, ) -> Weight {
        // Minimum execution time: 52_945 nanoseconds.
        Weight::from_ref_time(56_504_723 as u64)            // Standard Error: 10_837
            .saturating_add(Weight::from_ref_time(63_081 as u64).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
>>>>>>> 8caba3a7 (new weights)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: Vesting VestingSchedules (r:0 w:1)
	// Proof: Vesting VestingSchedules (max_values: None, max_size: Some(2850), added: 5325, mode: MaxEncodedLen)
	/// The range of component `i` is `[1, 100]`.
<<<<<<< HEAD
	fn update_vesting_schedules(i: u32) -> Weight {
		// Minimum execution time: 45_592 nanoseconds.
		Weight::from_ref_time(46_909_590 as u64) // Standard Error: 2_484
			.saturating_add(Weight::from_ref_time(91_799 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
=======
    fn update_vesting_schedules(i: u32, ) -> Weight {
        // Minimum execution time: 44_932 nanoseconds.
        Weight::from_ref_time(46_379_059 as u64)            // Standard Error: 3_056
            .saturating_add(Weight::from_ref_time(91_387 as u64).saturating_mul(i as u64))
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
>>>>>>> 8caba3a7 (new weights)
}
