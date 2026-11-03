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

//! The reference statistics (STATE.md item 2 (d2)): each measure's median and interquartile range
//! across the real worms, and a percentile bootstrap interval for the median
//! (Efron and Tibshirani 1993, [ch. 13, *Confidence intervals based on bootstrap percentiles*](https://doi.org/10.1007/978-1-4899-4541-9_13)).
//!
//! The bootstrap resamples whole rows - one row per recording - with replacement, by the crate's
//! seeded generator, so one seed fixes every resample. The same resamples serve every measure,
//! which keeps the measures' intervals drawn from one set of pseudo-populations;
//! [`bootstrap_draws`] hands them out so that other statistics (the eigen-shapes' variance shares)
//! can be resampled the same way. Quantiles are the usual linear interpolation between order
//! statistics (type 7), as in [`crate::compare`].

use crate::SplitMix64;
use crate::compare::quantile;

/// The quantiles of the resampled statistics that bound every reported interval: 2.5% and 97.5%, a
/// 95% percentile interval. The one place the level is written; [`summarise`] and the
/// `reference-stats` binary both read it.
pub const INTERVAL_TAILS: (f64, f64) = (0.025, 0.975);

/// One measure's statistics across the rows.
#[derive(Clone, Debug, PartialEq)]
pub struct Summary {
    /// How many rows the measure was computed over.
    pub rows: usize,
    pub median: f64,
    /// The first and third quartiles.
    pub quartiles: (f64, f64),
    /// The percentile bootstrap interval for the median: the 2.5% and 97.5% quantiles of the
    /// resamples' medians.
    pub median_interval: (f64, f64),
}

impl Summary {
    /// The interquartile range.
    pub fn iqr(&self) -> f64 {
        self.quartiles.1 - self.quartiles.0
    }
}

/// `resamples` bootstrap resamples of `rows` row indices, each `rows` long and drawn uniformly with
/// replacement, from a generator seeded by `seed`.
pub fn bootstrap_draws(rows: usize, resamples: usize, seed: u64) -> Vec<Vec<usize>> {
    let mut rng = SplitMix64(seed);
    (0..resamples).map(|_| (0..rows).map(|_| rng.below(rows)).collect()).collect()
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    quantile(&values, 0.5)
}

/// Each column's [`Summary`] over `rows`, the median's interval from `resamples` bootstrap
/// resamples seeded by `seed`. `None` when there are fewer than two rows, when the rows are ragged
/// or when any value is not finite: a gap is the caller's to decide, never dropped here.
pub fn summarise(rows: &[Vec<f64>], resamples: usize, seed: u64) -> Option<Vec<Summary>> {
    let width = rows.first()?.len();
    if rows.len() < 2 || resamples == 0 || rows.iter().any(|row| row.len() != width || row.iter().any(|v| !v.is_finite())) {
        return None;
    }
    let draws = bootstrap_draws(rows.len(), resamples, seed);
    Some(
        (0..width)
            .map(|column| {
                let mut values: Vec<f64> = rows.iter().map(|row| row[column]).collect();
                values.sort_by(f64::total_cmp);
                let mut medians: Vec<f64> = draws.iter().map(|draw| median(draw.iter().map(|&i| rows[i][column]).collect())).collect();
                medians.sort_by(f64::total_cmp);
                Summary {
                    rows: rows.len(),
                    median: quantile(&values, 0.5),
                    quartiles: (quantile(&values, 0.25), quantile(&values, 0.75)),
                    median_interval: (quantile(&medians, INTERVAL_TAILS.0), quantile(&medians, INTERVAL_TAILS.1)),
                }
            })
            .collect(),
    )
}

/// The ranks of `values`, 1 for the smallest, tied values sharing the mean of their ranks.
fn ranks(values: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|&a, &b| values[a].total_cmp(&values[b]).then(a.cmp(&b)));
    let mut ranks = vec![0.0; values.len()];
    let mut start = 0;
    while start < order.len() {
        let mut end = start + 1;
        while end < order.len() && values[order[end]] == values[order[start]] {
            end += 1;
        }
        // Positions start..end hold equal values: ranks start + 1 ..= end, whose mean is this.
        let shared = (start + end + 1) as f64 / 2.0;
        for &index in &order[start..end] {
            ranks[index] = shared;
        }
        start = end;
    }
    ranks
}

/// Spearman's rank correlation of `x` and `y`: Pearson's correlation of their ranks, ties given
/// mean ranks. `None` when the lengths differ, there are fewer than three pairs, a value is not
/// finite, or either side's ranks do not vary.
pub fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.len() < 3 || x.iter().chain(y).any(|v| !v.is_finite()) {
        return None;
    }
    let (rx, ry) = (ranks(x), ranks(y));
    let n = x.len() as f64;
    let (mx, my) = (rx.iter().sum::<f64>() / n, ry.iter().sum::<f64>() / n);
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for (a, b) in rx.iter().zip(&ry) {
        sxy += (a - mx) * (b - my);
        sxx += (a - mx) * (a - mx);
        syy += (b - my) * (b - my);
    }
    (sxx > 0.0 && syy > 0.0).then(|| sxy / (sxx * syy).sqrt())
}

/// Spearman's correlation of `x` and `y`, with its percentile bootstrap interval at
/// [`INTERVAL_TAILS`] from `resamples` resamples of the pairs seeded by `seed`, and how many
/// resamples gave no correlation (a side without variation) and were left out.
pub fn spearman_interval(x: &[f64], y: &[f64], resamples: usize, seed: u64) -> Option<(f64, (f64, f64), usize)> {
    let rho = spearman(x, y)?;
    let mut values = Vec::with_capacity(resamples);
    let mut undefined = 0;
    for draw in bootstrap_draws(x.len(), resamples, seed) {
        let xs: Vec<f64> = draw.iter().map(|&i| x[i]).collect();
        let ys: Vec<f64> = draw.iter().map(|&i| y[i]).collect();
        match spearman(&xs, &ys) {
            Some(value) => values.push(value),
            None => undefined += 1,
        }
    }
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    Some((rho, (quantile(&values, INTERVAL_TAILS.0), quantile(&values, INTERVAL_TAILS.1)), undefined))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quartiles_of_one_to_nine() {
        // 1..=9: the median 5, the quartiles 3 and 7 by type 7.
        let rows: Vec<Vec<f64>> = (1..=9).map(|v| vec![f64::from(v)]).collect();
        let summary = &summarise(&rows, 99, 1).unwrap()[0];
        assert_eq!((summary.rows, summary.median, summary.quartiles), (9, 5.0, (3.0, 7.0)));
        assert_eq!(summary.iqr(), 4.0);
    }

    #[test]
    fn a_constant_measure_has_a_point_interval() {
        let rows = vec![vec![2.5, 1.0]; 20];
        let summaries = summarise(&rows, 199, 7).unwrap();
        assert_eq!(summaries[0].median_interval, (2.5, 2.5));
        assert_eq!(summaries[1].median_interval, (1.0, 1.0));
    }

    #[test]
    fn the_interval_holds_the_median_and_narrows_with_more_rows() {
        // 0, 1, ..., n - 1 shuffled into no particular order by a fixed stride.
        let interval = |n: usize| {
            let rows: Vec<Vec<f64>> = (0..n).map(|i| vec![((i * 37) % n) as f64 / n as f64]).collect();
            let summary = summarise(&rows, 1999, 11).unwrap().remove(0);
            assert!(
                summary.median_interval.0 <= summary.median && summary.median <= summary.median_interval.1,
                "{summary:?}"
            );
            summary.median_interval.1 - summary.median_interval.0
        };
        let (narrow, wide) = (interval(400), interval(25));
        assert!(narrow < wide / 2.0, "400 rows: {narrow}, 25 rows: {wide}");
        // A uniform on [0, 1): the median's standard error is about 1 / (2 sqrt(n)), so the 95%
        // interval spans about 3.92 / (2 sqrt(400)) = 0.098 at 400 rows, and a 90% interval about
        // 3.29 / 40 = 0.082 (normal-theory widths for a continuous uniform, so approximate), so the
        // window admits the first and not the second.
        assert!((0.088..0.11).contains(&narrow), "{narrow}");
    }

    #[test]
    fn the_level_is_95_percent_and_the_interval_reads_it() {
        // The level is what a reader takes from every interval. It is written once, so it is
        // asserted once - a typo such as 0.035 fails here, whatever the data.
        assert_eq!(INTERVAL_TAILS, (0.025, 0.975));
        // And `summarise` reads it, which its behaviour pins: on 1000 distinct rows, 999 resamples
        // and seed 5, the order statistics bracketing both ends do not tie, so the interval is the
        // resampled medians' 2.5% and 97.5% quantiles and no nearby level's. (On a handful of rows
        // they tie, and neighbouring levels share one interval.)
        let rows: Vec<Vec<f64>> = (0..1000u32).map(|i| vec![f64::from((i * 37) % 1000) / 1000.0 + f64::from(i).sqrt() * 1e-7]).collect();
        let (resamples, seed) = (999, 5);
        let mut medians: Vec<f64> = bootstrap_draws(rows.len(), resamples, seed)
            .iter()
            .map(|draw| median(draw.iter().map(|&i| rows[i][0]).collect()))
            .collect();
        medians.sort_by(f64::total_cmp);
        // Type 7 at 2.5% interpolates between order statistics 24 and 25 (h = 998 x 0.025 = 24.95),
        // at 97.5% between 973 and 974.
        assert!(medians[24] < medians[25] && medians[973] < medians[974]);
        let interval = summarise(&rows, resamples, seed).unwrap()[0].median_interval;
        assert_eq!(interval, (quantile(&medians, 0.025), quantile(&medians, 0.975)));
        for low in [0.02, 0.024, 0.0249, 0.0251, 0.026, 0.03, 0.035, 0.05] {
            assert_ne!(interval.0, quantile(&medians, low), "lower end at {low}");
            assert_ne!(interval.1, quantile(&medians, 1.0 - low), "upper end at {}", 1.0 - low);
        }
    }

    #[test]
    fn draws_cover_every_row_and_follow_the_seed() {
        let draws = bootstrap_draws(10, 500, 3);
        assert_eq!(draws, bootstrap_draws(10, 500, 3));
        assert_ne!(draws, bootstrap_draws(10, 500, 4));
        assert!(draws.iter().all(|draw| draw.len() == 10 && draw.iter().all(|&i| i < 10)));
        let mut seen = [0usize; 10];
        draws.iter().flatten().for_each(|&i| seen[i] += 1);
        // 5000 draws, 500 expected per row; a fair generator stays well within 400 to 600.
        assert!(seen.iter().all(|&count| (400..600).contains(&count)), "{seen:?}");
    }

    #[test]
    fn a_gap_or_too_few_rows_give_nothing() {
        assert_eq!(summarise(&[vec![1.0], vec![f64::NAN]], 9, 1), None);
        assert_eq!(summarise(&[vec![1.0]], 9, 1), None);
        assert_eq!(summarise(&[vec![1.0], vec![1.0, 2.0]], 9, 1), None);
    }

    #[test]
    fn spearman_of_known_orders() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(spearman(&x, &[10.0, 20.0, 30.0, 40.0, 50.0]), Some(1.0));
        assert_eq!(spearman(&x, &[5.0, 4.0, 3.0, 2.0, 1.0]), Some(-1.0));
        // Monotone but not linear: still 1.
        assert_eq!(spearman(&x, &[1.0, 8.0, 27.0, 64.0, 1000.0]), Some(1.0));
        // A hand-worked case with a tie: ranks of y are 1, 2.5, 2.5, 4, 5 against 1-5, so
        // rho = sum (rx - 3)(ry - 3) / sqrt(10 * 9.5) = 9.5 / sqrt(95).
        let rho = spearman(&x, &[0.0, 1.0, 1.0, 2.0, 3.0]).unwrap();
        assert!((rho - 9.5 / 95.0_f64.sqrt()).abs() < 1e-12, "{rho}");
        assert_eq!(spearman(&x, &[1.0; 5]), None);
        assert_eq!(spearman(&x[..2], &x[..2]), None);
        assert_eq!(spearman(&x, &[1.0, 2.0, f64::NAN, 4.0, 5.0]), None);
    }

    #[test]
    fn spearman_interval_brackets_the_estimate_and_excludes_zero_when_strong() {
        // y follows x with a little scrambling: strongly positive.
        let x: Vec<f64> = (0..200).map(f64::from).collect();
        let y: Vec<f64> = (0..200).map(|i| f64::from(i) + f64::from((i * 37) % 23)).collect();
        let (rho, (low, high), _) = spearman_interval(&x, &y, 999, 3).unwrap();
        assert!(low <= rho && rho <= high && low > 0.9, "{rho} in {low}..{high}");
        // y unrelated to x by a stride: the interval holds zero.
        let z: Vec<f64> = (0..200).map(|i| f64::from((i * 73) % 200)).collect();
        let (_, (low, high), _) = spearman_interval(&x, &z, 999, 3).unwrap();
        assert!(low < 0.0 && high > 0.0, "{low}..{high}");
    }
}
