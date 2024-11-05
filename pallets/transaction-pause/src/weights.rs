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


//! Autogenerated weights for `pallet_transaction_pause`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --pallet=pallet-transaction-pause
// --extrinsic=*
// --template=scripts/pallet-weight-template.hbs
// --output=./weights/pallet_transaction_pause.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_transaction_pause.
pub trait WeightInfo {
	fn pause_transaction() -> Weight;
	fn unpause_transaction() -> Weight;
}

/// Weights for module_transaction_pause using the Acala node and recommended hardware.
impl WeightInfo for () {
	/// Storage: `TransactionPause::PausedTransactions` (r:1 w:1)
	/// Proof: `TransactionPause::PausedTransactions` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	fn pause_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3555`
		// Minimum execution time: 10_654_000 picoseconds.
		Weight::from_parts(11_047_000, 3555)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `TransactionPause::PausedTransactions` (r:1 w:1)
	/// Proof: `TransactionPause::PausedTransactions` (`max_values`: None, `max_size`: Some(90), added: 2565, mode: `MaxEncodedLen`)
	fn unpause_transaction() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `160`
		//  Estimated: `3555`
		// Minimum execution time: 12_775_000 picoseconds.
		Weight::from_parts(13_283_000, 3555)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
