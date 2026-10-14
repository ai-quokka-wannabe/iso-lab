/*
    Copyright (C) 2026 The AiQuokkaWannabe Authors <https://github.com/ai-quokka-wannabe/iso-lab>

    This program is free software: you can redistribute it and/or modify it under the terms of
    the GNU General Public License as published by the Free Software Foundation, either version
    3 of the License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
    without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
    See the GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along with this program.
    If not, see <https://www.gnu.org/licenses/>.
*/

//! The seeded draw of the reference records: one permutation of the whole eligible
//! manifest, fixed by a seed written down before it is drawn, cut into blocks that later
//! tasks take in order (data/README.md, *The Reference Draw*). Nothing is ever redrawn: a
//! need for more records takes the next block.
//!
//! The generator is SplitMix64 (Steele, Lea and Flood 2014,
//! <https://doi.org/10.1145/2714064.2660195>), as Vigna's public-domain reference code writes
//! it (<https://prng.di.unimi.it/splitmix64.c>): 64 bits of state, integer arithmetic only, so
//! the same seed gives the same draw on every machine.

/// SplitMix64, seeded once.
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// A generator whose state starts at `seed`.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// The next 64 bits.
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A uniform integer in `0..bound`, without the bias a plain remainder has: draws below
    /// `2^64 mod bound` are rejected, as OpenBSD's `arc4random_uniform` does. `bound` must
    /// not be zero.
    pub fn below(&mut self, bound: u64) -> u64 {
        assert!(bound > 0, "an empty range has no member to draw");
        let reject_below = bound.wrapping_neg() % bound;
        loop {
            let draw = self.next_u64();
            if draw >= reject_below {
                return draw % bound;
            }
        }
    }
}

/// The items in the order a Fisher-Yates shuffle seeded with `seed` puts them, starting from
/// the order given. The caller gives them sorted, so the draw depends on the seed alone.
pub fn permutation<T: Clone>(items: &[T], seed: u64) -> Vec<T> {
    let mut order = items.to_vec();
    let mut generator = SplitMix64::new(seed);
    for last in (1..order.len()).rev() {
        let pick = generator.below(last as u64 + 1) as usize;
        order.swap(last, pick);
    }
    order
}

#[cfg(test)]
mod tests {
    use super::*;

    // Printed by Vigna's reference splitmix64.c, compiled with gcc and seeded through its
    // state variable (the journal of 2026-10-14 has the commands).
    #[test]
    fn matches_the_reference_code() {
        let mut zero = SplitMix64::new(0);
        assert_eq!(
            [zero.next_u64(), zero.next_u64(), zero.next_u64()],
            [0xe220_a839_7b1d_cdaf, 0x6e78_9e6a_a1b9_65f4, 0x06c4_5d18_8009_454f]
        );
        let mut seeded = SplitMix64::new(20_261_014);
        assert_eq!(
            [seeded.next_u64(), seeded.next_u64(), seeded.next_u64()],
            [0x39a6_858c_b613_6e7d, 0x0fc0_d086_1082_fa0e, 0x2707_874c_4279_3f6c]
        );
    }

    #[test]
    fn below_stays_in_range_and_reaches_every_value() {
        let mut generator = SplitMix64::new(7);
        let mut seen = [0_u32; 6];
        for _ in 0..6_000 {
            let draw = generator.below(6);
            assert!(draw < 6);
            seen[draw as usize] += 1;
        }
        // Each face expects 1000; 800 is more than six standard deviations (about 29) away.
        assert!(seen.iter().all(|&count| count > 800), "{seen:?}");
    }

    // With a bound just above 2^63, about half of all draws fall below 2^64 mod bound and must
    // be rejected; a plain remainder would keep the first draw. Seed 20261014's first output
    // (0x39a6...) is below the threshold, so at least one rejection is exercised.
    #[test]
    fn below_rejects_the_biased_zone() {
        let bound = (1_u64 << 63) + 1;
        let threshold = bound.wrapping_neg() % bound;
        let mut raw = SplitMix64::new(20_261_014);
        let mut rejected = 0;
        let expected = loop {
            let draw = raw.next_u64();
            if draw >= threshold {
                break draw % bound;
            }
            rejected += 1;
        };
        assert!(rejected > 0, "the seed must exercise a rejection");
        assert_eq!(SplitMix64::new(20_261_014).below(bound), expected);
    }

    #[test]
    fn a_permutation_keeps_every_item_and_depends_on_the_seed_alone() {
        let items: Vec<u32> = (0..100).collect();
        let first = permutation(&items, 42);
        let mut sorted = first.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, items);
        assert_eq!(first, permutation(&items, 42));
        assert_ne!(first, permutation(&items, 43));
        assert_ne!(first, items);
    }

    // Every one of the six orders of three items must be reachable: a shuffle that never
    // touches one position would pass the tests above and still be biased.
    #[test]
    fn every_order_of_three_items_occurs() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..200 {
            seen.insert(permutation(&[1, 2, 3], seed));
        }
        assert_eq!(seen.len(), 6, "{seen:?}");
    }

    #[test]
    fn the_empty_and_single_lists_are_left_alone() {
        assert_eq!(permutation::<u8>(&[], 1), Vec::<u8>::new());
        assert_eq!(permutation(&[9], 1), vec![9]);
    }
}
