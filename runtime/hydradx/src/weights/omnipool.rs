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

//! Autogenerated weights for pallet_omnipool
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-24, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-omnipool
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// omnipool.rs
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

use pallet_omnipool::weights::WeightInfo;

/// Weights for pallet_omnipool using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Omnipool Assets (r:2 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:1)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool NextPositionId (r:1 w:1)
	// Proof: Omnipool NextPositionId (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: Omnipool TvlCap (r:1 w:0)
	// Proof: Omnipool TvlCap (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Omnipool Positions (r:0 w:1)
	// Proof: Omnipool Positions (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	fn add_token() -> Weight {
		// Minimum execution time: 150_622 nanoseconds.
		Weight::from_ref_time(151_024_000 as u64)
			.saturating_add(T::DbWeight::get().reads(15 as u64))
			.saturating_add(T::DbWeight::get().writes(10 as u64))
	}
	// Storage: Tokens Accounts (r:4 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:2 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: Omnipool NextPositionId (r:1 w:1)
	// Proof: Omnipool NextPositionId (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Omnipool TvlCap (r:1 w:0)
	// Proof: Omnipool TvlCap (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Omnipool Positions (r:0 w:1)
	// Proof: Omnipool Positions (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 228_878 nanoseconds.
		Weight::from_ref_time(235_865_000 as u64)
			.saturating_add(T::DbWeight::get().reads(23 as u64))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
	}
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Omnipool Positions (r:1 w:1)
	// Proof: Omnipool Positions (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:1 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:0)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	fn remove_liquidity() -> Weight {
		// Minimum execution time: 287_519 nanoseconds.
		Weight::from_ref_time(288_641_000 as u64)
			.saturating_add(T::DbWeight::get().reads(23 as u64))
			.saturating_add(T::DbWeight::get().writes(16 as u64))
	}
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:3 w:3)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: DynamicFees AssetFee (r:1 w:0)
	// Proof: DynamicFees AssetFee (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:1 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:0)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn sell() -> Weight {
		// Minimum execution time: 252_767 nanoseconds.
		Weight::from_ref_time(254_421_000 as u64)
			.saturating_add(T::DbWeight::get().reads(23 as u64))
			.saturating_add(T::DbWeight::get().writes(14 as u64))
	}
	// Storage: Omnipool Assets (r:3 w:3)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: DynamicFees AssetFee (r:1 w:1)
	// Proof: DynamicFees AssetFee (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:0)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn buy() -> Weight {
		// Minimum execution time: 272_781 nanoseconds.
		Weight::from_ref_time(274_844_000 as u64)
			.saturating_add(T::DbWeight::get().reads(24 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	fn set_asset_tradable_state() -> Weight {
		// Minimum execution time: 33_941 nanoseconds.
		Weight::from_ref_time(34_430_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:0)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn refund_refused_asset() -> Weight {
		// Minimum execution time: 106_574 nanoseconds.
		Weight::from_ref_time(107_433_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Omnipool Positions (r:1 w:1)
	// Proof: Omnipool Positions (max_values: None, max_size: Some(100), added: 2575, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Omnipool Assets (r:1 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	fn sacrifice_position() -> Weight {
		// Minimum execution time: 76_552 nanoseconds.
		Weight::from_ref_time(77_448_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	fn set_asset_weight_cap() -> Weight {
		// Minimum execution time: 34_304 nanoseconds.
		Weight::from_ref_time(34_615_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:3 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	fn withdraw_protocol_liquidity() -> Weight {
		// Minimum execution time: 160_117 nanoseconds.
		Weight::from_ref_time(161_104_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Omnipool Assets (r:1 w:1)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:3)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:2 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn remove_token() -> Weight {
		// Minimum execution time: 158_937 nanoseconds.
		Weight::from_ref_time(162_234_000 as u64)
			.saturating_add(T::DbWeight::get().reads(14 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Omnipool Assets (r:3 w:3)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: DynamicFees AssetFee (r:1 w:0)
	// Proof: DynamicFees AssetFee (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:1 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:0)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(c: u32, e: u32) -> Weight {
		// Minimum execution time: 53_821 nanoseconds.
		Weight::from_ref_time(41_181_125 as u64) // Standard Error: 379_584
			.saturating_add(Weight::from_ref_time(13_415_900 as u64).saturating_mul(c as u64))
			// Standard Error: 379_584
			.saturating_add(Weight::from_ref_time(214_332_825 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().reads((16 as u64).saturating_mul(e as u64)))
			.saturating_add(T::DbWeight::get().writes((14 as u64).saturating_mul(e as u64)))
	}
	// Storage: Omnipool Assets (r:3 w:3)
	// Proof: Omnipool Assets (max_values: None, max_size: Some(85), added: 2560, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Omnipool HubAssetImbalance (r:1 w:1)
	// Proof: Omnipool HubAssetImbalance (max_values: Some(1), max_size: Some(17), added: 512, mode: MaxEncodedLen)
	// Storage: DynamicFees AssetFee (r:1 w:1)
	// Proof: DynamicFees AssetFee (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	// Storage: EmaOracle Oracles (r:2 w:0)
	// Proof: EmaOracle Oracles (max_values: None, max_size: Some(177), added: 2652, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:1)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedTradeVolumeLimitPerAsset (r:2 w:2)
	// Proof: CircuitBreaker AllowedTradeVolumeLimitPerAsset (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityAddLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityAddLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedAddLiquidityAmountPerAsset (r:1 w:1)
	// Proof: CircuitBreaker AllowedAddLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: CircuitBreaker LiquidityRemoveLimitPerAsset (r:1 w:0)
	// Proof: CircuitBreaker LiquidityRemoveLimitPerAsset (max_values: None, max_size: Some(29), added: 2504, mode: MaxEncodedLen)
	// Storage: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (r:1 w:0)
	// Proof: CircuitBreaker AllowedRemoveLiquidityAmountPerAsset (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, e: u32) -> Weight {
		// Minimum execution time: 271_496 nanoseconds.
		Weight::from_ref_time(266_903_925 as u64) // Standard Error: 491_298
			.saturating_add(Weight::from_ref_time(9_205_000 as u64).saturating_mul(c as u64))
			// Standard Error: 491_298
			.saturating_add(Weight::from_ref_time(862_825 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(24 as u64))
			.saturating_add(T::DbWeight::get().writes(15 as u64))
	}
}
