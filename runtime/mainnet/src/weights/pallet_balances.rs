//! Autogenerated weights for `pallet_balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("mainnet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/ternoa
// benchmark
// pallet
// --chain=mainnet-dev
// --steps=50
// --repeat=20
// --pallet=pallet_balances
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./output

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
        // Storage: System Account (r:1 w:1)
        fn transfer() -> Weight {
                Weight::from_ref_time(87_385_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: System Account (r:1 w:1)
        fn transfer_keep_alive() -> Weight {
                Weight::from_ref_time(68_890_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: System Account (r:1 w:1)
        fn set_balance_creating() -> Weight {
                Weight::from_ref_time(53_469_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: System Account (r:1 w:1)
        fn set_balance_killing() -> Weight {
                Weight::from_ref_time(57_178_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: System Account (r:2 w:2)
        fn force_transfer() -> Weight {
                Weight::from_ref_time(100_700_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
        // Storage: System Account (r:1 w:1)
        fn transfer_all() -> Weight {
                Weight::from_ref_time(119_485_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: System Account (r:1 w:1)
        fn force_unreserve() -> Weight {
                Weight::from_ref_time(45_135_000 as RefTimeWeight)
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
}
