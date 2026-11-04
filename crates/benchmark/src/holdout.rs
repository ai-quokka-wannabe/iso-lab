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

//! Real worms against unseen real worms (STATE.md item 2 (e3)): two blocks' rows of measures,
//! as `profiles` writes them, compared by both of the benchmark's tests, and a positive control
//! in which one measure of the second block is moved by a stated amount.
//!
//! The rows are read from `profiles`' tab-separated output rather than from the recordings, so
//! that two blocks never need to be on disk at once. That output rounds every measure to six
//! decimals; both blocks are rounded alike.

use crate::compare::{CompareConfig, MeasureComparison, compare_measures};
use crate::{Config, Outcome, TestError, two_sample_test};

/// The context columns `profiles` writes before the measures: record, status, reason, rate,
/// window frames, window seconds, the two gap shares and the body length.
pub const CONTEXT_COLUMNS: usize = 9;

/// The measured rows of one block, and a count of each reason the others were excluded.
#[derive(Clone, Debug, PartialEq)]
pub struct Profiles {
    /// The record ids of the measured rows, in file order.
    pub records: Vec<String>,
    /// One row of measures per measured record, in the order of the header's measure columns.
    pub rows: Vec<Vec<f64>>,
    /// Each exclusion reason with its count, in order of first appearance.
    pub excluded: Vec<(String, usize)>,
}

/// Reads `profiles`' output. The header's measure columns must be exactly `names`, in order, so a
/// file written by another version of the measures is refused rather than misread. Only rows whose
/// status is `measured` are kept; a measured row with a measure that is not a finite number is an
/// error, never dropped.
pub fn read_profiles(text: &str, names: &[String]) -> Result<Profiles, String> {
    let mut lines = text.lines();
    let header: Vec<&str> = lines.next().ok_or("the file is empty")?.split('\t').collect();
    if header.len() != CONTEXT_COLUMNS + names.len() || header[..2] != ["record", "status"] {
        return Err(format!(
            "the header has {} columns; {} context and {} measure columns were expected",
            header.len(),
            CONTEXT_COLUMNS,
            names.len()
        ));
    }
    if let Some((found, wanted)) = header[CONTEXT_COLUMNS..].iter().zip(names).find(|(found, wanted)| *found != wanted) {
        return Err(format!("the header names the measure {found} where {wanted} was expected"));
    }
    let mut profiles = Profiles {
        records: Vec::new(),
        rows: Vec::new(),
        excluded: Vec::new(),
    };
    for (index, line) in lines.enumerate() {
        let cells: Vec<&str> = line.split('\t').collect();
        if cells.len() != header.len() {
            return Err(format!("line {} has {} cells, not {}", index + 2, cells.len(), header.len()));
        }
        if cells[1] != "measured" {
            match profiles.excluded.iter_mut().find(|(reason, _)| reason == cells[2]) {
                Some((_, count)) => *count += 1,
                None => profiles.excluded.push((cells[2].to_string(), 1)),
            }
            continue;
        }
        let row = cells[CONTEXT_COLUMNS..]
            .iter()
            .map(|cell| cell.parse::<f64>().ok().filter(|value| value.is_finite()))
            .collect::<Option<Vec<f64>>>()
            .ok_or_else(|| format!("line {} (record {}) has a measure that is not a finite number", index + 2, cells[0]))?;
        profiles.records.push(cells[0].to_string());
        profiles.rows.push(row);
    }
    Ok(profiles)
}

/// The sample standard deviation (divided by n - 1) of one column; `None` below two rows.
pub fn column_sd(rows: &[Vec<f64>], column: usize) -> Option<f64> {
    if rows.len() < 2 {
        return None;
    }
    let n = rows.len() as f64;
    let mean = rows.iter().map(|row| row[column]).sum::<f64>() / n;
    Some((rows.iter().map(|row| (row[column] - mean).powi(2)).sum::<f64>() / (n - 1.0)).sqrt())
}

/// The rows with `delta` added to one column of every row, and every other value unchanged: the
/// positive control's alteration.
pub fn shifted(rows: &[Vec<f64>], column: usize, delta: f64) -> Vec<Vec<f64>> {
    rows.iter()
        .map(|row| {
            let mut row = row.clone();
            row[column] += delta;
            row
        })
        .collect()
}

/// Both tests on one pair of samples, at one seed.
#[derive(Clone, Debug, PartialEq)]
pub struct Verdict {
    /// The per-measure comparisons, one per column.
    pub measures: Vec<MeasureComparison>,
    /// The classifier test.
    pub classifier: Outcome,
}

impl Verdict {
    /// The columns the per-measure comparisons flag, after Holm's correction.
    pub fn flagged(&self) -> Vec<usize> {
        self.measures.iter().filter(|m| m.differs).map(|m| m.column).collect()
    }

    /// Whether neither test tells the samples apart.
    pub fn indistinguishable(&self) -> bool {
        self.flagged().is_empty() && !self.classifier.told_apart
    }
}

/// Runs the per-measure comparisons and the classifier test, each with its default settings
/// (999 shuffles, alpha 0.05; the classifier's 21 neighbours and five folds) and the same seed.
pub fn both_tests(a: &[Vec<f64>], b: &[Vec<f64>], seed: u64) -> Result<Verdict, TestError> {
    Ok(Verdict {
        measures: compare_measures(a, b, &CompareConfig::default(), seed)?,
        classifier: two_sample_test(a, b, &Config::default(), seed)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SplitMix64;
    use crate::test_support::sample;

    fn names() -> Vec<String> {
        vec!["x[1]".to_string(), "y[rad]".to_string()]
    }

    const HEADER: &str = "record\tstatus\treason\tfps[1/s]\twindow_frames[1]\twindow[s]\tmissing[1]\teight[1]\tlength_L[um]\tx[1]\ty[rad]";

    fn file(rows: &[&str]) -> String {
        std::iter::once(HEADER).chain(rows.iter().copied()).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn only_measured_rows_are_kept_and_exclusions_are_counted() {
        let text = file(&[
            "1\tmeasured\t-\t30\t9\t0.3\t0.1\t0.9\t1000\t0.5\t1.5",
            "2\texcluded\tshort\t30\t9\t0.3\t0.1\t0.9\tNaN\tNaN\tNaN",
            "3\texcluded\tgappy\t30\t9\t0.3\t0.6\t0.4\tNaN\tNaN\tNaN",
            "4\texcluded\tshort\t30\t9\t0.3\t0.1\t0.9\tNaN\tNaN\tNaN",
            "5\tmeasured\t-\t30\t9\t0.3\t0.1\t0.9\t1000\t-0.25\t2",
        ]);
        let profiles = read_profiles(&text, &names()).unwrap();
        assert_eq!(profiles.records, ["1", "5"]);
        assert_eq!(profiles.rows, [vec![0.5, 1.5], vec![-0.25, 2.0]]);
        assert_eq!(profiles.excluded, [("short".to_string(), 2), ("gappy".to_string(), 1)]);
    }

    #[test]
    fn a_header_with_other_measures_is_refused() {
        let swapped = vec!["y[rad]".to_string(), "x[1]".to_string()];
        assert!(read_profiles(&file(&[]), &swapped).is_err());
        let fewer = vec!["x[1]".to_string()];
        assert!(read_profiles(&file(&[]), &fewer).is_err());
        assert!(read_profiles(&file(&[]), &names()).is_ok());
    }

    #[test]
    fn a_measured_row_without_a_value_is_an_error() {
        let text = file(&["1\tmeasured\t-\t30\t9\t0.3\t0.1\t0.9\t1000\tNaN\t1.5"]);
        assert!(read_profiles(&text, &names()).is_err());
        let short = file(&["1\tmeasured\t-\t30\t9\t0.3\t0.1\t0.9\t1000\t0.5"]);
        assert!(read_profiles(&short, &names()).is_err());
    }

    #[test]
    fn the_standard_deviation_divides_by_n_minus_one() {
        let rows = vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]];
        let sd = column_sd(&rows, 0).unwrap();
        assert!((sd - (5.0f64 / 3.0).sqrt()).abs() < 1e-15, "{sd}");
        assert_eq!(column_sd(&rows[..1], 0), None);
    }

    #[test]
    fn the_shift_moves_one_column_only() {
        let rows = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert_eq!(shifted(&rows, 1, 0.5), [vec![1.0, 2.5, 3.0], vec![4.0, 5.5, 6.0]]);
    }

    #[test]
    fn a_shifted_column_is_flagged_and_samples_of_one_distribution_are_not() {
        let mut rng = SplitMix64(4);
        let a = sample(&mut rng, 80, 3, 0.0, 1.0);
        let b = sample(&mut rng, 80, 3, 0.0, 1.0);
        let null = both_tests(&a, &b, 1).unwrap();
        assert!(null.indistinguishable(), "{null:?}");
        let control = both_tests(&a, &shifted(&b, 2, 2.0), 1).unwrap();
        assert_eq!(control.flagged(), [2]);
        assert!(control.classifier.told_apart, "{:?}", control.classifier);
        assert!(!control.indistinguishable());
        // The shuffles do not depend on the values, so the unshifted columns score as before.
        for column in 0..2 {
            assert_eq!(control.measures[column].p_value, null.measures[column].p_value);
        }
    }
}
