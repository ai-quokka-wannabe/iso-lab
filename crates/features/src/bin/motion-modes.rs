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

//! `motion-modes <fps>`: reads one frame a line from standard input, its midbody speed and body
//! length separated by a tab (`NaN` where undefined), and writes each frame's locomotion state
//! (`features::states`): 1 forward, -1 backward, 0 paused, `NaN` none; an infinite value or one
//! beyond a double's range is read as undefined. For checking the rule
//! against a table the lab's Rust cannot read, such as the reference files' published features.

use features::states::{MotionMode, motion_modes};
use std::io::{BufRead, Write};
use std::process::ExitCode;

fn parse(field: &str) -> Result<Option<f64>, String> {
    if field == "NaN" {
        return Ok(None);
    }
    let value: f64 = field.parse().map_err(|_| format!("not a number: {field:?}"))?;
    Ok(value.is_finite().then_some(value))
}

fn main() -> ExitCode {
    let fps = match std::env::args().nth(1).map(|fps| fps.parse::<f64>()) {
        Some(Ok(fps)) if fps > 0.0 && fps.is_finite() => fps,
        _ => {
            eprintln!("usage: motion-modes <fps>, a positive frame rate, with speed<TAB>length lines on standard input");
            return ExitCode::FAILURE;
        }
    };
    let (mut speeds, mut lengths) = (Vec::new(), Vec::new());
    for (number, line) in std::io::stdin().lock().lines().enumerate() {
        let parsed = line
            .map_err(|error| error.to_string())
            .and_then(|line| match line.split('\t').collect::<Vec<_>>()[..] {
                [speed, length] => Ok((parse(speed)?, parse(length)?)),
                _ => Err("expected two tab-separated fields".to_string()),
            });
        match parsed {
            Ok((speed, length)) => {
                speeds.push(speed);
                lengths.push(length);
            }
            Err(error) => {
                eprintln!("line {}: {error}", number + 1);
                return ExitCode::FAILURE;
            }
        }
    }
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    for mode in motion_modes(&speeds, &lengths, fps) {
        let text = match mode {
            Some(MotionMode::Forward) => "1",
            Some(MotionMode::Backward) => "-1",
            Some(MotionMode::Paused) => "0",
            None => "NaN",
        };
        if writeln!(out, "{text}").is_err() {
            return ExitCode::FAILURE;
        }
    }
    if out.flush().is_err() { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
