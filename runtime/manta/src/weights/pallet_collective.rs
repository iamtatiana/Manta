// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("manta-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=manta-dev
// --steps=50
// --repeat=40
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_collective.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_collective.
pub trait WeightInfo {
    fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
    fn execute(b: u32, m: u32, ) -> Weight;
    fn propose_execute(b: u32, m: u32, ) -> Weight;
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
    fn vote(m: u32, ) -> Weight;
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn close_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn disapprove_proposal(p: u32, ) -> Weight;
}

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 20_001 nanoseconds.
		Weight::from_ref_time(20_185_000)
			// Standard Error: 52_494
			.saturating_add(Weight::from_ref_time(5_981_845).saturating_mul(m.into()))
			// Standard Error: 52_494
			.saturating_add(Weight::from_ref_time(8_435_582).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Council Members (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(_b: u32, _m: u32, ) -> Weight {
		// Minimum execution time: 21_456 nanoseconds.
		Weight::from_ref_time(30_857_795)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 23_638 nanoseconds.
		Weight::from_ref_time(23_616_139)
			// Standard Error: 61
			.saturating_add(Weight::from_ref_time(1_931).saturating_mul(b.into()))
			// Standard Error: 631
			.saturating_add(Weight::from_ref_time(25_329).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 30_261 nanoseconds.
		Weight::from_ref_time(31_870_155)
			// Standard Error: 95
			.saturating_add(Weight::from_ref_time(4_075).saturating_mul(b.into()))
			// Standard Error: 999
			.saturating_add(Weight::from_ref_time(22_824).saturating_mul(m.into()))
			// Standard Error: 987
			.saturating_add(Weight::from_ref_time(166_520).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 33_625 nanoseconds.
		Weight::from_ref_time(34_613_592)
			// Standard Error: 426
			.saturating_add(Weight::from_ref_time(49_464).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 32_897 nanoseconds.
		Weight::from_ref_time(35_124_529)
			// Standard Error: 882
			.saturating_add(Weight::from_ref_time(29_030).saturating_mul(m.into()))
			// Standard Error: 860
			.saturating_add(Weight::from_ref_time(161_391).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 45_174 nanoseconds.
		Weight::from_ref_time(45_009_641)
			// Standard Error: 89
			.saturating_add(Weight::from_ref_time(4_154).saturating_mul(b.into()))
			// Standard Error: 950
			.saturating_add(Weight::from_ref_time(23_528).saturating_mul(m.into()))
			// Standard Error: 926
			.saturating_add(Weight::from_ref_time(188_619).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 35_348 nanoseconds.
		Weight::from_ref_time(39_085_383)
			// Standard Error: 871
			.saturating_add(Weight::from_ref_time(26_533).saturating_mul(m.into()))
			// Standard Error: 849
			.saturating_add(Weight::from_ref_time(158_715).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 47_311 nanoseconds.
		Weight::from_ref_time(50_881_513)
			// Standard Error: 97
			.saturating_add(Weight::from_ref_time(2_182).saturating_mul(b.into()))
			// Standard Error: 1_032
			.saturating_add(Weight::from_ref_time(17_415).saturating_mul(m.into()))
			// Standard Error: 1_006
			.saturating_add(Weight::from_ref_time(178_670).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 21_229 nanoseconds.
		Weight::from_ref_time(24_462_323)
			// Standard Error: 1_172
			.saturating_add(Weight::from_ref_time(162_850).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 20_001 nanoseconds.
		Weight::from_ref_time(20_185_000)
			// Standard Error: 52_494
			.saturating_add(Weight::from_ref_time(5_981_845).saturating_mul(m.into()))
			// Standard Error: 52_494
			.saturating_add(Weight::from_ref_time(8_435_582).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(RocksDbWeight::get().writes(2))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Council Members (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(_b: u32, _m: u32, ) -> Weight {
		// Minimum execution time: 21_456 nanoseconds.
		Weight::from_ref_time(30_857_795)
			.saturating_add(RocksDbWeight::get().reads(1))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 23_638 nanoseconds.
		Weight::from_ref_time(23_616_139)
			// Standard Error: 61
			.saturating_add(Weight::from_ref_time(1_931).saturating_mul(b.into()))
			// Standard Error: 631
			.saturating_add(Weight::from_ref_time(25_329).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 30_261 nanoseconds.
		Weight::from_ref_time(31_870_155)
			// Standard Error: 95
			.saturating_add(Weight::from_ref_time(4_075).saturating_mul(b.into()))
			// Standard Error: 999
			.saturating_add(Weight::from_ref_time(22_824).saturating_mul(m.into()))
			// Standard Error: 987
			.saturating_add(Weight::from_ref_time(166_520).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 33_625 nanoseconds.
		Weight::from_ref_time(34_613_592)
			// Standard Error: 426
			.saturating_add(Weight::from_ref_time(49_464).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 32_897 nanoseconds.
		Weight::from_ref_time(35_124_529)
			// Standard Error: 882
			.saturating_add(Weight::from_ref_time(29_030).saturating_mul(m.into()))
			// Standard Error: 860
			.saturating_add(Weight::from_ref_time(161_391).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 45_174 nanoseconds.
		Weight::from_ref_time(45_009_641)
			// Standard Error: 89
			.saturating_add(Weight::from_ref_time(4_154).saturating_mul(b.into()))
			// Standard Error: 950
			.saturating_add(Weight::from_ref_time(23_528).saturating_mul(m.into()))
			// Standard Error: 926
			.saturating_add(Weight::from_ref_time(188_619).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 35_348 nanoseconds.
		Weight::from_ref_time(39_085_383)
			// Standard Error: 871
			.saturating_add(Weight::from_ref_time(26_533).saturating_mul(m.into()))
			// Standard Error: 849
			.saturating_add(Weight::from_ref_time(158_715).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 47_311 nanoseconds.
		Weight::from_ref_time(50_881_513)
			// Standard Error: 97
			.saturating_add(Weight::from_ref_time(2_182).saturating_mul(b.into()))
			// Standard Error: 1_032
			.saturating_add(Weight::from_ref_time(17_415).saturating_mul(m.into()))
			// Standard Error: 1_006
			.saturating_add(Weight::from_ref_time(178_670).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 21_229 nanoseconds.
		Weight::from_ref_time(24_462_323)
			// Standard Error: 1_172
			.saturating_add(Weight::from_ref_time(162_850).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}