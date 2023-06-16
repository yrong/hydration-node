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

//! Autogenerated weights for pallet_route_executor
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-route-executor
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// route_executor.rs
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

use pallet_route_executor::weights::WeightInfo;

/// Weights for pallet_route_executor using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 2]`.
<<<<<<< HEAD
	fn sell(_n: u32) -> Weight {
		// Minimum execution time: 242_968 nanoseconds.
		Weight::from_ref_time(249_548_000 as u64)
			.saturating_add(T::DbWeight::get().reads(16 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
=======
    fn sell(_n: u32, ) -> Weight {
        // Minimum execution time: 240_244 nanoseconds.
        Weight::from_ref_time(252_571_000 as u64)            .saturating_add(T::DbWeight::get().reads(16 as u64))
            .saturating_add(T::DbWeight::get().writes(12 as u64))
    }
>>>>>>> 8caba3a7 (new weights)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:2)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(2961), added: 3456, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 2]`.
<<<<<<< HEAD
	fn buy(n: u32) -> Weight {
		// Minimum execution time: 239_841 nanoseconds.
		Weight::from_ref_time(238_415_250 as u64) // Standard Error: 807_409
			.saturating_add(Weight::from_ref_time(5_062_025 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(16 as u64))
			.saturating_add(T::DbWeight::get().writes(12 as u64))
	}
=======
    fn buy(n: u32, ) -> Weight {
        // Minimum execution time: 237_842 nanoseconds.
        Weight::from_ref_time(238_705_550 as u64)            // Standard Error: 227_230
            .saturating_add(Weight::from_ref_time(1_211_375 as u64).saturating_mul(n as u64))
            .saturating_add(T::DbWeight::get().reads(16 as u64))
            .saturating_add(T::DbWeight::get().writes(12 as u64))
    }
>>>>>>> 8caba3a7 (new weights)
}
