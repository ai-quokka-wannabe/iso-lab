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

//! `power-curves <a.tsv> <b.tsv>`: how large an alteration of each measure the benchmark's tests
//! can see (STATE.md item 2 (e4)). Each file is `profiles <block>`' output; only its measured rows
//! enter. The first file is the reference, whole; from the second, [`SAMPLE_SIZES`] rows are drawn
//! [`REPLICATES`] times each, and every measure of the drawn rows is moved by [`SIZES_SD`] standard
//! deviations of that measure over the first file, upwards and downwards
//! (`benchmark::power::power_study`). The plan it follows was written before it was run on the
//! blocks (docs/journal of 2026-11-08, *How Large an Alteration the Tests Can See: the Plan*).
//!
//! On standard output, one tab-separated table: lines beginning `#` with the settings, a header,
//! one row per sample size for the null (measure `none`, size 0), and one row per alteration
//! with each count and its exact 95% interval as a share.

use benchmark::clopper_pearson;
use benchmark::holdout::{Profiles, read_profiles};
use benchmark::power::{PowerConfig, power_study};
use features::profile::measure_names;
use std::fmt::Write as _;
use std::process::ExitCode;

/// The first seed, fixed in the plan; every draw's and test's seed derives from it.
const SEED: u64 = 20_261_108;
/// The alteration's sizes, in standard deviations of each measure over the first file.
const SIZES_SD: [f64; 4] = [0.25, 0.5, 1.0, 2.0];
/// How many rows of the second file enter, besides all of them.
const SAMPLE_SIZES: [usize; 3] = [20, 50, 100];
/// Replicates per sample size.
const REPLICATES: usize = 100;
/// Shuffles for the per-measure comparison's second run, beside the default's 999.
const MORE_PERMUTATIONS: usize = 9999;
/// Threads: the cloud machine's four CPUs are shared.
const THREADS: usize = 2;

const _: () = assert!(THREADS >= 1 && THREADS <= 2, "at most two of the four shared CPUs");
const _: () = assert!(MORE_PERMUTATIONS > 999, "the second run has more shuffles than the default");

fn read(path: &str, names: &[String]) -> Result<Profiles, String> {
    let text = std::fs::read_to_string(path).map_err(|error| format!("cannot read {path}: {error}"))?;
    read_profiles(&text, names).map_err(|error| format!("{path}: {error}"))
}

fn share(count: usize, of: usize) -> String {
    let (lower, upper) = clopper_pearson(count, of);
    format!("{count}\t{:.3}\t{:.3}\t{:.3}", count as f64 / of as f64, lower, upper)
}

fn report(path_a: &str, path_b: &str) -> Result<String, String> {
    let names = measure_names();
    let a = read(path_a, &names)?;
    let b = read(path_b, &names)?;
    let mut sample_sizes = SAMPLE_SIZES.to_vec();
    if !sample_sizes.contains(&b.rows.len()) {
        sample_sizes.push(b.rows.len());
    }
    let config = PowerConfig {
        sizes_sd: SIZES_SD.to_vec(),
        sample_sizes,
        replicates: REPLICATES,
        seed: SEED,
        more_permutations: MORE_PERMUTATIONS,
    };
    let power = power_study(&a.rows, &b.rows, &config, THREADS)?;

    let mut out = String::new();
    let _ = writeln!(
        out,
        "# power-curves: a = {path_a} ({} measured), b = {path_b} ({} measured)",
        a.rows.len(),
        b.rows.len()
    );
    let _ = writeln!(
        out,
        "# seed {SEED}; sizes {:?} sd of a; sample sizes {:?} of b; {REPLICATES} replicates; per-measure shuffles 999 and {MORE_PERMUTATIONS}; Holm, alpha 0.05; classifier 21 neighbours, 5 folds, 999 shuffles, alpha 0.05",
        config.sizes_sd, config.sample_sizes
    );
    let _ = writeln!(
        out,
        "sample_size\tmeasure\tdirection\tsize_sd\tdelta\treplicates\tflagged\tflagged_share\tflagged_lo\tflagged_hi\tflagged_more\tflagged_more_share\tflagged_more_lo\tflagged_more_hi\tother_flagged\tother_flagged_share\tother_flagged_lo\tother_flagged_hi\ttold_apart\ttold_apart_share\ttold_apart_lo\ttold_apart_hi"
    );
    for null in &power.nulls {
        let _ = writeln!(
            out,
            "{}\tnone\tnone\t0\t0\t{}\t{}\t{}\t{}\t{}",
            null.sample_size,
            null.replicates,
            share(null.any_flagged, null.replicates),
            share(null.any_flagged_more, null.replicates),
            share(0, null.replicates),
            share(null.told_apart, null.replicates)
        );
    }
    for cell in &power.cells {
        let _ = writeln!(
            out,
            "{}\t{}\t{}\t{}\t{:.6}\t{}\t{}\t{}\t{}\t{}",
            cell.sample_size,
            names[cell.column],
            if cell.direction > 0 { "up" } else { "down" },
            cell.size_sd,
            cell.delta,
            cell.replicates,
            share(cell.flagged, cell.replicates),
            share(cell.flagged_more, cell.replicates),
            share(cell.other_flagged, cell.replicates),
            share(cell.told_apart, cell.replicates)
        );
    }
    Ok(out)
}

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [a, b] = &arguments[..] else {
        eprintln!("usage: power-curves <profiles-a.tsv> <profiles-b.tsv>");
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
    fn the_settings_are_in_range() {
        assert!(SIZES_SD.iter().all(|&s| s > 0.0));
        assert!(SIZES_SD.windows(2).all(|w| w[0] < w[1]));
        // The classifier needs 21 neighbours outside a fold; the smallest draw against a block of
        // about 180 leaves far more.
        assert!(SAMPLE_SIZES.iter().all(|&n| n >= 5));
        assert!(SAMPLE_SIZES.windows(2).all(|w| w[0] < w[1]));
    }

    #[test]
    fn every_row_has_a_cell_per_header_column() {
        let header_cells = 22;
        assert_eq!(share(3, 10).split('\t').count(), 4);
        // 6 leading columns and four shares of 4 cells each.
        assert_eq!(6 + 4 * 4, header_cells);
    }
}
