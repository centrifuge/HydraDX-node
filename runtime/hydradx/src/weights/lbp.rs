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

//! Autogenerated weights for pallet_lbp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-11, STEPS: 10, REPEAT: 30, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-lbp
// --extrinsic=*
// --output=lbp_no_back.rs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_lbp::weights::WeightInfo;

/// Weights for pallet_lbp using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:1)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	fn create_pool() -> Weight {
		// Minimum execution time: 151_763 nanoseconds.
		Weight::from_ref_time(153_465_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:2)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	fn update_pool_data() -> Weight {
		// Minimum execution time: 31_781 nanoseconds.
		Weight::from_ref_time(32_284_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_liquidity() -> Weight {
		// Minimum execution time: 106_414 nanoseconds.
		Weight::from_ref_time(107_630_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:0)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:0 w:1)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	fn remove_liquidity() -> Weight {
		// Minimum execution time: 136_405 nanoseconds.
		Weight::from_ref_time(138_124_000 as u64)
			.saturating_add(T::DbWeight::get().reads(10 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn sell() -> Weight {
		// Minimum execution time: 199_893 nanoseconds.
		Weight::from_ref_time(201_395_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn buy() -> Weight {
		// Minimum execution time: 199_112 nanoseconds.
		Weight::from_ref_time(200_863_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn trade_execution_sell() -> Weight {
		// Minimum execution time: 223_028 nanoseconds.
		Weight::from_ref_time(225_062_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn trade_execution_buy() -> Weight {
		// Minimum execution time: 223_313 nanoseconds.
		Weight::from_ref_time(224_794_000 as u64)
			.saturating_add(T::DbWeight::get().reads(12 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
}
