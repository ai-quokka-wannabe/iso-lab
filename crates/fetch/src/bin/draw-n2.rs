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

//! `draw-n2` - puts the eligible manifest's records ([`ELIGIBLE`]) in the order the seeded draw
//! gives (`fetch::draw`) and writes that order to [`ORDER`], each record with its position and
//! its block. Run from the repository's root. The seed and the block size are constants here,
//! as the journal fixed them before the draw (2026-10-14, *The Reference Draw, Written Down
//! First*); nothing is taken from the command line. The manifest's lines are sorted by record
//! id first, so the order depends on the seed alone.

use fetch::draw::permutation;
use std::process::ExitCode;

const ELIGIBLE: &str = "data/manifests/owmd-n2-eligible.tsv";
const ORDER: &str = "data/manifests/owmd-n2-order.tsv";
const SEED: u64 = 20_261_014;
const BLOCK: usize = 200;

fn run() -> Result<String, String> {
    let text = std::fs::read_to_string(ELIGIBLE).map_err(|error| format!("reading {ELIGIBLE}: {error}"))?;
    let mut lines = text.lines();
    let header = lines.next().ok_or("the manifest is empty")?;
    let mut rows: Vec<(u64, &str)> = lines
        .enumerate()
        .map(|(index, line)| {
            let id = line
                .split('\t')
                .next()
                .and_then(|id| id.parse().ok())
                .ok_or_else(|| format!("no record id on line {}", index + 2))?;
            Ok((id, line))
        })
        .collect::<Result<_, String>>()?;
    rows.sort_unstable_by_key(|&(id, _)| id);
    let mut order = format!("position\tblock\t{header}\n");
    for (index, (_, line)) in permutation(&rows, SEED).iter().enumerate() {
        order.push_str(&format!("{}\t{}\t{line}\n", index + 1, index / BLOCK + 1));
    }
    std::fs::write(ORDER, order).map_err(|error| format!("writing {ORDER}: {error}"))?;
    Ok(format!("{} records ordered with seed {SEED}, in blocks of {BLOCK}, written to {ORDER}", rows.len()))
}

fn main() -> ExitCode {
    if std::env::args().len() > 1 {
        eprintln!("usage: draw-n2 (no arguments; it reads {ELIGIBLE} and writes {ORDER})");
        return ExitCode::FAILURE;
    }
    match run() {
        Ok(message) => {
            eprintln!("{message}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
