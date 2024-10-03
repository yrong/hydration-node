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


//! Autogenerated weights for `pallet_tips`
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
// --pallet=pallet-tips
// --extrinsic=*
// --template=scripts/pallet-weight-template.hbs
// --output=./weights/pallet_tips.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);

/// Weights for `pallet_tips` using the HydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for HydraWeight<T> {
	/// Storage: `Tips::Reasons` (r:1 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[0, 1024]`.
	fn report_awesome(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 34_011_000 picoseconds.
		Weight::from_parts(35_102_953, 3507)
			// Standard Error: 33
			.saturating_add(Weight::from_parts(1_220, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn retract_tip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259`
		//  Estimated: `3724`
		// Minimum execution time: 32_632_000 picoseconds.
		Weight::from_parts(33_087_000, 3724)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:1 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:0 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `r` is `[0, 1024]`.
	/// The range of component `t` is `[1, 13]`.
	fn tip_new(r: u32, t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438 + t * (64 ±0)`
		//  Estimated: `3903 + t * (64 ±0)`
		// Minimum execution time: 22_754_000 picoseconds.
		Weight::from_parts(22_734_008, 3903)
			// Standard Error: 20
			.saturating_add(Weight::from_parts(1_658, 0).saturating_mul(r.into()))
			// Standard Error: 1_648
			.saturating_add(Weight::from_parts(37_991, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(Weight::from_parts(0, 64).saturating_mul(t.into()))
	}
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 13]`.
	fn tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `659 + t * (112 ±0)`
		//  Estimated: `4124 + t * (112 ±0)`
		// Minimum execution time: 18_711_000 picoseconds.
		Weight::from_parts(18_993_380, 4124)
			// Standard Error: 1_903
			.saturating_add(Weight::from_parts(147_068, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 112).saturating_mul(t.into()))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 13]`.
	fn close_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `801 + t * (112 ±0)`
		//  Estimated: `6196 + t * (112 ±0)`
		// Minimum execution time: 67_694_000 picoseconds.
		Weight::from_parts(68_775_940, 6196)
			// Standard Error: 6_603
			.saturating_add(Weight::from_parts(145_198, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(Weight::from_parts(0, 112).saturating_mul(t.into()))
	}
	/// Storage: `Tips::Tips` (r:1 w:1)
	/// Proof: `Tips::Tips` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tips::Reasons` (r:0 w:1)
	/// Proof: `Tips::Reasons` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `t` is `[1, 13]`.
	fn slash_tip(t: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `307`
		//  Estimated: `3772`
		// Minimum execution time: 16_772_000 picoseconds.
		Weight::from_parts(17_200_331, 3772)
			// Standard Error: 962
			.saturating_add(Weight::from_parts(13_197, 0).saturating_mul(t.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}