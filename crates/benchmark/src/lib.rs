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

//! The benchmark's test: given the measures of two sets of worms, one row per worm, can a
//! classifier tell the sets apart?
//!
//! This is the classifier two-sample test that docs/PLAN.md § What "Indistinguishable" Will Mean
//! names ([Lopez-Paz and Oquab 2017](https://arxiv.org/abs/1610.06545)): pool the two samples,
//! label each row by the sample it came from, and measure how well a classifier predicts the
//! label of rows it was not trained on. If the samples come from one distribution, no classifier
//! can beat chance except by luck, and a permutation test says how much luck.
//!
//! The pieces, each chosen to be small and fully deterministic given a seed:
//!
//! - **Scaling.** Each measure is centred and divided by its standard deviation over the pooled
//!   rows. This never looks at the labels, so permuting the labels later leaves it unchanged.
//! - **The classifier.** A k-nearest-neighbour vote in the scaled space, Euclidean, with distance
//!   ties broken by each row's place in a seeded shuffle. Lopez-Paz and Oquab use k-nearest neighbours as one of their two
//!   classifiers; it has no training loop and no learning rate to tune.
//! - **Accuracy.** Cross-validated: the pooled rows are dealt into folds by a seeded shuffle that
//!   ignores the labels, and each row is classified by its neighbours in the other folds.
//! - **The p-value.** The labels are shuffled, by the same seeded generator, and the accuracy is
//!   recomputed; the p-value is (1 + the number of shuffles at least as accurate as the real
//!   labels) / (1 + the number of shuffles), which is never zero.
//! - **The decision.** The two samples are told apart when the p-value is at most `alpha`.
//!
//! Lopez-Paz and Oquab judge one held-out test set against a binomial null. Here every row is
//! classified once by cross-validation, and those predictions share training rows, so a binomial
//! null no longer applies, and no estimator of the variance of a cross-validated accuracy is
//! unbiased for every distribution ([Bengio and Grandvalet 2004](https://www.jmlr.org/papers/v5/grandvalet04a.html)). The
//! permutation null needs no such estimate: it is the accuracy's spread when the labels carry no
//! information, computed on the same folds.
//!
//! [`Config::default`] holds the values fixed for the lab's runs before any real data was seen
//! (journal of 2026-10-31, § The Benchmark's Test, Built Red-First). Freezing the benchmark
//! (docs/PLAN.md, Phase 0, step 5) is a separate step, which may still change them with a written
//! reason.

use std::fmt;

pub mod compare;

/// The test's settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    /// The number of neighbours that vote; odd, so a vote cannot tie.
    pub neighbours: usize,
    /// The number of cross-validation folds.
    pub folds: usize,
    /// The number of label shuffles behind the p-value.
    pub permutations: usize,
    /// The largest p-value at which the samples count as told apart.
    pub alpha: f64,
}

impl Default for Config {
    /// Twenty-one neighbours, five folds, 999 shuffles and alpha 0.05: fixed on synthetic samples,
    /// before any real data (the power table in the tests, and the journal of 2026-10-31,
    /// § The Benchmark's Test, Built Red-First).
    fn default() -> Self {
        Self {
            neighbours: 21,
            folds: 5,
            permutations: 999,
            alpha: 0.05,
        }
    }
}

/// What the test found.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    /// Rows in the first sample.
    pub rows_a: usize,
    /// Rows in the second sample.
    pub rows_b: usize,
    /// The cross-validated share of rows whose sample the classifier named correctly.
    pub accuracy: f64,
    /// The permutation p-value of that accuracy.
    pub p_value: f64,
    /// Whether the p-value is at most the configured alpha.
    pub told_apart: bool,
}

/// Why the test could not run.
#[derive(Clone, Debug, PartialEq)]
pub enum TestError {
    /// A setting is out of range; the text says which.
    BadConfig(&'static str),
    /// A sample has fewer rows than the folds need.
    TooFewRows { sample: char, rows: usize, needed: usize },
    /// The rows do not all have the same, non-zero number of measures.
    RaggedRows,
    /// A measure is NaN or infinite; a recording with a missing measure is excluded upstream.
    NotFinite { sample: char, row: usize, column: usize },
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadConfig(what) => write!(f, "bad configuration: {what}"),
            Self::TooFewRows { sample, rows, needed } => {
                write!(f, "sample {sample} has {rows} rows; at least {needed} are needed")
            }
            Self::RaggedRows => write!(f, "the rows do not all have the same, non-zero number of measures"),
            Self::NotFinite { sample, row, column } => {
                write!(f, "sample {sample}, row {row}, measure {column} is not a finite number")
            }
        }
    }
}

impl std::error::Error for TestError {}

/// SplitMix64 ([Steele, Lea and Flood 2014](https://doi.org/10.1145/2660193.2660195)): a small generator whose whole state is one integer,
/// so a seed fixes every shuffle.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A uniform index below `bound`, by rejection, so no index is favoured.
    fn below(&mut self, bound: usize) -> usize {
        let bound = bound as u64;
        let zone = u64::MAX - u64::MAX % bound;
        loop {
            let draw = self.next();
            if draw < zone {
                return (draw % bound) as usize;
            }
        }
    }

    /// Fisher-Yates.
    fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
    }
}

fn check(config: &Config, a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<usize, TestError> {
    if config.neighbours == 0 || config.neighbours.is_multiple_of(2) {
        return Err(TestError::BadConfig("neighbours must be odd"));
    }
    if config.folds < 2 {
        return Err(TestError::BadConfig("folds must be at least 2"));
    }
    if config.permutations == 0 {
        return Err(TestError::BadConfig("permutations must be at least 1"));
    }
    if !(config.alpha > 0.0 && config.alpha < 1.0) {
        return Err(TestError::BadConfig("alpha must lie strictly between 0 and 1"));
    }
    for (sample, rows) in [('a', a), ('b', b)] {
        if rows.len() < config.folds {
            return Err(TestError::TooFewRows {
                sample,
                rows: rows.len(),
                needed: config.folds,
            });
        }
    }
    let width = a[0].len();
    if width == 0 || a.iter().chain(b).any(|row| row.len() != width) {
        return Err(TestError::RaggedRows);
    }
    for (sample, rows) in [('a', a), ('b', b)] {
        for (row, values) in rows.iter().enumerate() {
            if let Some(column) = values.iter().position(|v| !v.is_finite()) {
                return Err(TestError::NotFinite { sample, row, column });
            }
        }
    }
    let pooled = a.len() + b.len();
    // A row's neighbours come from the other folds, which hold at least this many rows.
    let smallest_training = pooled - pooled.div_ceil(config.folds);
    if smallest_training < config.neighbours {
        return Err(TestError::BadConfig("too few rows outside a fold for the neighbours asked"));
    }
    Ok(width)
}

/// Every measure centred and divided by its standard deviation over all rows; a measure that
/// never varies becomes zero everywhere, carrying no information either way.
fn scaled(rows: &[&Vec<f64>], width: usize) -> Vec<Vec<f64>> {
    let n = rows.len() as f64;
    let mut out: Vec<Vec<f64>> = rows.iter().map(|r| r.to_vec()).collect();
    for c in 0..width {
        let mean = rows.iter().map(|r| r[c]).sum::<f64>() / n;
        let var = rows.iter().map(|r| (r[c] - mean).powi(2)).sum::<f64>() / n;
        let sd = var.sqrt();
        for row in &mut out {
            row[c] = if sd > 0.0 { (row[c] - mean) / sd } else { 0.0 };
        }
    }
    out
}

/// For every row, its `k` nearest rows among those in other folds, nearest first. Exact ties in
/// distance are broken by `rank`, each row's place in the seeded shuffle, never by row order: row
/// order puts every row of one sample before the other, and so would carry the labels.
fn neighbour_lists(points: &[Vec<f64>], fold: &[usize], rank: &[usize], k: usize) -> Vec<Vec<usize>> {
    let n = points.len();
    (0..n)
        .map(|i| {
            let mut others: Vec<(f64, usize)> = (0..n)
                .filter(|&j| fold[j] != fold[i])
                .map(|j| {
                    let d2: f64 = points[i].iter().zip(&points[j]).map(|(x, y)| (x - y).powi(2)).sum();
                    (d2, j)
                })
                .collect();
            others.sort_by(|x, y| x.0.total_cmp(&y.0).then(rank[x.1].cmp(&rank[y.1])));
            others.into_iter().take(k).map(|(_, j)| j).collect()
        })
        .collect()
}

fn accuracy(neighbours: &[Vec<usize>], labels: &[bool]) -> f64 {
    let correct = neighbours
        .iter()
        .zip(labels)
        .filter(|&(near, label)| {
            let votes_b = near.iter().filter(|&&j| labels[j]).count();
            (2 * votes_b > near.len()) == *label
        })
        .count();
    correct as f64 / labels.len() as f64
}

/// Runs the classifier two-sample test on samples `a` and `b`, one row of measures per worm.
///
/// The same inputs, settings and seed always give the same outcome.
pub fn two_sample_test(a: &[Vec<f64>], b: &[Vec<f64>], config: &Config, seed: u64) -> Result<Outcome, TestError> {
    let width = check(config, a, b)?;
    let rows: Vec<&Vec<f64>> = a.iter().chain(b).collect();
    let points = scaled(&rows, width);
    let mut labels: Vec<bool> = std::iter::repeat_n(false, a.len()).chain(std::iter::repeat_n(true, b.len())).collect();

    let mut rng = SplitMix64(seed);
    let mut order: Vec<usize> = (0..points.len()).collect();
    rng.shuffle(&mut order);
    let mut fold = vec![0; points.len()];
    let mut rank = vec![0; points.len()];
    for (position, &row) in order.iter().enumerate() {
        fold[row] = position % config.folds;
        rank[row] = position;
    }
    let near = neighbour_lists(&points, &fold, &rank, config.neighbours);

    let observed = accuracy(&near, &labels);
    let mut as_good = 0;
    for _ in 0..config.permutations {
        rng.shuffle(&mut labels);
        if accuracy(&near, &labels) >= observed {
            as_good += 1;
        }
    }
    let p_value = (1 + as_good) as f64 / (1 + config.permutations) as f64;
    Ok(Outcome {
        rows_a: a.len(),
        rows_b: b.len(),
        accuracy: observed,
        p_value,
        told_apart: p_value <= config.alpha,
    })
}

/// Helpers shared by the crate's tests: synthetic samples and an exact binomial interval.
#[cfg(test)]
mod test_support {
    use super::SplitMix64;

    /// Standard normal draws by the Box-Muller transform, from the test's own generator.
    pub(crate) fn normal(rng: &mut SplitMix64) -> f64 {
        let u1 = (rng.next() >> 11) as f64 / (1u64 << 53) as f64;
        let u2 = (rng.next() >> 11) as f64 / (1u64 << 53) as f64;
        (-2.0 * (1.0 - u1).ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }

    /// `rows` rows of `width` standard normal measures, the first scaled by `spread` and moved by `shift`.
    pub(crate) fn sample(rng: &mut SplitMix64, rows: usize, width: usize, shift: f64, spread: f64) -> Vec<Vec<f64>> {
        (0..rows)
            .map(|_| (0..width).map(|c| if c == 0 { normal(rng) * spread + shift } else { normal(rng) }).collect())
            .collect()
    }

    /// The exact (Clopper-Pearson) 95% interval of a binomial share, by bisection on the
    /// binomial distribution function.
    pub(crate) fn clopper_pearson(successes: usize, trials: usize) -> (f64, f64) {
        let cdf = |k: usize, p: f64| -> f64 {
            let mut term = (1.0 - p).powi(trials as i32);
            let mut total = term;
            for i in 0..k {
                term *= (trials - i) as f64 / (i + 1) as f64 * p / (1.0 - p);
                total += term;
            }
            total
        };
        let solve = |above: &dyn Fn(f64) -> bool| -> f64 {
            let (mut lo, mut hi) = (0.0, 1.0);
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                if above(mid) { hi = mid } else { lo = mid }
            }
            0.5 * (lo + hi)
        };
        let lower = if successes == 0 { 0.0 } else { solve(&|p| 1.0 - cdf(successes - 1, p) >= 0.025) };
        let upper = if successes == trials { 1.0 } else { solve(&|p| cdf(successes, p) <= 0.025) };
        (lower, upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{clopper_pearson, sample};

    /// Fewer shuffles than the lab's runs, so the suite stays fast in a debug build.
    fn quick() -> Config {
        Config {
            permutations: 199,
            ..Config::default()
        }
    }

    /// How many of `pairs` pairs of samples, 60 worms a side, the test tells apart.
    fn told_apart(pairs: u64, first_seed: u64, width: usize, shift: f64, spread: f64, config: &Config) -> usize {
        (0..pairs)
            .filter(|&seed| {
                let mut rng = SplitMix64(first_seed + seed);
                let a = sample(&mut rng, 60, width, 0.0, 1.0);
                let b = sample(&mut rng, 60, width, shift, spread);
                two_sample_test(&a, &b, config, seed).unwrap().told_apart
            })
            .count()
    }

    #[test]
    fn one_distribution_is_rarely_told_apart_from_itself() {
        // At alpha 0.05 about one pair in twenty is told apart by luck; four or more would happen
        // with probability about 0.016 if the test held its level exactly (binomial, 20, 0.05).
        let count = told_apart(20, 1000, 4, 0.0, 1.0, &quick());
        assert!(count <= 3, "{count} of 20 identical pairs told apart");
    }

    #[test]
    fn a_shift_of_one_and_a_half_standard_deviations_is_told_apart() {
        // One measure of four moved by 1.5 standard deviations: the power table puts the test's
        // power there near 1, so every one of ten pairs must be told apart.
        let count = told_apart(10, 2000, 4, 1.5, 1.0, &quick());
        assert_eq!(count, 10, "{count} of 10 shifted pairs told apart");
    }

    #[test]
    fn the_same_seed_gives_the_same_outcome() {
        let mut rng = SplitMix64(3000);
        let a = sample(&mut rng, 40, 3, 0.0, 1.0);
        let b = sample(&mut rng, 40, 3, 0.3, 1.0);
        let first = two_sample_test(&a, &b, &quick(), 7).unwrap();
        assert_eq!(first, two_sample_test(&a, &b, &quick(), 7).unwrap());
        assert!(first.p_value > 0.0 && first.p_value <= 1.0);
    }

    #[test]
    fn a_measure_that_never_varies_is_ignored() {
        // Adding a measure that is the same for every worm must change nothing at all.
        let mut rng = SplitMix64(4000);
        let a = sample(&mut rng, 30, 2, 0.0, 1.0);
        let b = sample(&mut rng, 30, 2, 0.5, 1.0);
        let without = two_sample_test(&a, &b, &quick(), 1).unwrap();
        let widen = |rows: &[Vec<f64>]| -> Vec<Vec<f64>> { rows.iter().map(|r| r.iter().copied().chain([5.0]).collect()).collect() };
        assert_eq!(two_sample_test(&widen(&a), &widen(&b), &quick(), 1).unwrap(), without);
    }

    #[test]
    fn bad_inputs_are_refused() {
        let row = vec![0.0, 1.0];
        let five = vec![row.clone(); 5];
        let config = Config::default();
        assert_eq!(
            two_sample_test(&five[..4], &five, &config, 0),
            Err(TestError::TooFewRows { sample: 'a', rows: 4, needed: 5 })
        );
        let mut ragged = five.clone();
        ragged[2] = vec![0.0];
        assert_eq!(two_sample_test(&five, &ragged, &config, 0), Err(TestError::RaggedRows));
        let mut missing = five.clone();
        missing[3][1] = f64::NAN;
        assert_eq!(
            two_sample_test(&five, &missing, &config, 0),
            Err(TestError::NotFinite { sample: 'b', row: 3, column: 1 })
        );
        assert!(matches!(two_sample_test(&five, &five, &config, 0), Err(TestError::BadConfig(_))));
        let even = Config { neighbours: 4, ..config };
        assert!(matches!(two_sample_test(&five, &five, &even, 0), Err(TestError::BadConfig(_))));
        let level = Config { alpha: 1.0, ..config };
        assert!(matches!(two_sample_test(&five, &five, &level, 0), Err(TestError::BadConfig(_))));
    }

    #[test]
    fn a_row_is_judged_only_by_rows_in_other_folds() {
        let mut rng = SplitMix64(5000);
        let points = sample(&mut rng, 23, 2, 0.0, 1.0);
        let fold: Vec<usize> = (0..points.len()).map(|i| i % 5).collect();
        let rank: Vec<usize> = (0..points.len()).collect();
        for (i, near) in neighbour_lists(&points, &fold, &rank, 7).iter().enumerate() {
            assert_eq!(near.len(), 7);
            assert!(near.iter().all(|&j| j != i && fold[j] != fold[i]), "row {i}: {near:?}");
        }
    }

    #[test]
    fn the_smallest_p_value_is_one_over_one_plus_the_shuffles() {
        // Two samples ten standard deviations apart: no shuffle can match the real labels, so the
        // p-value is the formula's floor, never zero.
        let mut rng = SplitMix64(6000);
        let a = sample(&mut rng, 30, 2, 0.0, 1.0);
        let b = sample(&mut rng, 30, 2, 10.0, 1.0);
        let config = Config {
            permutations: 19,
            ..Config::default()
        };
        let outcome = two_sample_test(&a, &b, &config, 3).unwrap();
        assert_eq!(outcome.accuracy, 1.0);
        assert_eq!(outcome.p_value, 1.0 / 20.0);
    }

    #[test]
    fn unequal_samples_from_one_distribution_are_rarely_told_apart() {
        // 30 worms against 90: chance accuracy is the larger share, 0.75, not 0.5, and the
        // permutation null must absorb it. The same bound as for equal samples applies.
        let count = (0..20u64)
            .filter(|&seed| {
                let mut rng = SplitMix64(7000 + seed);
                let a = sample(&mut rng, 30, 3, 0.0, 1.0);
                let b = sample(&mut rng, 90, 3, 0.0, 1.0);
                two_sample_test(&a, &b, &quick(), seed).unwrap().told_apart
            })
            .count();
        assert!(count <= 3, "{count} of 20 unequal identical pairs told apart");
    }

    /// The table behind the choice of 21 neighbours, over 100 pairs per cell, 60 worms a side:
    /// `cargo test --release -p benchmark power_table -- --ignored --nocapture`.
    #[test]
    #[ignore = "a table for the journal, not a check; run by hand in release"]
    fn power_table() {
        let cases: [(&str, usize, f64, f64); 4] = [
            ("same distribution, 4 measures", 4, 0.0, 1.0),
            ("1 sd shift in 1 of 4", 4, 1.0, 1.0),
            ("1 sd shift in 1 of 8", 8, 1.0, 1.0),
            ("spread x1.6 in 1 of 4", 4, 0.0, 1.6),
        ];
        for k in [5, 11, 21, 31] {
            let config = Config { neighbours: k, ..quick() };
            for (name, width, shift, spread) in cases {
                let count = told_apart(100, 9000, width, shift, spread, &config);
                let (lower, upper) = clopper_pearson(count, 100);
                println!("k {k:2}  {name:30} {count:3}/100  ({:.0}% to {:.0}%)", 100.0 * lower, 100.0 * upper);
            }
        }
    }
}
