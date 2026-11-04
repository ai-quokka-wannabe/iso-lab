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

//! `compare-blocks <a.tsv> <b.tsv>`: real worms against unseen real worms (STATE.md item 2 (e3)).
//! Each file is `profiles <block>`' output; only its measured rows enter. The plan it follows was
//! written before it was run on block 2 (docs/journal of 2026-11-04, *Real Against Unseen Real:
//! the Plan*).
//!
//! On standard output, in sections under lines beginning `#`:
//!
//! - **the null comparison**: both of the benchmark's tests (`benchmark::holdout::both_tests`) on
//!   the two files' rows, at [`SEED`], each measure's comparison in a table;
//! - **the positive control**: the same, with [`CONTROL_MEASURE`] of every row of the second file
//!   moved up by [`CONTROL_SDS`] standard deviations of that measure over the first file;
//! - **the seeds beside them**: both comparisons at [`EXTRA_SEEDS`] further seeds, counted, as a
//!   description of how much the verdicts depend on the seed; the decision is the primary seed's.

use benchmark::holdout::{Profiles, Verdict, both_tests, column_sd, read_profiles, shifted};
use features::profile::measure_names;
use std::fmt::Write as _;
use std::process::ExitCode;

/// The seed of the decision, fixed in the plan.
const SEED: u64 = 20_261_104;
/// The seeds after [`SEED`] at which both comparisons are repeated, as a description only.
const EXTRA_SEEDS: u64 = 20;
/// The measure the positive control moves.
const CONTROL_MEASURE: &str = "eigen1_amplitude_sd[rad]";
/// How far it moves, in standard deviations of that measure over the first sample.
const CONTROL_SDS: f64 = 1.0;

fn read(path: &str, names: &[String]) -> Result<Profiles, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("cannot read {path}: {error}"))?;
    read_profiles(&text, names).map_err(|error| format!("{path}: {error}"))
}

fn verdict_table(out: &mut String, names: &[String], verdict: &Verdict) {
    let _ = writeln!(out, "measure\tstatistic_T\tp\tholm_p\tdiffers\tmedian_a\tiqr_a\tmedian_b\tiqr_b");
    for m in &verdict.measures {
        let _ = writeln!(
            out,
            "{}\t{:.6}\t{:.3}\t{:.3}\t{}\t{:.6}\t{:.6}\t{:.6}\t{:.6}",
            names[m.column], m.statistic, m.p_value, m.adjusted_p, m.differs, m.median_a, m.iqr_a, m.median_b, m.iqr_b
        );
    }
    let flagged: Vec<&str> = verdict.flagged().iter().map(|&c| names[c].as_str()).collect();
    let smallest = verdict.measures.iter().map(|m| m.p_value).fold(1.0, f64::min);
    let _ = writeln!(
        out,
        "per-measure: {} of {} differ{}; smallest raw p {smallest:.3}",
        flagged.len(),
        names.len(),
        if flagged.is_empty() { String::new() } else { format!(" ({})", flagged.join(", ")) }
    );
    let c = &verdict.classifier;
    let _ = writeln!(
        out,
        "classifier: rows {} and {}, accuracy {:.4}, p {:.3}, told apart {}",
        c.rows_a, c.rows_b, c.accuracy, c.p_value, c.told_apart
    );
}

fn report(path_a: &str, path_b: &str) -> Result<String, String> {
    let names = measure_names();
    let a = read(path_a, &names)?;
    let b = read(path_b, &names)?;
    let column = names
        .iter()
        .position(|n| n == CONTROL_MEASURE)
        .ok_or("the control's measure is not among the measures")?;
    let sd = column_sd(&a.rows, column).ok_or("the first sample has too few rows")?;
    let delta = CONTROL_SDS * sd;
    let altered = shifted(&b.rows, column, delta);

    let mut out = String::new();
    let _ = writeln!(out, "# compare-blocks: a = {path_a}, b = {path_b}; seed {SEED}");
    for (name, p) in [("a", &a), ("b", &b)] {
        let reasons: Vec<String> = p.excluded.iter().map(|(r, n)| format!("{r} {n}")).collect();
        let _ = writeln!(
            out,
            "# {name}: measured {}; excluded: {}",
            p.rows.len(),
            if reasons.is_empty() { "none".into() } else { reasons.join(", ") }
        );
    }
    let null = both_tests(&a.rows, &b.rows, SEED).map_err(|e| e.to_string())?;
    let _ = writeln!(out, "\n# the null comparison: a against b, {} measures", names.len());
    verdict_table(&mut out, &names, &null);
    let _ = writeln!(out, "null indistinguishable: {}", null.indistinguishable());

    let control = both_tests(&a.rows, &altered, SEED).map_err(|e| e.to_string())?;
    let _ = writeln!(
        out,
        "\n# the positive control: {CONTROL_MEASURE} of every row of b moved by {CONTROL_SDS} sd of a = {delta:.6}"
    );
    verdict_table(&mut out, &names, &control);
    let only = control.flagged() == [column];
    let _ = writeln!(
        out,
        "control told apart by both: {}; the control's measure flagged, and no other: {only}",
        control.classifier.told_apart && control.flagged().contains(&column)
    );
    let unchanged = null.measures.iter().zip(&control.measures).all(|(n, c)| n.column == column || n.p_value == c.p_value);
    let _ = writeln!(out, "every other measure's raw p equal to the null's: {unchanged}");

    let _ = writeln!(
        out,
        "\n# the seeds beside them, {} to {}: a description, not the decision",
        SEED + 1,
        SEED + EXTRA_SEEDS
    );
    let _ = writeln!(out, "seed\tnull_flagged\tnull_classifier_p\tcontrol_flagged\tcontrol_classifier_p");
    let (mut null_any, mut null_apart, mut control_measure, mut control_apart) = (0, 0, 0, 0);
    for seed in SEED + 1..=SEED + EXTRA_SEEDS {
        let n = both_tests(&a.rows, &b.rows, seed).map_err(|e| e.to_string())?;
        let c = both_tests(&a.rows, &altered, seed).map_err(|e| e.to_string())?;
        null_any += usize::from(!n.flagged().is_empty());
        null_apart += usize::from(n.classifier.told_apart);
        control_measure += usize::from(c.flagged().contains(&column));
        control_apart += usize::from(c.classifier.told_apart);
        let _ = writeln!(
            out,
            "{seed}\t{}\t{:.3}\t{}\t{:.3}",
            n.flagged().len(),
            n.classifier.p_value,
            c.flagged().len(),
            c.classifier.p_value
        );
    }
    let _ = writeln!(
        out,
        "of {EXTRA_SEEDS} seeds: null with any measure flagged {null_any}, null told apart by the classifier {null_apart}; control's measure flagged {control_measure}, control told apart by the classifier {control_apart}"
    );
    Ok(out)
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [a, b] = &arguments[..] else {
        eprintln!("usage: compare-blocks <profiles-a.tsv> <profiles-b.tsv>");
        return ExitCode::FAILURE;
    };
    match report(a, b) {
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
    fn the_control_measure_is_a_measure() {
        assert!(measure_names().iter().any(|n| n == CONTROL_MEASURE));
    }
}
