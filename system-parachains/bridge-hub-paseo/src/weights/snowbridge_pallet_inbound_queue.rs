// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
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
//! Autogenerated weights for `snowbridge_pallet_inbound_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-03-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./bridge-hub-polkadot-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=./bridge-hub-polkadot-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=snowbridge_pallet_inbound_queue
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./bridge-hub-polkadot-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `snowbridge_pallet_inbound_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> snowbridge_pallet_inbound_queue::WeightInfo for WeightInfo<T> {
	/// Storage: `EthereumInboundQueue::OperatingMode` (r:1 w:0)
	/// Proof: `EthereumInboundQueue::OperatingMode` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `EthereumBeaconClient::ExecutionHeaders` (r:1 w:0)
	/// Proof: `EthereumBeaconClient::ExecutionHeaders` (`max_values`: None, `max_size`: Some(136), added: 2611, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xaed97c7854d601808b98ae43079dafb3` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0xaed97c7854d601808b98ae43079dafb3` (r:1 w:0)
	/// Storage: `EthereumSystem::Channels` (r:1 w:0)
	/// Proof: `EthereumSystem::Channels` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `EthereumInboundQueue::Nonce` (r:1 w:1)
	/// Proof: `EthereumInboundQueue::Nonce` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `EthereumSystem::PricingParameters` (r:1 w:0)
	/// Proof: `EthereumSystem::PricingParameters` (`max_values`: Some(1), `max_size`: Some(112), added: 607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `815`
		//  Estimated: `4280`
		// Minimum execution time: 104_849_000 picoseconds.
		Weight::from_parts(140_211_500, 0)
			.saturating_add(Weight::from_parts(0, 4280))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
