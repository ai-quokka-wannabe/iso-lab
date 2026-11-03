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

//! `reference-stats <block> [--basis]`: the reference statistics of a block of the reference draw
//! (STATE.md item 2 (d2)), run from the repository's root after `fetch-block`. The plan they
//! follow was written before they were computed (docs/journal of 2026-11-03, *The Reference
//! Statistics: the Plan, Written Before the Statistics*).
//!
//! Each record of the block is measured by `features::profile`, as `profiles` measures it, and
//! only the measured ones enter. The output, on standard output, is plain text in sections, each
//! a tab-separated table under a line beginning `#`:
//!
//! - the eigen-shapes of the eight mean-removed arc angles over every eight-arc frame of the
//!   measured windows, each recording weighted equally, with bootstrap intervals for their
//!   variance shares; the same fit with frames pooled; and each half's fit;
//! - each measure's median, quartiles and bootstrap interval for the median, over all measured
//!   recordings and over each half;
//! - the frozen basis checked against the fit by the law of total variance and loading by loading,
//!   required only of the reference block;
//! - the halves compared, measure by measure (`benchmark::compare`) and by the classifier test.
//!
//! With `--basis`, it prints instead the first four eigen-shapes, fitted as above, as the Rust
//! source of `features::eigenshapes::REFERENCE_SHAPES`, the basis frozen from block 1.
//!
//! The halves are the block's records in the first and second halves of its draw order. The seed
//! and the number of resamples are fixed below, as the plan states them.

use benchmark::compare::{CompareConfig, compare_measures, quantile};
use benchmark::reference::{INTERVAL_TAILS, Summary, bootstrap_draws, summarise};
use benchmark::{Config, two_sample_test};
use features::eigenshapes::{EigenShapes, Moments, REFERENCE_BLOCK, REFERENCE_SHAPES, Weighting, alignment, basis_source, covariance, eigen_shapes};
use features::posture::{ARCS, eight_arcs};
use features::profile::{mean_removed_angles, measure_names, profile, window_frames};
use features::read_skeletons;
use std::fmt::Write as _;
use std::path::Path;
use std::process::ExitCode;

const ORDER: &str = "data/manifests/owmd-n2-order.tsv";
const DIRECTORY: &str = "data/raw/owmd-n2";
/// The seed of every resample and shuffle, as the plan fixed it.
const SEED: u64 = 20_261_103;
/// The bootstrap's resamples, as the plan fixed them.
const RESAMPLES: usize = 9999;
/// The components whose alignment between the halves is reported.
const ALIGNED: usize = 4;

/// One measured recording.
struct Measured {
    record: String,
    first_half: bool,
    /// The measures, in the order of `features::profile::measure_names`.
    values: Vec<f64>,
    /// The context summarised beside them: the body length and the share of frames without a skeleton.
    context: Vec<f64>,
    moments: Moments,
}

fn summary_table(out: &mut String, names: &[String], set: &str, summaries: &[Summary]) {
    for (name, summary) in names.iter().zip(summaries) {
        let _ = writeln!(
            out,
            "{name}\t{set}\t{}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{:.6}",
            summary.rows,
            summary.median,
            summary.quartiles.0,
            summary.quartiles.1,
            summary.iqr(),
            summary.median_interval.0,
            summary.median_interval.1
        );
    }
}

fn shapes_table(out: &mut String, shapes: &EigenShapes, intervals: Option<&[(f64, f64)]>) {
    let mut cumulative = 0.0;
    let _ = writeln!(
        out,
        "component\teigenvalue[rad^2]\tshare\tshare_low\tshare_high\tcumulative\t{}",
        (1..=ARCS).map(|arc| format!("arc{arc}")).collect::<Vec<_>>().join("\t")
    );
    for (k, share) in shapes.shares().iter().enumerate() {
        cumulative += share;
        let (low, high) = intervals.map_or(("-".to_string(), "-".to_string()), |i| (format!("{:.6}", i[k].0), format!("{:.6}", i[k].1)));
        let loadings: Vec<String> = shapes.shapes[k].iter().map(|x| format!("{x:.4}")).collect();
        let _ = writeln!(
            out,
            "{}\t{:.6e}\t{share:.6}\t{low}\t{high}\t{cumulative:.6}\t{}",
            k + 1,
            shapes.values[k],
            loadings.join("\t")
        );
    }
    let _ = writeln!(out, "components carrying 95%: {}", shapes.carrying(0.95));
}

fn fit(recordings: &[&Measured], weighting: Weighting) -> Option<EigenShapes> {
    let moments: Vec<&Moments> = recordings.iter().map(|r| &r.moments).collect();
    covariance(&moments, weighting).map(|matrix| eigen_shapes(&matrix))
}

/// The 2.5% and 97.5% quantiles of each component's share over the bootstrap's resamples of the
/// recordings, and how many resamples needed each number of components to carry 95%.
fn share_intervals(recordings: &[&Measured]) -> (Vec<(f64, f64)>, [usize; ARCS + 1]) {
    let mut shares: Vec<Vec<f64>> = (0..ARCS).map(|_| Vec::with_capacity(RESAMPLES)).collect();
    let mut carrying = [0usize; ARCS + 1];
    for draw in bootstrap_draws(recordings.len(), RESAMPLES, SEED) {
        let resample: Vec<&Measured> = draw.iter().map(|&i| recordings[i]).collect();
        let Some(shapes) = fit(&resample, Weighting::Recordings) else { continue };
        for (k, share) in shapes.shares().iter().enumerate() {
            shares[k].push(*share);
        }
        carrying[shapes.carrying(0.95)] += 1;
    }
    let intervals = shares
        .into_iter()
        .map(|mut values| {
            values.sort_by(f64::total_cmp);
            (quantile(&values, INTERVAL_TAILS.0), quantile(&values, INTERVAL_TAILS.1))
        })
        .collect();
    (intervals, carrying)
}

/// Reads and measures the block's records, in draw order; returns the measured ones and a count of
/// each reason the others were not.
fn measure_block(records: &[&str]) -> (Vec<Measured>, Vec<(String, usize)>) {
    let mut measured = Vec::new();
    let mut reasons: Vec<(String, usize)> = Vec::new();
    let mut note = |reason: &str| match reasons.iter_mut().find(|(r, _)| r == reason) {
        Some((_, count)) => *count += 1,
        None => reasons.push((reason.to_string(), 1)),
    };
    let half = records.len() / 2;
    for (index, record) in records.iter().enumerate() {
        let path = Path::new(DIRECTORY).join(format!("{record}_features.hdf5"));
        if !path.is_file() {
            note("not_fetched");
            continue;
        }
        let skeletons = match read_skeletons(&path) {
            Ok(skeletons) => skeletons,
            Err(error) => {
                eprintln!("{record}: {error}");
                note("unreadable");
                continue;
            }
        };
        let row = profile(&skeletons);
        let measures = match row.measures {
            Ok(measures) => measures,
            Err(exclusion) => {
                note(exclusion.reason());
                continue;
            }
        };
        let mut moments = Moments::default();
        for frame in 0..window_frames(skeletons.frames(), skeletons.fps) {
            if let Some(arcs) = skeletons.frame(frame).and_then(eight_arcs) {
                moments.add(&mean_removed_angles(&arcs));
            }
        }
        measured.push(Measured {
            record: (*record).to_string(),
            first_half: index < half,
            values: measures.values(),
            context: vec![measures.length, row.context.missing_share],
            moments,
        });
    }
    (measured, reasons)
}

/// The largest absolute difference between a loading of [`REFERENCE_SHAPES`] and the same loading
/// of `fitted`. Unlike the eigenvalues, which a small rotation within the basis changes only to
/// second order, this sees any change of the shapes to first order.
fn basis_mismatch(fitted: &EigenShapes) -> f64 {
    REFERENCE_SHAPES
        .iter()
        .zip(&fitted.shapes)
        .flat_map(|(frozen, fit)| frozen.iter().zip(fit).map(|(a, b)| (a - b).abs()))
        .fold(0.0, f64::max)
}

/// For frozen shape `k`, the mean within-recording variance of its amplitude over `all` (divisor
/// n, recovered from the measure's n - 1 spread) and the variance of the recordings' mean
/// amplitudes: by the law of total variance their sum is the shape's eigenvalue in a fit weighting
/// recordings equally.
fn within_and_between(k: usize, all: &[&Measured]) -> (f64, f64) {
    let shape = &REFERENCE_SHAPES[k];
    let n = all.len() as f64;
    let within = all
        .iter()
        .map(|m| {
            let frames = m.moments.frames as f64;
            let sd = m.values[ARCS + k];
            sd * sd * (frames - 1.0) / frames
        })
        .sum::<f64>()
        / n;
    let means: Vec<f64> = all
        .iter()
        .map(|m| shape.iter().zip(&m.moments.sum).map(|(e, s)| e * s).sum::<f64>() / m.moments.frames as f64)
        .collect();
    let grand = means.iter().sum::<f64>() / n;
    let between = means.iter().map(|x| (x - grand) * (x - grand)).sum::<f64>() / n;
    (within, between)
}

/// The frozen basis checked against `fitted`, the block's own fit, by the law of total variance:
/// each eigenvalue against the mean within-recording variance of its amplitude (from the measure)
/// plus the variance of the recordings' mean amplitudes, and on [`REFERENCE_BLOCK`] each frozen
/// loading against the fit's. Only the reference block must pass: for another block the table is a
/// diagnostic, as its own fit need not match a basis fitted elsewhere. Returns the table, or an
/// error when the reference block fails.
fn check_basis(block: u32, fitted: &EigenShapes, all: &[&Measured]) -> Result<String, String> {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "\n# the frozen basis checked by the law of total variance: each eigenvalue against the mean within-recording variance of its amplitude (divisor n, from the measure) plus the variance of the recordings' mean amplitudes"
    );
    let _ = writeln!(out, "component\teigenvalue\twithin\tbetween\twithin_plus_between\trelative_difference\twithin_share");
    let mut worst = 0.0_f64;
    for k in 0..REFERENCE_SHAPES.len() {
        let (within, between) = within_and_between(k, all);
        let eigenvalue = fitted.values[k];
        let relative = ((within + between) - eigenvalue).abs() / eigenvalue;
        worst = worst.max(relative);
        let _ = writeln!(
            out,
            "{}\t{eigenvalue:.9}\t{within:.9}\t{between:.9}\t{:.9}\t{relative:.3e}\t{:.4}",
            k + 1,
            within + between,
            within / (within + between)
        );
    }
    if block != REFERENCE_BLOCK {
        let _ = writeln!(
            out,
            "block {block} is not the reference block {REFERENCE_BLOCK}: the table above is a diagnostic, and nothing is required of it"
        );
        return Ok(out);
    }
    if worst > 1e-9 {
        return Err(format!("the basis check fails: a relative difference of {worst:.3e} exceeds 1e-9"));
    }
    let largest = basis_mismatch(fitted);
    let _ = writeln!(out, "largest difference between a frozen loading and the fit's: {largest:.3e}");
    if largest >= 1e-12 {
        return Err(format!(
            "the basis check fails: a frozen loading differs from the fit by {largest:.3e}, not below 1e-12"
        ));
    }
    Ok(out)
}

fn report(block: u32, records: &[&str]) -> Result<String, String> {
    let (measured, reasons) = measure_block(records);
    if let Some(gap) = measured.iter().find(|m| m.values.iter().any(|v| !v.is_finite())) {
        return Err(format!(
            "record {} has a measure without a value; the plan stops here rather than drop it",
            gap.record
        ));
    }
    let all: Vec<&Measured> = measured.iter().collect();
    let first: Vec<&Measured> = measured.iter().filter(|m| m.first_half).collect();
    let second: Vec<&Measured> = measured.iter().filter(|m| !m.first_half).collect();
    let mut out = String::new();
    let _ = writeln!(out, "# reference-stats, block {block}: seed {SEED}, {RESAMPLES} bootstrap resamples");
    let _ = writeln!(
        out,
        "# records {}, measured {} (first half {}, second half {}); not measured: {}",
        records.len(),
        all.len(),
        first.len(),
        second.len(),
        reasons.iter().map(|(reason, count)| format!("{reason} {count}")).collect::<Vec<_>>().join(", ")
    );

    let frames: usize = all.iter().map(|m| m.moments.frames).sum();
    let fitted = fit(&all, Weighting::Recordings).ok_or("no frame to fit the eigen-shapes on")?;
    let (intervals, carrying) = share_intervals(&all);
    let _ = writeln!(out, "\n# eigen-shapes, each recording weighted equally ({} recordings, {frames} frames)", all.len());
    shapes_table(&mut out, &fitted, Some(&intervals));
    let _ = writeln!(
        out,
        "resamples needing k components for 95%: {}",
        (1..=ARCS)
            .filter(|&k| carrying[k] > 0)
            .map(|k| format!("k={k}: {}", carrying[k]))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let pooled = fit(&all, Weighting::Frames).ok_or("no frame")?;
    let _ = writeln!(out, "\n# eigen-shapes, frames pooled (the check on the weighting)");
    shapes_table(&mut out, &pooled, None);
    let mut halves = Vec::new();
    for (name, set) in [("first", &first), ("second", &second)] {
        let shapes = fit(set, Weighting::Recordings).ok_or("a half has no frame")?;
        let _ = writeln!(out, "\n# eigen-shapes, {name} half, each recording weighted equally ({} recordings)", set.len());
        shapes_table(&mut out, &shapes, None);
        halves.push(shapes);
    }
    let _ = writeln!(out, "\n# eigen-shapes: absolute cosine between the halves' components");
    for k in 0..ALIGNED {
        let _ = writeln!(
            out,
            "component {}: halves {:.4}, first half against all {:.4}, frames pooled against equal weights {:.4}",
            k + 1,
            alignment(&halves[0].shapes[k], &halves[1].shapes[k]),
            alignment(&halves[0].shapes[k], &fitted.shapes[k]),
            alignment(&pooled.shapes[k], &fitted.shapes[k])
        );
    }

    out.push_str(&check_basis(block, &fitted, &all)?);

    let names = measure_names();
    let context_names = ["context_length_L[um]".to_string(), "context_missing_skeleton_share[1]".to_string()];
    let _ = writeln!(
        out,
        "\n# measures: median, quartiles, interquartile range and 95% percentile bootstrap interval of the median"
    );
    let _ = writeln!(out, "measure\tset\tn\tmedian\tq1\tq3\tiqr\tmedian_low\tmedian_high");
    for (set_name, set) in [("all", &all), ("first", &first), ("second", &second)] {
        let rows: Vec<Vec<f64>> = set.iter().map(|m| m.values.clone()).collect();
        let context: Vec<Vec<f64>> = set.iter().map(|m| m.context.clone()).collect();
        summary_table(&mut out, &names, set_name, &summarise(&rows, RESAMPLES, SEED).ok_or("too few rows")?);
        summary_table(&mut out, &context_names, set_name, &summarise(&context, RESAMPLES, SEED).ok_or("too few rows")?);
    }

    let a: Vec<Vec<f64>> = first.iter().map(|m| m.values.clone()).collect();
    let b: Vec<Vec<f64>> = second.iter().map(|m| m.values.clone()).collect();
    let config = CompareConfig::default();
    let comparisons = compare_measures(&a, &b, &config, SEED).map_err(|e| e.to_string())?;
    let _ = writeln!(
        out,
        "\n# halves compared, measure by measure: Cramér-von Mises, {} shuffles, Holm, alpha {}",
        config.permutations, config.alpha
    );
    let _ = writeln!(out, "measure\tstatistic\tp\tp_holm\tdiffers\tmedian_first\tiqr_first\tmedian_second\tiqr_second");
    for c in &comparisons {
        let _ = writeln!(
            out,
            "{}\t{:.6}\t{:.4}\t{:.4}\t{}\t{:.6}\t{:.6}\t{:.6}\t{:.6}",
            names[c.column], c.statistic, c.p_value, c.adjusted_p, c.differs, c.median_a, c.iqr_a, c.median_b, c.iqr_b
        );
    }
    let _ = writeln!(
        out,
        "measures differing: {} of {}",
        comparisons.iter().filter(|c| c.differs).count(),
        comparisons.len()
    );
    let config = Config::default();
    let outcome = two_sample_test(&a, &b, &config, SEED).map_err(|e| e.to_string())?;
    let _ = writeln!(
        out,
        "\n# halves compared by the classifier test: {} neighbours, {} folds, {} shuffles",
        config.neighbours, config.folds, config.permutations
    );
    let _ = writeln!(
        out,
        "rows {} and {}, accuracy {:.4}, p {:.4}, told apart {}",
        outcome.rows_a, outcome.rows_b, outcome.accuracy, outcome.p_value, outcome.told_apart
    );
    Ok(out)
}

fn main() -> ExitCode {
    let block = match std::env::args().nth(1).map(|argument| argument.parse::<u32>()) {
        Some(Ok(block)) => block,
        _ => {
            eprintln!("usage: reference-stats <block> [--basis]");
            return ExitCode::FAILURE;
        }
    };
    let order = match std::fs::read_to_string(ORDER) {
        Ok(order) => order,
        Err(error) => {
            eprintln!("cannot read {ORDER}: {error}");
            return ExitCode::FAILURE;
        }
    };
    // position, block, record, ...; the first line is the header; the file is in draw order.
    let records: Vec<&str> = order
        .lines()
        .skip(1)
        .filter_map(|line| {
            let fields: Vec<&str> = line.split('\t').collect();
            (fields.get(1)?.parse::<u32>().ok()? == block).then_some(*fields.get(2)?)
        })
        .collect();
    if records.is_empty() {
        eprintln!("block {block} has no records in {ORDER}");
        return ExitCode::FAILURE;
    }
    if std::env::args().nth(2).as_deref() == Some("--basis") {
        let (measured, _) = measure_block(&records);
        let all: Vec<&Measured> = measured.iter().collect();
        return match fit(&all, Weighting::Recordings) {
            Some(shapes) => {
                println!("{}", basis_source(&shapes));
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("no frame to fit the eigen-shapes on");
                ExitCode::FAILURE
            }
        };
    }
    match report(block, &records) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_summary_line_has_a_cell_per_header_column() {
        let summary = Summary {
            rows: 3,
            median: 1.0,
            quartiles: (0.5, 1.5),
            median_interval: (0.8, 1.2),
        };
        let mut out = String::new();
        summary_table(&mut out, &["m".to_string()], "all", &[summary]);
        assert_eq!(
            out.trim_end().split('\t').count(),
            "measure\tset\tn\tmedian\tq1\tq3\tiqr\tmedian_low\tmedian_high".split('\t').count()
        );
    }

    /// One measured recording whose frames all lie at `angles`, `frames` of them, with the given
    /// amplitude spreads in its measures.
    fn recording(angles: [f64; ARCS], frames: usize, spreads: [f64; 4]) -> Measured {
        let mut moments = Moments::default();
        for _ in 0..frames {
            moments.add(&angles);
        }
        let mut values = vec![0.0; ARCS];
        values.extend(spreads);
        Measured {
            record: "1".to_string(),
            first_half: true,
            values,
            context: vec![],
            moments,
        }
    }

    #[test]
    fn the_basis_check_stops_only_for_the_reference_block() {
        // Two recordings that do not fit the frozen basis's eigenvalues at all.
        let a = recording([0.0; ARCS], 10, [0.1; 4]);
        let b = recording([0.0; ARCS], 10, [0.2; 4]);
        let shapes = EigenShapes {
            values: [1.0; ARCS],
            shapes: [[0.0; ARCS]; ARCS],
        };
        assert!(check_basis(REFERENCE_BLOCK, &shapes, &[&a, &b]).is_err());
        let diagnosis = check_basis(REFERENCE_BLOCK + 1, &shapes, &[&a, &b]).expect("another block is diagnosed, not stopped");
        assert!(diagnosis.contains("diagnostic"), "{diagnosis}");
    }

    #[test]
    fn a_small_rotation_of_the_basis_is_seen_component_by_component() {
        let mut shapes = EigenShapes {
            values: [1.0; ARCS],
            shapes: [[0.0; ARCS]; ARCS],
        };
        shapes.shapes[..4].copy_from_slice(&REFERENCE_SHAPES);
        assert_eq!(basis_mismatch(&shapes), 0.0);
        // Shapes 1 and 2 turned by 3e-5 rad within their plane: the eigenvalues of a fit would move
        // only by about 1e-9 of a gap's size, but the loadings move by up to 3e-5.
        let (sin, cos) = 3e-5_f64.sin_cos();
        let (one, two) = (REFERENCE_SHAPES[0], REFERENCE_SHAPES[1]);
        shapes.shapes[0] = std::array::from_fn(|i| cos * one[i] + sin * two[i]);
        shapes.shapes[1] = std::array::from_fn(|i| cos * two[i] - sin * one[i]);
        let mismatch = basis_mismatch(&shapes);
        assert!(mismatch > 1e-6 && mismatch < 3e-5, "{mismatch}");
    }

    /// The stop at 1e-12 is the check's only first-order guard, so it is tested on its own: a fit
    /// whose eigenvalues satisfy the law of total variance exactly, with shapes 1 and 2 turned by
    /// 1e-6 rad, passes the eigenvalue check and must still be stopped; with the frozen shapes
    /// themselves, it passes.
    #[test]
    fn the_basis_check_stops_a_fit_whose_loadings_differ_beyond_1e_12() {
        let mut angles_a = [0.0; ARCS];
        let mut angles_b = [0.0; ARCS];
        for i in 0..ARCS {
            angles_a[i] = 0.3 * REFERENCE_SHAPES[0][i] + 0.1 * REFERENCE_SHAPES[2][i];
            angles_b[i] = -0.2 * REFERENCE_SHAPES[1][i] + 0.05 * REFERENCE_SHAPES[3][i];
        }
        let a = recording(angles_a, 10, [0.4, 0.3, 0.2, 0.1]);
        let b = recording(angles_b, 12, [0.5, 0.35, 0.25, 0.15]);
        let all = [&a, &b];
        let mut fitted = EigenShapes {
            values: [1.0; ARCS],
            shapes: [[0.0; ARCS]; ARCS],
        };
        for k in 0..REFERENCE_SHAPES.len() {
            let (within, between) = within_and_between(k, &all);
            fitted.values[k] = within + between;
        }
        fitted.shapes[..4].copy_from_slice(&REFERENCE_SHAPES);
        assert!(check_basis(REFERENCE_BLOCK, &fitted, &all).is_ok());
        let (sin, cos) = 1e-6_f64.sin_cos();
        let (one, two) = (REFERENCE_SHAPES[0], REFERENCE_SHAPES[1]);
        fitted.shapes[0] = std::array::from_fn(|i| cos * one[i] + sin * two[i]);
        fitted.shapes[1] = std::array::from_fn(|i| cos * two[i] - sin * one[i]);
        let error = check_basis(REFERENCE_BLOCK, &fitted, &all).expect_err("a turned basis must be stopped");
        assert!(error.contains("frozen loading"), "{error}");
    }
}
