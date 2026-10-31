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

//! `profiles <block>`: one tab-separated row of measures per recording of a block of the reference
//! draw (`data/manifests/owmd-n2-order.tsv`), in the draw's order, by `features::profile`; run
//! from the repository's root, after `fetch-block`. The header names each column with its unit in
//! brackets, `[1]` for a share or a count, `[L/s]` for body lengths per second.
//!
//! Every record of the block gets a row. `status` is `measured` or `excluded`, and `reason` says
//! why: `short` or `gappy` by the rules of `features::profile`, `unreadable` when the file cannot
//! be read, `not_fetched` when it is not in `data/raw/owmd-n2/`. An excluded row carries `NaN` in
//! every measure, and its context columns where they are known. The columns whose names begin
//! `provisional_` are the locomotion states' durations and rates, provisional until STATE.md item
//! 2(c)'s check of the published durations passes.

use features::posture::ARCS;
use features::profile::{Profile, profile};
use features::read_skeletons;
use std::fmt::Write as _;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const ORDER: &str = "data/manifests/owmd-n2-order.tsv";
const DIRECTORY: &str = "data/raw/owmd-n2";
const MODES: [&str; 3] = ["forward", "backward", "paused"];

fn header() -> String {
    let mut columns: Vec<String> = [
        "record",
        "status",
        "reason",
        "fps[1/s]",
        "window_frames[1]",
        "window[s]",
        "missing_skeleton_share[1]",
        "eight_arc_share[1]",
        "length_L[um]",
    ]
    .map(String::from)
    .to_vec();
    columns.extend((1..=ARCS).map(|arc| format!("arc{arc}_angle_sd[rad]")));
    columns.extend(["midbody_speed_mean[L/s]", "midbody_speed_abs_mean[L/s]"].map(String::from));
    columns.extend(MODES.iter().chain(&["no_state"]).map(|mode| format!("{mode}_share[1]")));
    for mode in MODES {
        columns.extend([
            format!("provisional_{mode}_events[1]"),
            format!("provisional_{mode}_mean_duration[s]"),
            format!("provisional_{mode}_frequency[1/s]"),
        ]);
    }
    columns.join("\t")
}

/// How many measure columns follow the context ones.
fn measure_columns() -> usize {
    ARCS + 2 + 4 + 3 * MODES.len()
}

fn number(value: f64) -> String {
    if value.is_nan() { "NaN".to_string() } else { format!("{value:.6}") }
}

fn optional(value: Option<f64>) -> String {
    value.map_or("NaN".to_string(), number)
}

fn row(record: &str, profile: &Profile) -> String {
    let context = &profile.context;
    let (status, reason) = match &profile.measures {
        Ok(_) => ("measured", "-"),
        Err(exclusion) => ("excluded", exclusion.reason()),
    };
    let mut line = format!(
        "{record}\t{status}\t{reason}\t{}\t{}\t{}\t{}\t{}",
        number(context.fps),
        context.frames,
        number(context.seconds),
        number(context.missing_share),
        number(context.eight_arc_share)
    );
    let Ok(measures) = &profile.measures else {
        line.push_str(&"\tNaN".repeat(1 + measure_columns()));
        return line;
    };
    let mut cells = vec![number(measures.length)];
    cells.extend(measures.angle_spread.iter().map(|&spread| number(spread)));
    cells.extend([optional(measures.speed_mean), optional(measures.speed_abs_mean)]);
    cells.extend(measures.state_shares.iter().map(|&share| number(share)));
    for mode in &measures.modes {
        cells.extend([mode.events.to_string(), optional(mode.mean_duration), optional(mode.frequency)]);
    }
    for cell in cells {
        let _ = write!(line, "\t{cell}");
    }
    line
}

/// A row for a record with no profile: no context known beyond the record.
fn unmeasured(record: &str, reason: &str) -> String {
    format!("{record}\texcluded\t{reason}{}", "\tNaN".repeat(6 + measure_columns()))
}

fn main() -> ExitCode {
    let block = match std::env::args().nth(1).map(|argument| argument.parse::<u32>()) {
        Some(Ok(block)) => block,
        _ => {
            eprintln!("usage: profiles <block>");
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
    if writeln!(out, "{}", header()).is_err() {
        return ExitCode::FAILURE;
    }
    for record in records {
        let path = Path::new(DIRECTORY).join(format!("{record}_features.hdf5"));
        let line = if !path.is_file() {
            unmeasured(record, "not_fetched")
        } else {
            match read_skeletons(&path) {
                Ok(skeletons) => row(record, &profile(&skeletons)),
                Err(error) => {
                    eprintln!("{record}: {error}");
                    unmeasured(record, "unreadable")
                }
            }
        };
        if writeln!(out, "{line}").is_err() {
            return ExitCode::FAILURE;
        }
    }
    if out.flush().is_err() { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_row_has_as_many_cells_as_the_header() {
        let width = header().split('\t').count();
        assert_eq!(unmeasured("1", "not_fetched").split('\t').count(), width);
        let context = features::profile::Context {
            fps: 30.0,
            frames: 10,
            seconds: 1.0 / 3.0,
            missing_share: 0.0,
            eight_arc_share: 1.0,
        };
        let excluded = Profile {
            context,
            measures: Err(features::profile::Exclusion::Short),
        };
        assert_eq!(row("1", &excluded).split('\t').count(), width);
        let none = || features::profile::ModeEvents {
            events: 0,
            mean_duration: None,
            frequency: None,
        };
        let measured = Profile {
            measures: Ok(features::profile::Measures {
                length: 1000.0,
                angle_spread: [0.1; ARCS],
                speed_mean: Some(0.1),
                speed_abs_mean: Some(0.1),
                state_shares: [0.25; 4],
                modes: [none(), none(), none()],
            }),
            ..excluded
        };
        assert_eq!(row("1", &measured).split('\t').count(), width);
    }
}
