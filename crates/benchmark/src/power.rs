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

//! How large an alteration of each measure the benchmark's tests can see (STATE.md item 2 (e4)):
//! the positive control of `benchmark::holdout` - one measure of the second sample moved by a
//! stated number of standard deviations of the first - at several sizes, in both directions, at
//! the sample sizes a Grid comparison will have, and the share of replicates in which each test
//! sees it.
//!
//! One **job** is a replicate at one sample size: `n` rows drawn from the second sample without
//! replacement by a seeded shuffle (all of them when `n` is the sample's size), compared with the
//! whole first sample by both tests at the job's own seed - the **null** - and then, for every
//! measure, every size and both directions, with that measure of the drawn rows moved. The
//! shuffles of the per-measure comparison do not depend on the values, so in each alteration only
//! the moved measure's raw p-value is recomputed, from the generator's state recorded in the null
//! (`compare::compare_measures_recording_states`), and Holm's correction is applied again over
//! all the measures; the classifier test is run anew, since the alteration moves every row in its
//! scaled space. The per-measure comparison is also repeated with more shuffles, because Holm's
//! correction at 999 shuffles over 27 measures can flag a measure only when not one shuffle
//! reaches its statistic (journal of 2026-11-04, § Real Against Unseen Real: What Ran, *A floor
//! worth knowing*).
//!
//! Jobs are independent and are shared among a fixed number of threads; each writes into its own
//! slot, so the result does not depend on which thread ran which job.

use crate::compare::{CompareConfig, column_p_from_state, compare_measures_recording_states, holm};
use crate::holdout::{column_sd, shifted};
use crate::{Config, SplitMix64, TestError, two_sample_test};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The study's settings.
#[derive(Clone, Debug, PartialEq)]
pub struct PowerConfig {
    /// The alteration's sizes, in standard deviations of the measure over the first sample; each is
    /// applied upwards and downwards.
    pub sizes_sd: Vec<f64>,
    /// How many rows of the second sample enter, each a sample size of the study.
    pub sample_sizes: Vec<usize>,
    /// Replicates per sample size: draws of the rows, each with its own test seed.
    pub replicates: usize,
    /// The first seed; every job's seeds derive from it by [`job_seeds`].
    pub seed: u64,
    /// Shuffles for the per-measure comparison's second run, beside the default's 999.
    pub more_permutations: usize,
}

/// The seeds of one job: the draw's, and the tests'. Distinct across replicates and sample sizes,
/// so no two jobs share a draw or a shuffle sequence.
pub fn job_seeds(seed: u64, replicate: usize, sample_size: usize) -> (u64, u64) {
    let job = seed.wrapping_mul(1_000_003).wrapping_add(replicate as u64 * 100_003 + sample_size as u64);
    (job.wrapping_mul(2), job.wrapping_mul(2).wrapping_add(1))
}

/// `n` of the rows, drawn without replacement by a seeded shuffle and kept in their original
/// order; all of them, unshuffled, when `n` is their count. More than the rows is an error.
pub fn subsample(rows: &[Vec<f64>], n: usize, seed: u64) -> Result<Vec<Vec<f64>>, String> {
    if n > rows.len() {
        return Err(format!("{n} rows asked of {}", rows.len()));
    }
    let mut indices: Vec<usize> = (0..rows.len()).collect();
    if n < rows.len() {
        SplitMix64(seed).shuffle(&mut indices);
        indices.truncate(n);
        indices.sort_unstable();
    }
    Ok(indices.into_iter().map(|i| rows[i].clone()).collect())
}

/// The null's outcome at one sample size, summed over the replicates.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NullCell {
    pub sample_size: usize,
    pub replicates: usize,
    /// Replicates in which the per-measure comparison flagged any measure.
    pub any_flagged: usize,
    /// The same, with [`PowerConfig::more_permutations`] shuffles.
    pub any_flagged_more: usize,
    /// Replicates in which the classifier told the samples apart.
    pub told_apart: usize,
}

/// One alteration's outcome at one sample size, summed over the replicates.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub sample_size: usize,
    /// The measure moved.
    pub column: usize,
    /// Upwards (+1) or downwards (-1).
    pub direction: i8,
    /// The alteration's size, in standard deviations of the measure over the first sample.
    pub size_sd: f64,
    /// The alteration in the measure's own unit: direction x size x standard deviation.
    pub delta: f64,
    pub replicates: usize,
    /// Replicates in which the per-measure comparison flagged the moved measure.
    pub flagged: usize,
    /// The same, with [`PowerConfig::more_permutations`] shuffles.
    pub flagged_more: usize,
    /// Replicates in which it flagged any other measure, at the default shuffles.
    pub other_flagged: usize,
    /// Replicates in which the classifier told the samples apart.
    pub told_apart: usize,
}

/// The study's outcome: the null at each sample size, then every alteration, in the order sample
/// size, measure, size, direction (up before down).
#[derive(Clone, Debug, PartialEq)]
pub struct Power {
    pub nulls: Vec<NullCell>,
    pub cells: Vec<Cell>,
}

/// One job's outcome, before summing: the null's three verdicts, then one entry per alteration in
/// the cells' order: (moved measure flagged, flagged with more shuffles, any other flagged,
/// told apart).
struct JobOutcome {
    null: (bool, bool, bool),
    alterations: Vec<(bool, bool, bool, bool)>,
}

/// The study, the first sample whole against `n` rows of the second, over `threads` threads.
/// The alteration's sizes are in standard deviations of each measure over the first sample.
pub fn power_study(a: &[Vec<f64>], b: &[Vec<f64>], config: &PowerConfig, threads: usize) -> Result<Power, String> {
    if threads == 0 || config.replicates == 0 || config.sizes_sd.is_empty() || config.sample_sizes.is_empty() {
        return Err("threads, replicates, sizes and sample sizes must each be at least one".into());
    }
    if config.sizes_sd.iter().any(|&s| !s.is_finite() || s <= 0.0) {
        return Err("every size must be positive".into());
    }
    let width = a.first().map(Vec::len).unwrap_or(0);
    let sds: Vec<f64> = (0..width)
        .map(|c| column_sd(a, c))
        .collect::<Option<_>>()
        .ok_or("the first sample has too few rows")?;
    for &n in &config.sample_sizes {
        subsample(b, n, 0)?;
    }
    let jobs: Vec<(usize, usize)> = config.sample_sizes.iter().flat_map(|&n| (0..config.replicates).map(move |r| (n, r))).collect();
    let outcomes: Mutex<Vec<Option<Result<JobOutcome, TestError>>>> = Mutex::new((0..jobs.len()).map(|_| None).collect());
    let next = AtomicUsize::new(0);
    std::thread::scope(|scope| {
        for _ in 0..threads.min(jobs.len()) {
            scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, Ordering::SeqCst);
                    let Some(&(n, replicate)) = jobs.get(index) else { break };
                    let outcome = run_job(a, b, config, &sds, n, replicate);
                    outcomes.lock().expect("a job panicked while holding the outcomes")[index] = Some(outcome);
                }
            });
        }
    });
    let outcomes = outcomes.into_inner().map_err(|_| "a job panicked")?;

    let mut nulls: Vec<NullCell> = Vec::new();
    let mut cells: Vec<Cell> = Vec::new();
    for &n in &config.sample_sizes {
        nulls.push(NullCell {
            sample_size: n,
            replicates: config.replicates,
            ..NullCell::default()
        });
        for (column, &sd) in sds.iter().enumerate() {
            for &size in &config.sizes_sd {
                for direction in [1i8, -1] {
                    cells.push(Cell {
                        sample_size: n,
                        column,
                        direction,
                        size_sd: size,
                        delta: f64::from(direction) * size * sd,
                        replicates: config.replicates,
                        flagged: 0,
                        flagged_more: 0,
                        other_flagged: 0,
                        told_apart: 0,
                    });
                }
            }
        }
    }
    let per_size = width * config.sizes_sd.len() * 2;
    for ((n, _), outcome) in jobs.iter().zip(outcomes) {
        let outcome = outcome.ok_or("a job was not run")?.map_err(|e| e.to_string())?;
        let size_index = config.sample_sizes.iter().position(|m| m == n).expect("the job's sample size is the study's");
        let null = &mut nulls[size_index];
        null.any_flagged += usize::from(outcome.null.0);
        null.any_flagged_more += usize::from(outcome.null.1);
        null.told_apart += usize::from(outcome.null.2);
        for (cell, &(flagged, more, other, apart)) in cells[size_index * per_size..(size_index + 1) * per_size].iter_mut().zip(&outcome.alterations) {
            cell.flagged += usize::from(flagged);
            cell.flagged_more += usize::from(more);
            cell.other_flagged += usize::from(other);
            cell.told_apart += usize::from(apart);
        }
    }
    Ok(Power { nulls, cells })
}

/// Which columns Holm's correction flags at `alpha`, given every column's raw p-value.
fn flagged_by_holm(raw: &[f64], alpha: f64) -> Vec<usize> {
    holm(raw).iter().enumerate().filter(|&(_, &p)| p <= alpha).map(|(c, _)| c).collect()
}

fn run_job(a: &[Vec<f64>], b: &[Vec<f64>], config: &PowerConfig, sds: &[f64], n: usize, replicate: usize) -> Result<JobOutcome, TestError> {
    let (draw_seed, test_seed) = job_seeds(config.seed, replicate, n);
    let drawn = subsample(b, n, draw_seed).expect("the sample sizes were checked");
    let defaults = CompareConfig::default();
    let more = CompareConfig {
        permutations: config.more_permutations,
        ..defaults
    };
    let (null_default, states_default) = compare_measures_recording_states(a, &drawn, &defaults, test_seed)?;
    let (null_more, states_more) = compare_measures_recording_states(a, &drawn, &more, test_seed)?;
    let null_classifier = two_sample_test(a, &drawn, &Config::default(), test_seed)?;
    let raw_default: Vec<f64> = null_default.iter().map(|m| m.p_value).collect();
    let raw_more: Vec<f64> = null_more.iter().map(|m| m.p_value).collect();
    let null = (
        null_default.iter().any(|m| m.differs),
        null_more.iter().any(|m| m.differs),
        null_classifier.told_apart,
    );

    let mut alterations = Vec::with_capacity(sds.len() * config.sizes_sd.len() * 2);
    for (column, &sd) in sds.iter().enumerate() {
        for &size in &config.sizes_sd {
            for direction in [1.0, -1.0] {
                let altered = shifted(&drawn, column, direction * size * sd);
                let values: Vec<f64> = a.iter().chain(&altered).map(|r| r[column]).collect();
                let mut raw = raw_default.clone();
                raw[column] = column_p_from_state(&values, a.len(), defaults.permutations, states_default[column]).1;
                let flagged = flagged_by_holm(&raw, defaults.alpha);
                let mut raw = raw_more.clone();
                raw[column] = column_p_from_state(&values, a.len(), more.permutations, states_more[column]).1;
                let flagged_more = flagged_by_holm(&raw, more.alpha);
                let classifier = two_sample_test(a, &altered, &Config::default(), test_seed)?;
                alterations.push((
                    flagged.contains(&column),
                    flagged_more.contains(&column),
                    flagged.iter().any(|&c| c != column),
                    classifier.told_apart,
                ));
            }
        }
    }
    Ok(JobOutcome { null, alterations })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare::compare_measures;
    use crate::test_support::sample;

    #[test]
    fn a_subsample_is_a_seeded_draw_without_replacement_in_file_order() {
        let rows: Vec<Vec<f64>> = (0..50).map(|i| vec![f64::from(i)]).collect();
        let drawn = subsample(&rows, 10, 7).unwrap();
        assert_eq!(drawn.len(), 10);
        assert!(drawn.windows(2).all(|w| w[0][0] < w[1][0]), "{drawn:?}");
        assert!(drawn.iter().all(|r| rows.contains(r)));
        assert_eq!(drawn, subsample(&rows, 10, 7).unwrap());
        assert_ne!(drawn, subsample(&rows, 10, 8).unwrap());
        assert_ne!(drawn, rows[..10]);
        assert_eq!(subsample(&rows, 50, 7).unwrap(), rows);
        assert!(subsample(&rows, 51, 7).is_err());
    }

    #[test]
    fn job_seeds_are_distinct() {
        let mut seen = Vec::new();
        for replicate in 0..5 {
            for n in [20, 50, 100, 189] {
                let (draw, test) = job_seeds(20_261_108, replicate, n);
                assert_ne!(draw, test);
                seen.push(draw);
                seen.push(test);
            }
        }
        let count = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), count);
    }

    #[test]
    fn one_altered_column_recomputed_from_its_state_equals_the_full_comparison() {
        // The study recomputes only the moved column's raw p-value and applies Holm again; that
        // must give exactly what compare_measures gives on the altered rows, to the last bit.
        let mut rng = SplitMix64(11);
        let a = sample(&mut rng, 40, 5, 0.0, 1.0);
        let b = sample(&mut rng, 30, 5, 0.0, 1.0);
        let config = CompareConfig {
            permutations: 99,
            ..CompareConfig::default()
        };
        let (null, states) = compare_measures_recording_states(&a, &b, &config, 3).unwrap();
        for column in 0..5 {
            let altered = shifted(&b, column, 0.7);
            let full = compare_measures(&a, &altered, &config, 3).unwrap();
            let values: Vec<f64> = a.iter().chain(&altered).map(|r| r[column]).collect();
            let mut raw: Vec<f64> = null.iter().map(|m| m.p_value).collect();
            let (statistic, p) = column_p_from_state(&values, a.len(), config.permutations, states[column]);
            raw[column] = p;
            assert_eq!(statistic, full[column].statistic, "column {column}");
            assert_eq!(raw, full.iter().map(|m| m.p_value).collect::<Vec<_>>(), "column {column}");
            assert_eq!(holm(&raw), full.iter().map(|m| m.adjusted_p).collect::<Vec<_>>(), "column {column}");
        }
    }

    fn small() -> PowerConfig {
        PowerConfig {
            sizes_sd: vec![0.1, 3.0],
            sample_sizes: vec![25, 40],
            replicates: 3,
            seed: 5,
            more_permutations: 199,
        }
    }

    #[test]
    fn a_large_alteration_is_always_seen_and_the_cells_are_in_order() {
        let mut rng = SplitMix64(21);
        let a = sample(&mut rng, 60, 2, 0.0, 1.0);
        let b = sample(&mut rng, 40, 2, 0.0, 1.0);
        let power = power_study(&a, &b, &small(), 1).unwrap();
        assert_eq!(power.nulls.len(), 2);
        assert_eq!(power.cells.len(), 2 * 2 * 2 * 2);
        let order: Vec<(usize, usize, f64, i8)> = power.cells.iter().map(|c| (c.sample_size, c.column, c.size_sd, c.direction)).collect();
        assert_eq!(order[..4], [(25, 0, 0.1, 1), (25, 0, 0.1, -1), (25, 0, 3.0, 1), (25, 0, 3.0, -1)]);
        assert_eq!(order[8].0, 40);
        for cell in &power.cells {
            assert_eq!(cell.replicates, 3);
            let sd = column_sd(&a, cell.column).unwrap();
            assert!((cell.delta - f64::from(cell.direction) * cell.size_sd * sd).abs() < 1e-12);
            if cell.size_sd == 3.0 {
                assert_eq!((cell.flagged, cell.flagged_more, cell.told_apart), (3, 3, 3), "{cell:?}");
            }
            assert_eq!(cell.other_flagged, 0, "{cell:?}");
        }
        // A tenth of a standard deviation at 25 rows is seen rarely, if at all.
        let tiny: usize = power.cells.iter().filter(|c| c.size_sd == 0.1).map(|c| c.flagged + c.told_apart).sum();
        assert!(tiny <= 2, "{tiny}");
    }

    #[test]
    fn two_threads_give_the_same_study_as_one() {
        // At these sizes the verdicts vary between replicates, so a job summed into the wrong cell
        // would show; a study whose every cell is all or nothing could not tell.
        let mut rng = SplitMix64(22);
        let a = sample(&mut rng, 60, 2, 0.0, 1.0);
        let b = sample(&mut rng, 40, 2, 0.0, 1.0);
        let config = PowerConfig {
            sizes_sd: vec![0.5, 1.0],
            replicates: 4,
            ..small()
        };
        let one = power_study(&a, &b, &config, 1).unwrap();
        assert!(one.cells.iter().any(|c| 0 < c.told_apart && c.told_apart < c.replicates), "{:?}", one.cells);
        assert_eq!(one, power_study(&a, &b, &config, 2).unwrap());
    }

    #[test]
    fn bad_settings_are_refused() {
        let mut rng = SplitMix64(23);
        let a = sample(&mut rng, 60, 2, 0.0, 1.0);
        let b = sample(&mut rng, 40, 2, 0.0, 1.0);
        assert!(power_study(&a, &b, &small(), 0).is_err());
        let zero = PowerConfig { sizes_sd: vec![0.0], ..small() };
        assert!(power_study(&a, &b, &zero, 1).is_err());
        let too_many = PowerConfig {
            sample_sizes: vec![41],
            ..small()
        };
        assert!(power_study(&a, &b, &too_many, 1).is_err());
        let none = PowerConfig { replicates: 0, ..small() };
        assert!(power_study(&a, &b, &none, 1).is_err());
    }
}
