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

//! `stored-events [--lab-speed] <block>`: for each recording of a block of the reference draw
//! (`data/manifests/owmd-n2-order.tsv`) fetched into `data/raw/owmd-n2/`, recomputes each
//! locomotion state's events (`features::states::motion_events`, `event_stats`) and compares them
//! with those the toolbox stored (`features::stored_events`). The input is the published
//! `midbody_speed` and `length` from `/features_timeseries`, or with `--lab-speed` the lab's
//! eight-arc speed with the published choices (`locomotion::published_choice_speeds`) and eight-arc
//! length, as `features::profile` takes them, over the whole recording. Run from the repository's root.
//!
//! One tab-separated row per recording and mode, in the draw's order: the counts, mean durations
//! and frequencies, recomputed and stored, the largest relative difference of paired durations
//! (`NaN` when the counts differ), that of the frequencies, and `yes` or `no` for reproduced
//! within `stored_events::TOLERANCE`, and `yes` or `no` for whether the recomputed events are a lone
//! event touching an edge (`states::lone_edge_event`). A recording not fetched or unreadable gets one row with its
//! reason. A summary goes to standard error: how many recording-modes are reproduced, the largest
//! relative difference, how many recording-modes have a lone edge event and which, and per mode the median of ours over stored for the count, the mean duration
//! and the frequency.

use features::locomotion::published_choice_speeds;
use features::posture::eight_arcs;
use features::states::{MotionMode, event_stats, lone_edge_event, motion_events};
use features::stored_events::{Agreement, compare, read_stored_events};
use features::{PublishedSpeed, read_published_speed, read_skeletons};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const ORDER: &str = "data/manifests/owmd-n2-order.tsv";
const DIRECTORY: &str = "data/raw/owmd-n2";

const HEADER: &str = "record\tmode\tcount_ours\tcount_stored\tmean_duration_ours[s]\tmean_duration_stored[s]\tfrequency_ours[1/s]\tfrequency_stored[1/s]\tworst_duration_relative[1]\tfrequency_relative[1]\treproduced\tlone_edge_ours";

fn number(value: Option<f64>) -> String {
    value.map_or("NaN".to_string(), |value| format!("{value:e}"))
}

fn name(mode: MotionMode) -> &'static str {
    match mode {
        MotionMode::Forward => "forward",
        MotionMode::Backward => "backward",
        MotionMode::Paused => "paused",
    }
}

fn row(record: &str, mode: MotionMode, agreement: &Agreement, lone_edge: bool) -> String {
    format!(
        "{record}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
        name(mode),
        agreement.counts.0,
        agreement.counts.1,
        number(agreement.mean_durations.0),
        number(agreement.mean_durations.1),
        number(agreement.frequencies.0),
        number(agreement.frequencies.1),
        number(agreement.worst_duration),
        number(Some(agreement.frequency_difference).filter(|difference| difference.is_finite())),
        if agreement.reproduced() { "yes" } else { "no" },
        if lone_edge { "yes" } else { "no" }
    )
}

/// The speeds, lengths and frame rate a recording's events are recomputed from, in the published
/// speed's shape whichever speed it is.
fn input(path: &Path, lab: bool) -> Result<PublishedSpeed, String> {
    if lab {
        let skeletons = read_skeletons(path).map_err(|error| error.to_string())?;
        let arcs: Vec<_> = (0..skeletons.frames()).map(|index| skeletons.frame(index).and_then(eight_arcs)).collect();
        let lengths = arcs.iter().map(|arcs| arcs.as_ref().map(|arcs| arcs.length)).collect();
        Ok(PublishedSpeed {
            fps: skeletons.fps,
            speeds: published_choice_speeds(&arcs, skeletons.fps),
            lengths,
        })
    } else {
        read_published_speed(path).map_err(|error| error.to_string())
    }
}

/// Per mode, the agreement of the recomputed events with the stored ones, and whether the
/// recomputed events are a lone edge event.
fn agreements(path: &Path, lab: bool) -> Result<Vec<(MotionMode, Agreement, bool)>, String> {
    let PublishedSpeed { fps, speeds, lengths } = input(path, lab)?;
    let stored = read_stored_events(path).map_err(|error| error.to_string())?;
    Ok(motion_events(&speeds, &lengths, fps)
        .iter()
        .zip(&stored)
        .map(|((mode, events), (stored_mode, stored))| {
            debug_assert_eq!(mode, stored_mode);
            (*mode, compare(&event_stats(events, speeds.len(), fps), stored), lone_edge_event(events, speeds.len()))
        })
        .collect())
}

fn main() -> ExitCode {
    let mut arguments: Vec<String> = std::env::args().skip(1).collect();
    let lab = match arguments.iter().position(|argument| argument == "--lab-speed") {
        Some(at) => {
            arguments.remove(at);
            true
        }
        None => false,
    };
    let block = match arguments.as_slice() {
        [block] => match block.parse::<u32>() {
            Ok(block) => block,
            Err(_) => {
                eprintln!("usage: stored-events [--lab-speed] <block>");
                return ExitCode::FAILURE;
            }
        },
        _ => {
            eprintln!("usage: stored-events [--lab-speed] <block>");
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
    // position, block, record, ...; the first line is the header.
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
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let mut lines = vec![HEADER.to_string()];
    let (mut compared, mut reproduced, mut skipped) = (0, 0, 0);
    let mut worst: f64 = 0.0;
    let mut lone_edges: Vec<String> = Vec::new();
    // Per mode, ours over stored for the count, the mean duration and the frequency, where both are defined.
    let mut ratios: [[Vec<f64>; 3]; 3] = Default::default();
    for record in &records {
        let path = Path::new(DIRECTORY).join(format!("{record}_features.hdf5"));
        if !path.is_file() {
            lines.push(format!("{record}\t-\tnot_fetched"));
            skipped += 1;
            continue;
        }
        match agreements(&path, lab) {
            Ok(modes) => {
                for (mode, agreement, lone_edge) in &modes {
                    compared += 1;
                    reproduced += usize::from(agreement.reproduced());
                    worst = worst.max(agreement.worst_duration.unwrap_or(f64::INFINITY)).max(agreement.frequency_difference);
                    lines.push(row(record, *mode, agreement, *lone_edge));
                    if *lone_edge {
                        lone_edges.push(format!("{record} {}{}", name(*mode), if agreement.reproduced() { "" } else { " (not reproduced)" }));
                    }
                    let slot = &mut ratios[*mode as usize];
                    let pairs = [
                        (Some(agreement.counts.0 as f64), Some(agreement.counts.1 as f64)),
                        agreement.mean_durations,
                        agreement.frequencies,
                    ];
                    for (values, pair) in slot.iter_mut().zip(pairs) {
                        if let (Some(ours), Some(stored)) = pair
                            && stored > 0.0
                        {
                            values.push(ours / stored);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("{record}: {error}");
                lines.push(format!("{record}\t-\tunreadable"));
                skipped += 1;
            }
        }
    }
    for line in lines {
        if writeln!(out, "{line}").is_err() {
            return ExitCode::FAILURE;
        }
    }
    eprintln!(
        "block {block}, {} speed: {reproduced} of {compared} recording-modes reproduced; {skipped} recordings skipped; largest relative difference {worst:e}",
        if lab { "the lab's" } else { "the published" }
    );
    eprintln!("lone edge events: {} recording-modes: {}", lone_edges.len(), lone_edges.join(", "));
    for mode in [MotionMode::Forward, MotionMode::Backward, MotionMode::Paused] {
        let [count, duration, frequency] = &ratios[mode as usize];
        eprintln!(
            "{}: median ours/stored, count {} (n = {}), mean duration {} (n = {}), frequency {} (n = {})",
            name(mode),
            number(median(count)),
            count.len(),
            number(median(duration)),
            duration.len(),
            number(median(frequency)),
            frequency.len()
        );
    }
    if out.flush().is_err() { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

/// The median, the mean of the middle two for an even count; `None` when empty.
fn median(values: &[f64]) -> Option<f64> {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let half = sorted.len() / 2;
    match sorted.len() {
        0 => None,
        n if n % 2 == 1 => Some(sorted[half]),
        _ => Some((sorted[half - 1] + sorted[half]) / 2.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_has_as_many_cells_as_the_header() {
        let agreement = compare(
            &event_stats(&[(1, 2)], 5, 1.0),
            &features::stored_events::StoredEvents {
                durations: vec![],
                frequency: None,
            },
        );
        assert_eq!(row("1", MotionMode::Paused, &agreement, false).split('\t').count(), HEADER.split('\t').count());
        assert!(row("1", MotionMode::Paused, &agreement, false).ends_with("\tNaN\tNaN\tno\tno"));
        assert!(row("1", MotionMode::Paused, &agreement, true).ends_with("\tno\tyes"));
    }

    #[test]
    fn the_median_of_odd_and_even_counts() {
        assert_eq!(median(&[]), None);
        assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), Some(2.5));
    }
}
