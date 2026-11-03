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
        // And `summarise` reads it: the same seed's resampled medians, computed here, give the
        // interval exactly at those quantiles and not at 90%'s. The behaviour alone cannot pin the
        // level more finely - resampled medians of finitely many rows tie, so neighbouring levels
        // can share one interval - which is why the constant carries the exact value.
        let rows: Vec<Vec<f64>> = (0..31).map(|i| vec![f64::from((i * 13) % 31).powi(2)]).collect();
        let (resamples, seed) = (999, 5);
        let mut medians: Vec<f64> = bootstrap_draws(rows.len(), resamples, seed)
            .iter()
            .map(|draw| median(draw.iter().map(|&i| rows[i][0]).collect()))
            .collect();
        medians.sort_by(f64::total_cmp);
        let interval = summarise(&rows, resamples, seed).unwrap()[0].median_interval;
        assert_eq!(interval, (quantile(&medians, 0.025), quantile(&medians, 0.975)));
        assert_ne!(interval, (quantile(&medians, 0.05), quantile(&medians, 0.95)));
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
}
