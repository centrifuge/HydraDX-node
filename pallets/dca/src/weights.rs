// This file is part of HydraDX.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for pallet_dca
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-18, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-dca
// --chain=local
// --steps=5
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// weights.rs
// --template
// .maintain/pallet-weight-template.hbs
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_dca.
pub trait WeightInfo {
	fn on_initialize_with_one_trade() -> Weight;
	fn on_initialize_with_empty_block() -> Weight;
	fn schedule() -> Weight;
	fn terminate() -> Weight;
}

/// Weights for pallet_dca using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn on_initialize_with_one_trade() -> Weight {
		Weight::from_ref_time(299_988_000 as u64)
			.saturating_add(T::DbWeight::get().reads(28 as u64))
			.saturating_add(T::DbWeight::get().writes(17 as u64))
	}
	fn on_initialize_with_empty_block() -> Weight {
		Weight::from_ref_time(9_270_000 as u64).saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	fn schedule() -> Weight {
		Weight::from_ref_time(118_999_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(8 as u64))
	}
	fn terminate() -> Weight {
		Weight::from_ref_time(63_169_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(7 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize_with_one_trade() -> Weight {
		Weight::from_ref_time(299_988_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(28 as u64))
			.saturating_add(RocksDbWeight::get().writes(17 as u64))
	}
	fn on_initialize_with_empty_block() -> Weight {
		Weight::from_ref_time(9_270_000 as u64).saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	fn schedule() -> Weight {
		Weight::from_ref_time(118_999_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(11 as u64))
			.saturating_add(RocksDbWeight::get().writes(8 as u64))
	}
	fn terminate() -> Weight {
		Weight::from_ref_time(63_169_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(7 as u64))
	}
}
