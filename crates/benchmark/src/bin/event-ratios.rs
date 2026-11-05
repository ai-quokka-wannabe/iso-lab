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

//! `event-ratios <stored-events.tsv>`: the lab's speed against the published one, per locomotion
//! state, from the output of `stored-events --lab-speed <block>` (STATE.md item 2 (c)), by
//! `benchmark::ratios` at [`SEED`] with [`RESAMPLES`] resamples, both fixed in the plan (docs/journal
//! of 2026-11-05, *Item 2 (c)'s Follow-Ups: the Plan, Written Before the Numbers*).
//!
//! One tab-separated row per mode and quantity, in the input's mode order: the recordings it is
//! taken over; the ratio's median and 95% interval; the difference's median and interval; the
//! stored values' interquartile range; the shift and its interval in those ranges; and its reading
//! against `ratios::THRESHOLD_IQR`.

use benchmark::ratios::{QUANTITIES, Reading, compare_cell, read_rows};
use std::fmt::Write as _;
use std::process::ExitCode;

/// The seed of every resample, fixed in the plan.
const SEED: u64 = 20_261_105;
/// The number of bootstrap resamples, as `reference-stats` takes.
const RESAMPLES: usize = 9999;

const HEADER: &str = "mode\tquantity\trecordings\tratio_median\tratio_low\tratio_high\tdifference_median\tdifference_low\tdifference_high\tstored_iqr\tshift_iqr\tshift_low\tshift_high\treading";

fn run(text: &str) -> Result<String, String> {
    let (rows, skipped) = read_rows(text)?;
    let mut modes: Vec<&str> = Vec::new();
    for row in &rows {
        if !modes.contains(&row.mode.as_str()) {
            modes.push(&row.mode);
        }
    }
    let mut out = String::new();
    let _ = writeln!(
        out,
        "# event-ratios: seed {SEED}, {RESAMPLES} bootstrap resamples; {} rows, {skipped} recordings skipped",
        rows.len()
    );
    let _ = writeln!(out, "{HEADER}");
    for mode in modes {
        for (index, quantity) in QUANTITIES.iter().enumerate() {
            let pairs: Vec<_> = rows.iter().filter(|row| row.mode == mode).map(|row| row.pairs[index]).collect();
            let Some(cell) = compare_cell(&pairs, RESAMPLES, SEED) else {
                let _ = writeln!(out, "{mode}\t{quantity}\t-\tno_cell");
                continue;
            };
            let reading = match cell.reading {
                Reading::Negligible => "negligible",
                Reading::Matters => "matters",
                Reading::Undecided => "undecided",
            };
            let _ = writeln!(
                out,
                "{mode}\t{quantity}\t{}\t{:.4}\t{:.4}\t{:.4}\t{:.4e}\t{:.4e}\t{:.4e}\t{:.4e}\t{:.4}\t{:.4}\t{:.4}\t{reading}",
                cell.ratio.rows,
                cell.ratio.median,
                cell.ratio.median_interval.0,
                cell.ratio.median_interval.1,
                cell.difference.median,
                cell.difference.median_interval.0,
                cell.difference.median_interval.1,
                cell.stored_iqr,
                cell.shift.0,
                cell.shift.1.0,
                cell.shift.1.1,
            );
        }
    }
    Ok(out)
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [path] = arguments.as_slice() else {
        eprintln!("usage: event-ratios <stored-events.tsv>");
        return ExitCode::FAILURE;
    };
    match std::fs::read_to_string(path)
        .map_err(|error| format!("cannot read {path}: {error}"))
        .and_then(|text| run(&text))
    {
        Ok(out) => {
            print!("{out}");
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
    fn every_row_has_a_cell_per_header_column() {
        let header = "record\tmode\tcount_ours\tcount_stored\tmean_duration_ours[s]\tmean_duration_stored[s]\tfrequency_ours[1/s]\tfrequency_stored[1/s]";
        let rows: Vec<String> = (1..=6).map(|v| format!("{v}\tforward\t{}\t{v}\t1e0\t{v}e0\t1e-1\t{v}e-1", v - 1)).collect();
        let out = run(&format!("{header}\n{}", rows.join("\n"))).unwrap();
        let lines: Vec<&str> = out.lines().skip(2).collect();
        assert_eq!(lines.len(), QUANTITIES.len());
        assert!(lines.iter().all(|line| line.split('\t').count() == HEADER.split('\t').count()));
    }
}
