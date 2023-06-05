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

//! Autogenerated weights for pallet_otc
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-otc
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// otc.rs
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

use pallet_otc::weights::WeightInfo;

/// Weights for pallet_otc using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: OTC NextOrderId (r:1 w:1)
	// Proof: OTC NextOrderId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: OTC Orders (r:0 w:1)
	// Proof: OTC Orders (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	fn place_order() -> Weight {
		// Minimum execution time: 27_180 nanoseconds.
		Weight::from_ref_time(27_624_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: OTC Orders (r:1 w:1)
	// Proof: OTC Orders (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	fn partial_fill_order() -> Weight {
		// Minimum execution time: 60_362 nanoseconds.
		Weight::from_ref_time(61_284_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: OTC Orders (r:1 w:1)
	// Proof: OTC Orders (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	fn fill_order() -> Weight {
		// Minimum execution time: 56_500 nanoseconds.
		Weight::from_ref_time(57_570_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: OTC Orders (r:1 w:1)
	// Proof: OTC Orders (max_values: None, max_size: Some(93), added: 2568, mode: MaxEncodedLen)
	// Storage: Balances Reserves (r:1 w:1)
	// Proof: Balances Reserves (max_values: None, max_size: Some(1249), added: 3724, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn cancel_order() -> Weight {
		// Minimum execution time: 21_967 nanoseconds.
		Weight::from_ref_time(22_568_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
