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

//! The lab's speed against the published one, per locomotion state (STATE.md item 2 (c)): from
//! `stored-events --lab-speed`'s rows, the median ratio of the lab's value to the stored one with a
//! bootstrap interval, and the paired shift in units of the stored values' spread, for each mode's
//! count, mean duration and frequency. The plan, and the threshold, were written before the run
//! (docs/journal of 2026-11-05, *Item 2 (c)'s Follow-Ups: the Plan, Written Before the Numbers*).
//!
//! A cell's recordings are those where both values are defined and the stored one is above zero,
//! as `stored-events` takes them for its medians. The ratio's and the difference's medians come
//! with percentile intervals from [`crate::reference::summarise`], which resamples whole
//! recordings; the shift is the difference's median and interval divided by the interquartile
//! range of the stored values over the same recordings, held fixed.

use crate::compare::quantile;
use crate::reference::{Summary, summarise};

/// The size of shift, in interquartile ranges of the stored values, below which a cell is
/// negligible: a judgement fixed in the plan before the run, to be replaced by item 2 (e4)'s answer.
pub const THRESHOLD_IQR: f64 = 0.25;

/// The three quantities compared per mode, in the order of [`Row::pairs`].
pub const QUANTITIES: [&str; 3] = ["count", "mean_duration", "frequency"];

/// One recording-mode of `stored-events`' output: the lab's value and the stored one, `None` where
/// undefined.
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub record: String,
    pub mode: String,
    /// The count, the mean duration and the frequency, each as (the lab's, stored).
    pub pairs: [(Option<f64>, Option<f64>); 3],
}

fn cell(text: &str) -> Result<Option<f64>, String> {
    match text {
        "NaN" => Ok(None),
        _ => text.parse::<f64>().map(Some).map_err(|_| format!("{text} is not a number")),
    }
}

/// Reads `stored-events`' output. The header must begin as `stored-events` writes it; rows of a
/// recording that was not fetched or could not be read (three cells) are skipped and counted.
/// Returns the rows and the number skipped.
pub fn read_rows(text: &str) -> Result<(Vec<Row>, usize), String> {
    const START: [&str; 8] = [
        "record",
        "mode",
        "count_ours",
        "count_stored",
        "mean_duration_ours[s]",
        "mean_duration_stored[s]",
        "frequency_ours[1/s]",
        "frequency_stored[1/s]",
    ];
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("the file is empty")?.split('\t').collect();
    if header.len() < START.len() || header[..START.len()] != START {
        return Err("the header is not that of stored-events".to_string());
    }
    let (mut rows, mut skipped) = (Vec::new(), 0);
    for (index, line) in lines.enumerate() {
        let cells: Vec<&str> = line.split('\t').collect();
        if cells.len() == 3 && cells[1] == "-" {
            skipped += 1;
            continue;
        }
        if cells.len() != header.len() {
            return Err(format!("line {} has {} cells, not {}", index + 2, cells.len(), header.len()));
        }
        let pair = |at: usize| -> Result<(Option<f64>, Option<f64>), String> { Ok((cell(cells[at])?, cell(cells[at + 1])?)) };
        rows.push(Row {
            record: cells[0].to_string(),
            mode: cells[1].to_string(),
            pairs: [pair(2)?, pair(4)?, pair(6)?],
        });
    }
    Ok((rows, skipped))
}

/// Where a cell's shift lies against [`THRESHOLD_IQR`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reading {
    /// The whole interval lies within ±[`THRESHOLD_IQR`].
    Negligible,
    /// The whole interval lies outside it, on one side.
    Matters,
    /// Neither.
    Undecided,
}

/// One mode's quantity, over its recordings.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    /// The lab's value over the stored one: its median, quartiles and the median's interval.
    pub ratio: Summary,
    /// The lab's value minus the stored one, likewise.
    pub difference: Summary,
    /// The stored values' interquartile range over the same recordings.
    pub stored_iqr: f64,
    /// The difference's median and interval divided by `stored_iqr`.
    pub shift: (f64, (f64, f64)),
    pub reading: Reading,
}

/// The reading of a shift's interval `(low, high)`, in interquartile ranges.
pub fn reading(interval: (f64, f64)) -> Reading {
    let (low, high) = interval;
    if low >= -THRESHOLD_IQR && high <= THRESHOLD_IQR {
        Reading::Negligible
    } else if low > THRESHOLD_IQR || high < -THRESHOLD_IQR {
        Reading::Matters
    } else {
        Reading::Undecided
    }
}

/// The cell of `pairs`, (the lab's, stored), over those where both are defined and the stored one
/// is above zero, with `resamples` resamples seeded by `seed`. `None` with fewer than two such
/// recordings, or when the stored values have no spread.
pub fn compare_cell(pairs: &[(Option<f64>, Option<f64>)], resamples: usize, seed: u64) -> Option<Cell> {
    let kept: Vec<(f64, f64)> = pairs
        .iter()
        .filter_map(|&pair| match pair {
            (Some(ours), Some(stored)) if stored > 0.0 => Some((ours, stored)),
            _ => None,
        })
        .collect();
    let rows: Vec<Vec<f64>> = kept.iter().map(|&(ours, stored)| vec![ours / stored, ours - stored]).collect();
    let mut summaries = summarise(&rows, resamples, seed)?.into_iter();
    let (ratio, difference) = (summaries.next()?, summaries.next()?);
    let mut stored: Vec<f64> = kept.iter().map(|&(_, stored)| stored).collect();
    stored.sort_by(f64::total_cmp);
    let stored_iqr = quantile(&stored, 0.75) - quantile(&stored, 0.25);
    if stored_iqr <= 0.0 {
        return None;
    }
    let interval = (difference.median_interval.0 / stored_iqr, difference.median_interval.1 / stored_iqr);
    Some(Cell {
        shift: (difference.median / stored_iqr, interval),
        reading: reading(interval),
        ratio,
        difference,
        stored_iqr,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEADER: &str = "record\tmode\tcount_ours\tcount_stored\tmean_duration_ours[s]\tmean_duration_stored[s]\tfrequency_ours[1/s]\tfrequency_stored[1/s]\tworst_duration_relative[1]\tfrequency_relative[1]\treproduced\tlone_edge_ours";

    #[test]
    fn rows_are_read_with_nan_as_undefined_and_missing_recordings_skipped() {
        let text =
            format!("{HEADER}\n1\tforward\t2\t4\t1e0\t2e0\t3e-1\t6e-1\tNaN\t1e0\tno\tno\n2\t-\tnot_fetched\n3\tpaused\t0\t0\tNaN\tNaN\tNaN\tNaN\t0e0\t0e0\tyes\tno");
        let (rows, skipped) = read_rows(&text).unwrap();
        assert_eq!(skipped, 1);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].pairs, [(Some(2.0), Some(4.0)), (Some(1.0), Some(2.0)), (Some(0.3), Some(0.6))]);
        assert_eq!(rows[1].pairs[1], (None, None));
        assert!(read_rows("record\tmode\n").is_err());
        assert!(read_rows(&format!("{HEADER}\n1\tforward\t2")).is_err());
    }

    #[test]
    fn a_shift_is_read_against_the_threshold_on_both_sides() {
        assert_eq!(reading((-0.2, 0.25)), Reading::Negligible);
        assert_eq!(reading((0.3, 0.9)), Reading::Matters);
        assert_eq!(reading((-0.9, -0.3)), Reading::Matters);
        assert_eq!(reading((0.1, 0.4)), Reading::Undecided);
        assert_eq!(reading((-0.4, 0.4)), Reading::Undecided);
    }

    #[test]
    fn a_cell_keeps_defined_positive_pairs_and_scales_the_shift_by_the_stored_spread() {
        // Stored 1..=8 (IQR 3.5 by type 7), the lab's value one less in each: ratio below 1, shift -1/3.5.
        let mut pairs: Vec<(Option<f64>, Option<f64>)> = (1..=8).map(|v| (Some(v as f64 - 1.0), Some(v as f64))).collect();
        pairs.extend([(None, Some(5.0)), (Some(5.0), None), (Some(1.0), Some(0.0))]);
        let cell = compare_cell(&pairs, 199, 7).unwrap();
        assert_eq!(cell.ratio.rows, 8);
        assert_eq!(cell.stored_iqr, 3.5);
        assert_eq!(cell.difference.median, -1.0);
        // Every difference is -1, so every resample's median is too.
        assert_eq!(cell.shift, (-1.0 / 3.5, (-1.0 / 3.5, -1.0 / 3.5)));
        assert_eq!(cell.reading, Reading::Matters);
        assert!(cell.ratio.median < 1.0);
        // Fewer than two recordings, or no spread: no cell.
        assert!(compare_cell(&pairs[..1], 199, 7).is_none());
        assert!(compare_cell(&[(Some(1.0), Some(2.0)), (Some(1.0), Some(2.0))], 199, 7).is_none());
    }
}
