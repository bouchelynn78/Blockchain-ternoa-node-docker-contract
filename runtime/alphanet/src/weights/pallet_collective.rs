//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-01, STEPS: `10`, REPEAT: 5, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./ternoa
// benchmark
// --chain
// alphanet-dev
// --steps=10
// --repeat=5
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/
// --pallet=pallet_collective

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: TechnicalCommittee Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommittee Voting (r:100 w:100)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	fn set_members(m: u32, _n: u32, p: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 413_000
			.saturating_add((10_889_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 413_000
			.saturating_add((17_262_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	fn execute(_b: u32, _m: u32) -> Weight {
		(30_597_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32) -> Weight {
		(27_235_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 13_000
			.saturating_add((53_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	fn propose_proposed(_b: u32, _m: u32, p: u32) -> Weight {
		(57_381_000 as Weight)
			// Standard Error: 45_000
			.saturating_add((367_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	fn vote(m: u32) -> Weight {
		(56_673_000 as Weight)
			// Standard Error: 99_000
			.saturating_add((146_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32) -> Weight {
		(47_061_000 as Weight)
			// Standard Error: 40_000
			.saturating_add((26_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 39_000
			.saturating_add((410_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
		(21_591_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((19_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 75_000
			.saturating_add((323_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 72_000
			.saturating_add((544_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:1 w:0)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32) -> Weight {
		(39_852_000 as Weight)
			// Standard Error: 59_000
			.saturating_add((149_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 56_000
			.saturating_add((478_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32) -> Weight {
		(46_801_000 as Weight)
			// Standard Error: 8_000
			.saturating_add((13_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 88_000
			.saturating_add((154_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 84_000
			.saturating_add((589_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32) -> Weight {
		(28_982_000 as Weight)
			// Standard Error: 29_000
			.saturating_add((396_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
