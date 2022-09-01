
//! Autogenerated weights for `frame_system`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `marko-MS-7B85`, CPU: `AMD Ryzen 7 5800X 8-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/ternoa
// benchmark
// pallet
// --chain
// alphanet-dev
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=frame_system

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_system`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_system::WeightInfo for WeightInfo<T> {
	/// The range of component `b` is `[0, 3932160]`.
	fn remark(_b: u32, ) -> Weight {
		Weight::from_ref_time(9_321_000 as RefTimeWeight)
	}
	/// The range of component `b` is `[0, 3932160]`.
	fn remark_with_event(b: u32, ) -> Weight {
		Weight::from_ref_time(63_535_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as RefTimeWeight).scalar_saturating_mul(b as RefTimeWeight))
	}
	// Storage: System Digest (r:1 w:1)
	// Storage: unknown [0x3a686561707061676573] (r:0 w:1)
	fn set_heap_pages() -> Weight {
		Weight::from_ref_time(9_400_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[1, 1000]`.
	fn set_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(648_000 as RefTimeWeight).scalar_saturating_mul(i as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[1, 1000]`.
	fn kill_storage(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(512_000 as RefTimeWeight).scalar_saturating_mul(i as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(i as RefTimeWeight)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `p` is `[1, 1000]`.
	fn kill_prefix(p: u32, ) -> Weight {
		Weight::from_ref_time(0 as RefTimeWeight)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(1_058_000 as RefTimeWeight).scalar_saturating_mul(p as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(p as RefTimeWeight)))
	}
}
