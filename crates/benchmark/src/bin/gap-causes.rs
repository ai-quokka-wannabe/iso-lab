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

//! `gap-causes <block>`: a block's missing frames by cause, and how much each measure leans on
//! them (STATE.md item 2 (d3)), run from the repository's root after `fetch-block`. The plan it
//! follows was written before it ran (docs/journal of 2026-11-03, *The Gaps by Cause: the Plan,
//! Written Before the Numbers*). The output, on standard output, is plain text in sections, each
//! a tab-separated table under a line beginning `#`:
//!
//! - every record's gaps by cause (`features::gaps`), over its window as `features::profile`
//!   counts it, from the file's first row, and the share of the window before the file when the
//!   window is counted from the video's first frame instead;
//! - for the measured recordings, Spearman's correlation of each measure with the share of each
//!   cause, with a bootstrap interval;
//! - the bend energy and the absolute midbody speed beside each cause's runs against far from any
//!   gap, as a ratio per recording, with its median's bootstrap interval;
//! - the late-starting files, and what counting the window from the video's first frame would
//!   change.

use benchmark::compare::quantile;
use benchmark::reference::{INTERVAL_TAILS, spearman_interval, summarise};
use features::gaps::{Row, classify, edges, mean_over, runs};
use features::locomotion::published_choice_speeds;
use features::posture::{EightArcs, eight_arcs};
use features::profile::{WINDOW_SECONDS, mean_removed_angles, measure_names, profile, window_frames};
use features::{FrameIndex, Skeletons, read_frame_index, read_skeletons};
use std::fmt::Write as _;
use std::path::Path;
use std::process::ExitCode;

const ORDER: &str = "data/manifests/owmd-n2-order.tsv";
const DIRECTORY: &str = "data/raw/owmd-n2";
/// The seed of every resample, as the plan fixed it.
const SEED: u64 = 20_261_103;
/// The bootstrap's resamples, as the plan fixed them.
const RESAMPLES: usize = 9999;
/// Rows within this many seconds of a run's end are beside it.
const BORDER_SECONDS: f64 = 0.5;
/// Rows at least this many seconds from every missing row are far from the gaps.
const FAR_SECONDS: f64 = 2.0;
/// The causes whose runs are looked at: the kind, and its name in the output.
const CAUSES: [(Row, &str); 2] = [(Row::NoGoodSkeleton, "no_good_skeleton"), (Row::Stage, "stage")];

/// One record as read and classified.
struct Record {
    record: String,
    status: String,
    fps: f64,
    first_frame: u64,
    rows: Vec<Row>,
    /// Measures in the order of `measure_names`, for a measured record.
    values: Option<Vec<f64>>,
    /// Border-over-far ratios, bend energy then speed, for each cause; `None` without border or far rows.
    ratios: [[Option<f64>; 2]; 2],
    /// The status, and measures, with the window counted from the video's first frame.
    from_video: (String, Option<Vec<f64>>, f64),
}

fn status(skeletons: &Skeletons) -> (String, Option<Vec<f64>>) {
    match profile(skeletons).measures {
        Ok(measures) => ("measured".to_string(), Some(measures.values())),
        Err(exclusion) => (exclusion.reason().to_string(), None),
    }
}

fn share(rows: &[Row], kind: Row) -> f64 {
    rows.iter().filter(|&&row| row == kind).count() as f64 / rows.len().max(1) as f64
}

fn median_run_seconds(rows: &[Row], kind: Row, fps: f64) -> f64 {
    let mut lengths: Vec<f64> = runs(rows, kind).iter().map(|&(_, length)| length as f64 / fps).collect();
    if lengths.is_empty() {
        return f64::NAN;
    }
    lengths.sort_by(f64::total_cmp);
    quantile(&lengths, 0.5)
}

fn read(record: &str) -> Result<Record, String> {
    let path = Path::new(DIRECTORY).join(format!("{record}_features.hdf5"));
    let skeletons = read_skeletons(&path).map_err(|e| e.to_string())?;
    let index: FrameIndex = read_frame_index(&path).map_err(|e| e.to_string())?;
    let fps = skeletons.fps;
    let window = window_frames(skeletons.frames(), fps);
    let rows = classify(&skeletons, &index, window).map_err(|e| format!("{e:?}"))?;
    let (status_d1, values) = status(&skeletons);

    let arcs: Vec<Option<EightArcs>> = (0..window).map(|row| skeletons.frame(row).and_then(eight_arcs)).collect();
    let energy: Vec<Option<f64>> = arcs
        .iter()
        .map(|arcs| arcs.as_ref().map(|arcs| mean_removed_angles(arcs).iter().map(|a| a * a).sum()))
        .collect();
    let speed: Vec<Option<f64>> = published_choice_speeds(&arcs, fps).iter().map(|speed| speed.map(f64::abs)).collect();
    let (border, far) = (rows_in(BORDER_SECONDS, fps), rows_in(FAR_SECONDS, fps));
    let ratios = CAUSES.map(|(kind, _)| {
        let e = edges(&rows, kind, border, far, index.first_frame > 0);
        [&energy, &speed].map(|values| match (mean_over(values, &e.border), mean_over(values, &e.far)) {
            (Some(near), Some(away)) if away > 0.0 => Some(near / away),
            _ => None,
        })
    });

    // The window from the video's first frame: the rows of the file inside the protocol's 900 s.
    let video_window = (WINDOW_SECONDS * fps).round() as usize;
    let video_rows = video_window.saturating_sub(index.first_frame as usize).min(skeletons.frames());
    let (status_video, values_video) = status(&skeletons.first(video_rows));
    Ok(Record {
        record: record.to_string(),
        status: status_d1,
        fps,
        first_frame: index.first_frame,
        rows,
        values,
        ratios,
        from_video: (
            status_video,
            values_video,
            (index.first_frame as usize).min(video_window) as f64 / video_window as f64,
        ),
    })
}

fn rows_in(seconds: f64, fps: f64) -> usize {
    (seconds * fps).round() as usize
}

fn report(block: u32, records: &[&str]) -> Result<String, String> {
    let mut out = String::new();
    let mut read_records = Vec::new();
    for record in records {
        match read(record) {
            Ok(r) => read_records.push(r),
            Err(error) => {
                eprintln!("{record}: not read ({error})");
                let _ = writeln!(out, "# {record}: not read");
            }
        }
    }
    let _ = writeln!(
        out,
        "# gap-causes, block {block}: seed {SEED}, {RESAMPLES} bootstrap resamples, intervals at {INTERVAL_TAILS:?}"
    );
    let _ = writeln!(out, "# border {BORDER_SECONDS} s, far {FAR_SECONDS} s");
    let _ = writeln!(
        out,
        "\n# gaps by cause over the window from the file's first row (shares of the window's rows); runs and their median length in seconds"
    );
    let _ = writeln!(
        out,
        "record\tstatus\tfps\tfirst_frame\toffset[s]\twindow[s]\teight_arcs\tskeleton_only\tstage\tno_good_skeleton\tstage_runs\tstage_run_median[s]\tno_good_runs\tno_good_run_median[s]\tbefore_file_share_from_video"
    );
    for r in &read_records {
        let _ = writeln!(
            out,
            "{}\t{}\t{:.4}\t{}\t{:.3}\t{:.3}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{}\t{:.3}\t{}\t{:.3}\t{:.6}",
            r.record,
            r.status,
            r.fps,
            r.first_frame,
            r.first_frame as f64 / r.fps,
            r.rows.len() as f64 / r.fps,
            share(&r.rows, Row::EightArcs),
            share(&r.rows, Row::SkeletonOnly),
            share(&r.rows, Row::Stage),
            share(&r.rows, Row::NoGoodSkeleton),
            runs(&r.rows, Row::Stage).len(),
            median_run_seconds(&r.rows, Row::Stage, r.fps),
            runs(&r.rows, Row::NoGoodSkeleton).len(),
            median_run_seconds(&r.rows, Row::NoGoodSkeleton, r.fps),
            r.from_video.2
        );
    }

    let measured: Vec<&Record> = read_records.iter().filter(|r| r.values.is_some()).collect();
    let _ = writeln!(
        out,
        "\n# shares by cause over the {} measured recordings: median, quartiles, median's interval",
        measured.len()
    );
    let _ = writeln!(out, "cause\tmedian\tq1\tq3\tmedian_low\tmedian_high");
    let kinds = [
        (Row::EightArcs, "eight_arcs"),
        (Row::SkeletonOnly, "skeleton_only"),
        (Row::Stage, "stage"),
        (Row::NoGoodSkeleton, "no_good_skeleton"),
    ];
    let share_rows: Vec<Vec<f64>> = measured.iter().map(|r| kinds.iter().map(|&(kind, _)| share(&r.rows, kind)).collect()).collect();
    let summaries = summarise(&share_rows, RESAMPLES, SEED).ok_or("too few measured recordings")?;
    for ((_, name), s) in kinds.iter().zip(&summaries) {
        let _ = writeln!(
            out,
            "{name}\t{:.6}\t{:.6}\t{:.6}\t{:.6}\t{:.6}",
            s.median, s.quartiles.0, s.quartiles.1, s.median_interval.0, s.median_interval.1
        );
    }

    let names = measure_names();
    let _ = writeln!(
        out,
        "\n# Spearman's correlation of each measure with each cause's share, over the measured recordings"
    );
    let _ = writeln!(out, "measure\tcause\trho\trho_low\trho_high\tresamples_without_rho\texcludes_zero");
    let mut excluding = 0;
    for (column, name) in names.iter().enumerate() {
        let values: Vec<f64> = measured.iter().map(|r| r.values.as_ref().map_or(f64::NAN, |v| v[column])).collect();
        for (kind, cause) in CAUSES {
            let shares: Vec<f64> = measured.iter().map(|r| share(&r.rows, kind)).collect();
            match spearman_interval(&values, &shares, RESAMPLES, SEED) {
                Some((rho, (low, high), undefined)) => {
                    let excludes = low > 0.0 || high < 0.0;
                    excluding += usize::from(excludes);
                    let _ = writeln!(out, "{name}\t{cause}\t{rho:.4}\t{low:.4}\t{high:.4}\t{undefined}\t{excludes}");
                }
                None => {
                    let _ = writeln!(out, "{name}\t{cause}\tNaN\tNaN\tNaN\t-\tfalse");
                }
            }
        }
    }
    let _ = writeln!(out, "intervals excluding zero: {excluding} of {}", 2 * names.len());

    let _ = writeln!(out, "\n# border over far, per measured recording: the median ratio and its interval");
    let _ = writeln!(out, "cause\tquantity\trecordings\tmedian\tq1\tq3\tmedian_low\tmedian_high");
    for (c, (_, cause)) in CAUSES.iter().enumerate() {
        for (q, quantity) in ["bend_energy", "abs_midbody_speed"].iter().enumerate() {
            let ratios: Vec<Vec<f64>> = measured.iter().filter_map(|r| r.ratios[c][q]).map(|ratio| vec![ratio]).collect();
            match summarise(&ratios, RESAMPLES, SEED) {
                Some(s) => {
                    let s = &s[0];
                    let _ = writeln!(
                        out,
                        "{cause}\t{quantity}\t{}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\t{:.4}",
                        s.rows, s.median, s.quartiles.0, s.quartiles.1, s.median_interval.0, s.median_interval.1
                    );
                }
                None => {
                    let _ = writeln!(out, "{cause}\t{quantity}\t{}\tNaN", ratios.len());
                }
            }
        }
    }

    // The block's IQR of each measure, to scale a change by.
    let iqr: Vec<f64> = (0..names.len())
        .map(|column| {
            let mut values: Vec<f64> = measured.iter().filter_map(|r| r.values.as_ref().map(|v| v[column])).collect();
            values.sort_by(f64::total_cmp);
            quantile(&values, 0.75) - quantile(&values, 0.25)
        })
        .collect();
    let _ = writeln!(
        out,
        "\n# files starting after the video's first frame, and the window counted from the video's first frame"
    );
    let _ = writeln!(
        out,
        "record\tfirst_frame\toffset[s]\tstatus_from_file\tstatus_from_video\tlargest_change_over_iqr\tmeasure"
    );
    for r in read_records.iter().filter(|r| r.first_frame > 0) {
        let (change, which) = match (&r.values, &r.from_video.1) {
            (Some(a), Some(b)) => {
                let (column, change) = a
                    .iter()
                    .zip(b)
                    .zip(&iqr)
                    .map(|((x, y), scale)| (y - x).abs() / scale)
                    .enumerate()
                    .fold((0, 0.0_f64), |best, (column, change)| if change > best.1 { (column, change) } else { best });
                (format!("{change:.4}"), names[column].clone())
            }
            _ => ("-".to_string(), "-".to_string()),
        };
        let _ = writeln!(
            out,
            "{}\t{}\t{:.3}\t{}\t{}\t{change}\t{which}",
            r.record,
            r.first_frame,
            r.first_frame as f64 / r.fps,
            r.status,
            r.from_video.0
        );
    }
    let changed = read_records.iter().filter(|r| r.status != r.from_video.0).count();
    let _ = writeln!(out, "records whose status would change: {changed}");
    Ok(out)
}

fn main() -> ExitCode {
    let block = match std::env::args().nth(1).map(|argument| argument.parse::<u32>()) {
        Some(Ok(block)) => block,
        _ => {
            eprintln!("usage: gap-causes <block>");
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
    fn runs_are_timed_in_seconds_and_shares_are_of_the_rows() {
        use Row::{EightArcs as A, NoGoodSkeleton as N};
        let rows = [N, N, A, N, N, N, N, A];
        // Runs of 2 and 4 rows at 2 frames per second: 1 s and 2 s, median 1.5 s.
        assert_eq!(median_run_seconds(&rows, N, 2.0), 1.5);
        assert!(median_run_seconds(&rows, Row::Stage, 2.0).is_nan());
        assert_eq!(share(&rows, N), 0.75);
        assert_eq!(rows_in(0.5, 25.0), 13);
    }
}
