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
//! - the frozen basis checked against the fit by the law of total variance;
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
use features::eigenshapes::{EigenShapes, Moments, REFERENCE_SHAPES, Weighting, alignment, basis_source, covariance, eigen_shapes};
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

    let _ = writeln!(
        out,
        "\n# the frozen basis checked by the law of total variance: each eigenvalue against the mean within-recording variance of its amplitude (divisor n, from the measure) plus the variance of the recordings' mean amplitudes"
    );
    let _ = writeln!(out, "component\teigenvalue\twithin\tbetween\twithin_plus_between\trelative_difference\twithin_share");
    let mut worst = 0.0_f64;
    for (k, shape) in REFERENCE_SHAPES.iter().enumerate() {
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
    if worst > 1e-9 {
        return Err(format!("the basis check fails: a relative difference of {worst:.3e} exceeds 1e-9"));
    }

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
}
