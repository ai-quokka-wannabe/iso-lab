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

//! The per-measure comparisons beside the classifier: for each measure on its own, do the two
//! sets of worms differ, and how?
//!
//! docs/PLAN.md § What "Indistinguishable" Will Mean asks for them: "Per-feature comparisons of
//! the distributions say where they differ when they do." The classifier test in the crate root
//! sees a shifted centre well and a changed spread poorly (journal of 2026-10-31, § The
//! Benchmark's Test, Built Red-First), so these compare whole distributions, not only centres.
//!
//! For each measure:
//!
//! - **The statistic** is the two-sample Cramér-von Mises statistic: the squared gap between the
//!   two samples' empirical distribution functions, summed over every pooled value. It responds to
//!   any difference in shape - centre, spread or tails - and was chosen over the Kolmogorov-Smirnov
//!   statistic on synthetic samples (journal of 2026-10-31, § The Per-Measure Comparisons).
//! - **The p-value** comes from shuffling which worm belongs to which set, by a seeded generator,
//!   as in the classifier test: (1 + shuffles at least as large) / (1 + shuffles). Ties in the
//!   data need no correction, because the shuffles keep them.
//! - **Many measures at once** are corrected by Holm's step-down method
//!   ([Holm 1979](https://www.jstor.org/stable/4615733)), which holds the chance of any false
//!   "differs" at alpha across all the measures, under any dependence between them.
//! - **How they differ** is described by each sample's median and interquartile range, so a reader
//!   sees whether a difference is one of centre or of spread.

use crate::{SplitMix64, TestError};

/// The comparisons' settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompareConfig {
    /// The number of shuffles behind each measure's p-value.
    pub permutations: usize,
    /// The largest Holm-adjusted p-value at which a measure counts as differing.
    pub alpha: f64,
}

impl Default for CompareConfig {
    /// 999 shuffles and alpha 0.05, as for the classifier test.
    fn default() -> Self {
        Self { permutations: 999, alpha: 0.05 }
    }
}

/// One measure's comparison.
#[derive(Clone, Debug, PartialEq)]
pub struct MeasureComparison {
    /// The measure's column in the rows.
    pub column: usize,
    /// Anderson's T for the real split into the two sets.
    pub statistic: f64,
    /// The permutation p-value, before correction for the number of measures.
    pub p_value: f64,
    /// The Holm-adjusted p-value.
    pub adjusted_p: f64,
    /// Whether the adjusted p-value is at most alpha.
    pub differs: bool,
    /// The first sample's median.
    pub median_a: f64,
    /// The first sample's interquartile range.
    pub iqr_a: f64,
    /// The second sample's median.
    pub median_b: f64,
    /// The second sample's interquartile range.
    pub iqr_b: f64,
}

/// Which statistic compares two distribution functions. The lab uses Cramér-von Mises; the
/// Kolmogorov-Smirnov statistic is kept only so the power table that chose between them can be re-run.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Statistic {
    /// Kolmogorov and Smirnov: the largest gap between the two functions.
    #[cfg(test)]
    KolmogorovSmirnov,
    /// Cramér and von Mises (Anderson 1962): the squared gap, summed over the pooled values.
    CramerVonMises,
}

fn check(config: &CompareConfig, a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<usize, TestError> {
    if config.permutations == 0 {
        return Err(TestError::BadConfig("permutations must be at least 1"));
    }
    if !(config.alpha > 0.0 && config.alpha < 1.0) {
        return Err(TestError::BadConfig("alpha must lie strictly between 0 and 1"));
    }
    for (sample, rows) in [('a', a), ('b', b)] {
        if rows.len() < 2 {
            return Err(TestError::TooFewRows {
                sample,
                rows: rows.len(),
                needed: 2,
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
    Ok(width)
}

/// The pooled values of one measure, sorted, as runs of equal values: each run is the rows it holds.
fn runs_of_equal_values(values: &[f64]) -> Vec<Vec<usize>> {
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|&x, &y| values[x].total_cmp(&values[y]).then(x.cmp(&y)));
    let mut runs: Vec<Vec<usize>> = Vec::new();
    for row in order {
        match runs.last_mut() {
            Some(run) if values[run[0]] == values[row] => run.push(row),
            _ => runs.push(vec![row]),
        }
    }
    runs
}

/// The statistic for one labelling, exactly, as an integer: `in_b[row]` says which set a pooled row
/// belongs to. With `n_a` and `n_b` rows, the gap between the two distribution functions after a
/// run is `(seen_a * n_b - seen_b * n_a) / (n_a * n_b)`; its numerator is an integer, so summing
/// `len * numerator^2` in integers keeps two labellings with equal statistics exactly equal. A sum
/// in floating point would not: added in another order, equal statistics can differ in the last
/// bit, and a shuffle as large as the real labelling would then be missed by `>=`.
fn score(runs: &[Vec<usize>], in_b: &[bool], count_b: usize, which: Statistic) -> u128 {
    let count_a = in_b.len() - count_b;
    let (mut seen_a, mut seen_b) = (0u128, 0u128);
    let mut total = 0u128;
    for run in runs {
        let b_here = run.iter().filter(|&&row| in_b[row]).count() as u128;
        seen_b += b_here;
        seen_a += run.len() as u128 - b_here;
        let numerator = (seen_a * count_b as u128).abs_diff(seen_b * count_a as u128);
        total = match which {
            #[cfg(test)]
            Statistic::KolmogorovSmirnov => total.max(numerator),
            Statistic::CramerVonMises => total + run.len() as u128 * numerator * numerator,
        };
    }
    total
}

/// Anderson's T from the exact score: the sum of `len * gap^2`, times `n_a * n_b / N^2`, which is
/// the score divided by `n_a * n_b * N^2`. Used only to report the statistic, never to compare.
fn anderson_t(score: u128, count_a: usize, count_b: usize) -> f64 {
    let pooled = (count_a + count_b) as f64;
    score as f64 / (count_a as f64 * count_b as f64 * pooled * pooled)
}

/// The value below which a share `q` of the sorted values lie, by linear interpolation between
/// order statistics (the usual "type 7" rule).
fn quantile(sorted: &[f64], q: f64) -> f64 {
    let position = q * (sorted.len() - 1) as f64;
    let below = position.floor() as usize;
    let above = position.ceil() as usize;
    sorted[below] + (position - below as f64) * (sorted[above] - sorted[below])
}

fn median_and_iqr(rows: &[Vec<f64>], column: usize) -> (f64, f64) {
    let mut values: Vec<f64> = rows.iter().map(|r| r[column]).collect();
    values.sort_by(f64::total_cmp);
    (quantile(&values, 0.5), quantile(&values, 0.75) - quantile(&values, 0.25))
}

/// Holm's step-down adjustment of a family of p-values, returned in their original order.
fn holm(p_values: &[f64]) -> Vec<f64> {
    let m = p_values.len();
    let mut order: Vec<usize> = (0..m).collect();
    order.sort_by(|&x, &y| p_values[x].total_cmp(&p_values[y]).then(x.cmp(&y)));
    let mut adjusted = vec![0.0; m];
    let mut running = 0.0f64;
    for (step, &index) in order.iter().enumerate() {
        running = running.max(((m - step) as f64 * p_values[index]).min(1.0));
        adjusted[index] = running;
    }
    adjusted
}

/// Compares samples `a` and `b`, one row of measures per worm, measure by measure, by the
/// two-sample Cramér-von Mises statistic ([Anderson 1962](https://doi.org/10.1214/aoms/1177704477)).
/// The same inputs, settings and seed always give the same comparisons.
pub fn compare_measures(a: &[Vec<f64>], b: &[Vec<f64>], config: &CompareConfig, seed: u64) -> Result<Vec<MeasureComparison>, TestError> {
    compare_measures_with(a, b, config, seed, Statistic::CramerVonMises)
}

fn compare_measures_with(a: &[Vec<f64>], b: &[Vec<f64>], config: &CompareConfig, seed: u64, which: Statistic) -> Result<Vec<MeasureComparison>, TestError> {
    let width = check(config, a, b)?;
    let mut rng = SplitMix64(seed);
    let mut raw = Vec::with_capacity(width);
    for column in 0..width {
        let values: Vec<f64> = a.iter().chain(b).map(|r| r[column]).collect();
        let runs = runs_of_equal_values(&values);
        let mut in_b: Vec<bool> = std::iter::repeat_n(false, a.len()).chain(std::iter::repeat_n(true, b.len())).collect();
        let observed = score(&runs, &in_b, b.len(), which);
        let mut as_large = 0;
        for _ in 0..config.permutations {
            rng.shuffle(&mut in_b);
            if score(&runs, &in_b, b.len(), which) >= observed {
                as_large += 1;
            }
        }
        raw.push((anderson_t(observed, a.len(), b.len()), (1 + as_large) as f64 / (1 + config.permutations) as f64));
    }
    let adjusted = holm(&raw.iter().map(|r| r.1).collect::<Vec<_>>());
    Ok((0..width)
        .map(|column| {
            let (median_a, iqr_a) = median_and_iqr(a, column);
            let (median_b, iqr_b) = median_and_iqr(b, column);
            MeasureComparison {
                column,
                statistic: raw[column].0,
                p_value: raw[column].1,
                adjusted_p: adjusted[column],
                differs: adjusted[column] <= config.alpha,
                median_a,
                iqr_a,
                median_b,
                iqr_b,
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{clopper_pearson, sample};

    fn quick() -> CompareConfig {
        CompareConfig {
            permutations: 199,
            ..CompareConfig::default()
        }
    }

    /// How many of `pairs` pairs, 60 worms a side and one measure, are found to differ.
    fn differs(pairs: u64, first_seed: u64, shift: f64, spread: f64, which: Statistic) -> usize {
        (0..pairs)
            .filter(|&seed| {
                let mut rng = SplitMix64(first_seed + seed);
                let a = sample(&mut rng, 60, 1, 0.0, 1.0);
                let b = sample(&mut rng, 60, 1, shift, spread);
                compare_measures_with(&a, &b, &quick(), seed, which).unwrap()[0].differs
            })
            .count()
    }

    #[test]
    fn one_distribution_rarely_differs_from_itself() {
        // The same bound as the classifier test's: four or more of twenty would happen with
        // probability about 0.016 if the comparison held its level exactly.
        let count = (0..20u64)
            .filter(|&seed| {
                let mut rng = SplitMix64(1000 + seed);
                let a = sample(&mut rng, 60, 1, 0.0, 1.0);
                let b = sample(&mut rng, 60, 1, 0.0, 1.0);
                compare_measures(&a, &b, &quick(), seed).unwrap()[0].differs
            })
            .count();
        assert!(count <= 3, "{count} of 20 identical pairs differ");
    }

    #[test]
    fn a_tripled_spread_with_the_same_centre_differs() {
        // The change the classifier sees worst: every one of ten pairs must be flagged.
        for seed in 0..10u64 {
            let mut rng = SplitMix64(2000 + seed);
            let a = sample(&mut rng, 60, 1, 0.0, 1.0);
            let b = sample(&mut rng, 60, 1, 0.0, 3.0);
            let found = &compare_measures(&a, &b, &quick(), seed).unwrap()[0];
            assert!(found.differs, "seed {seed}: {found:?}");
            assert!(found.iqr_b > found.iqr_a, "seed {seed}: {found:?}");
        }
    }

    #[test]
    fn only_the_measure_that_changed_is_flagged() {
        let mut rng = SplitMix64(3000);
        let a = sample(&mut rng, 80, 3, 0.0, 1.0);
        let b = sample(&mut rng, 80, 3, 2.0, 1.0);
        let found: Vec<bool> = compare_measures(&a, &b, &quick(), 5).unwrap().iter().map(|c| c.differs).collect();
        assert_eq!(found, vec![true, false, false]);
    }

    #[test]
    fn twenty_unchanged_measures_rarely_flag_any() {
        // Holm holds the chance of any false "differs" across all twenty measures at alpha. Without
        // the correction about two in three comparisons would flag one; with it, three or more of
        // ten would happen with probability about 0.012 (binomial, 10, 0.05).
        let flagged = (0..10u64)
            .filter(|&seed| {
                let mut rng = SplitMix64(8000 + seed);
                let a = sample(&mut rng, 40, 20, 0.0, 1.0);
                let b = sample(&mut rng, 40, 20, 0.0, 1.0);
                compare_measures(&a, &b, &quick(), seed).unwrap().iter().any(|c| c.differs)
            })
            .count();
        assert!(flagged <= 2, "{flagged} of 10 comparisons flagged an unchanged measure");
    }

    #[test]
    fn identical_values_give_no_gap() {
        let rows = vec![vec![2.0]; 10];
        let found = &compare_measures(&rows, &rows, &quick(), 1).unwrap()[0];
        assert_eq!(found.statistic, 0.0);
        assert_eq!(found.p_value, 1.0);
        assert!(!found.differs);
    }

    #[test]
    fn the_same_seed_gives_the_same_comparisons() {
        let mut rng = SplitMix64(4000);
        let a = sample(&mut rng, 30, 2, 0.0, 1.0);
        let b = sample(&mut rng, 30, 2, 0.4, 1.2);
        assert_eq!(compare_measures(&a, &b, &quick(), 9).unwrap(), compare_measures(&a, &b, &quick(), 9).unwrap());
    }

    #[test]
    fn holm_adjusts_in_order_and_keeps_the_original_order() {
        // Sorted: 0.01 x4 = 0.04; 0.03 x3 = 0.09; 0.04 x2 = 0.08, raised to 0.09; 0.2 x1 = 0.2.
        let adjusted = holm(&[0.01, 0.04, 0.03, 0.2]);
        let expected = [0.04, 0.09, 0.09, 0.2];
        for (got, want) in adjusted.iter().zip(expected) {
            assert!((got - want).abs() < 1e-12, "{adjusted:?}");
        }
    }

    #[test]
    fn holm_caps_at_one() {
        assert_eq!(holm(&[0.6, 0.7]), vec![1.0, 1.0]);
    }

    #[test]
    fn a_mirrored_measure_scores_exactly_the_same() {
        // Negating every value reverses the order of the runs, so the same terms are summed in the
        // opposite order; the score must not change by a single bit.
        for seed in 0..20u64 {
            let mut rng = SplitMix64(9500 + seed);
            let values: Vec<f64> = sample(&mut rng, 70, 1, 0.0, 1.0).into_iter().map(|r| r[0]).collect();
            let mirrored: Vec<f64> = values.iter().map(|v| -v).collect();
            let mut in_b: Vec<bool> = (0..values.len()).map(|i| i >= 31).collect();
            rng.shuffle(&mut in_b);
            let forward = score(&runs_of_equal_values(&values), &in_b, 39, Statistic::CramerVonMises);
            let backward = score(&runs_of_equal_values(&mirrored), &in_b, 39, Statistic::CramerVonMises);
            assert_eq!(forward, backward, "seed {seed}");
        }
    }

    #[test]
    fn the_reported_statistic_is_andersons_t() {
        // a = {1, 2}, b = {3, 4}: the gaps after each value are 1/2, 1, 1/2, 0, so T = (4/16)
        // x (1/4 + 1 + 1/4) = 0.375.
        let a = vec![vec![1.0], vec![2.0]];
        let b = vec![vec![3.0], vec![4.0]];
        assert_eq!(compare_measures(&a, &b, &quick(), 0).unwrap()[0].statistic, 0.375);
    }

    #[test]
    fn quartiles_interpolate_between_order_statistics() {
        let rows: Vec<Vec<f64>> = [4.0, 1.0, 3.0, 2.0].iter().map(|&v| vec![v]).collect();
        assert_eq!(median_and_iqr(&rows, 0), (2.5, 1.5));
    }

    #[test]
    fn bad_inputs_are_refused() {
        let rows = vec![vec![0.0, 1.0]; 3];
        assert!(matches!(
            compare_measures(&rows[..1], &rows, &quick(), 0),
            Err(TestError::TooFewRows { sample: 'a', .. })
        ));
        let mut missing = rows.clone();
        missing[2][0] = f64::INFINITY;
        assert!(matches!(compare_measures(&rows, &missing, &quick(), 0), Err(TestError::NotFinite { .. })));
        let zero = CompareConfig { permutations: 0, ..quick() };
        assert!(matches!(compare_measures(&rows, &rows, &zero, 0), Err(TestError::BadConfig(_))));
    }

    /// The level check behind the journal's deciles: 1000 pairs of one distribution (seeds 50000 to
    /// 50999, shuffle seeds 0 to 999, 199 shuffles), Cramér-von Mises, with p binned by floor(10p):
    /// `cargo test --release -p benchmark compare::tests::level_deciles -- --ignored --nocapture`.
    #[test]
    #[ignore = "a table for the journal, not a check; run by hand in release"]
    fn level_deciles() {
        let mut deciles = [0usize; 10];
        let mut flagged = 0;
        for seed in 0..1000u64 {
            let mut rng = SplitMix64(50_000 + seed);
            let a = sample(&mut rng, 60, 1, 0.0, 1.0);
            let b = sample(&mut rng, 60, 1, 0.0, 1.0);
            let found = &compare_measures(&a, &b, &quick(), seed).unwrap()[0];
            flagged += usize::from(found.differs);
            deciles[((found.p_value * 10.0) as usize).min(9)] += 1;
        }
        println!("flagged {flagged}/1000; p-value deciles {deciles:?}");
    }

    /// The table behind the choice of statistic:
    /// `cargo test --release -p benchmark compare::tests::power_table -- --ignored --nocapture`.
    #[test]
    #[ignore = "a table for the journal, not a check; run by hand in release"]
    fn power_table() {
        let cases: [(&str, f64, f64); 5] = [
            ("same distribution", 0.0, 1.0),
            ("0.5 sd shift", 0.5, 1.0),
            ("spread x1.6", 0.0, 1.6),
            ("spread x2", 0.0, 2.0),
            ("0.3 sd shift, spread x1.3", 0.3, 1.3),
        ];
        for which in [Statistic::KolmogorovSmirnov, Statistic::CramerVonMises] {
            for (name, shift, spread) in cases {
                // The level gets ten times the pairs, from seeds of its own, to pin it more closely.
                let (pairs, first_seed) = if shift == 0.0 && spread == 1.0 { (1000, 50_000) } else { (100, 9000) };
                let count = differs(pairs, first_seed, shift, spread, which);
                let (lower, upper) = clopper_pearson(count, pairs as usize);
                println!("{which:?} {name:28} {count:4}/{pairs}  ({:.1}% to {:.1}%)", 100.0 * lower, 100.0 * upper);
            }
        }
    }
}
