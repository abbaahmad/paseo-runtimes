// Copyright (C) Paseo and the various Paseo contributors, see Contributions.md
// for a list of specific contributors.
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

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `p-ch-hmb-vm-vapas-002`, CPU: `Intel(R) Xeon(R) Silver 4314 CPU @ 2.40GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./runtimes/paseo-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./polkadot-sdk/target/production/polkadot
// benchmark
// pallet
// --chain=./runtimes/paseo-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./paseo-weights-1.2/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_027_000 picoseconds.
		Weight::from_parts(5_402_145, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 1_670
			.saturating_add(Weight::from_parts(3_798_311, 0).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_798_000 picoseconds.
		Weight::from_parts(4_997_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_889_000 picoseconds.
		Weight::from_parts(6_789_092, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 1_214
			.saturating_add(Weight::from_parts(4_089_664, 0).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_060_000 picoseconds.
		Weight::from_parts(7_376_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_061_000 picoseconds.
		Weight::from_parts(5_217_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2_360
			.saturating_add(Weight::from_parts(3_821_307, 0).saturating_mul(c.into()))
	}
}
