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

//! `locomotion-states [--published-choices] [--three-ends] [--columns] <record>`: the locomotion
//! state at every frame of one reference recording fetched into `data/raw/owmd-n2/`, by the published rule
//! (`features::states`) applied to the lab's own eight-arc midbody speed (`features::locomotion`,
//! the published choices with the flag) and eight-arc body length. One line a frame: 1 forward,
//! -1 backward, 0 paused, `NaN` none. With `--columns`, each line is the frame's speed, its
//! eight-arc length and its state, tab-separated, `NaN` where undefined. `--three-ends` takes the
//! published-choice speed with the midbody as the mean of the three middle ends of the eight arcs
//! (`locomotion::Midbody::ThreeMiddleEnds`). Run from the repository's root.

use features::locomotion::{Midbody, midbody_speeds, published_choice_speeds_with};
use features::posture::eight_arcs;
use features::read_skeletons;
use features::states::{MotionMode, motion_modes};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const DIRECTORY: &str = "data/raw/owmd-n2";

fn main() -> ExitCode {
    let mut arguments: Vec<String> = std::env::args().skip(1).collect();
    let mut flag = |name: &str| match arguments.iter().position(|argument| argument == name) {
        Some(at) => {
            arguments.remove(at);
            true
        }
        None => false,
    };
    let (published, three, columns) = (flag("--published-choices"), flag("--three-ends"), flag("--columns"));
    let [record] = arguments.as_slice() else {
        eprintln!("usage: locomotion-states [--published-choices] [--three-ends] [--columns] <record>");
        return ExitCode::FAILURE;
    };
    let path = Path::new(DIRECTORY).join(format!("{record}_features.hdf5"));
    let skeletons = match read_skeletons(&path) {
        Ok(skeletons) => skeletons,
        Err(error) => {
            eprintln!("{record}: {error}");
            return ExitCode::FAILURE;
        }
    };
    let arcs: Vec<_> = (0..skeletons.frames()).map(|index| skeletons.frame(index).and_then(eight_arcs)).collect();
    let speeds = match (published, three) {
        (_, true) => published_choice_speeds_with(&arcs, skeletons.fps, Midbody::ThreeMiddleEnds),
        (true, false) => published_choice_speeds_with(&arcs, skeletons.fps, Midbody::MiddleEnd),
        (false, false) => midbody_speeds(&arcs, skeletons.fps),
    };
    let lengths: Vec<Option<f64>> = arcs.iter().map(|arcs| arcs.as_ref().map(|arcs| arcs.length)).collect();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let number = |value: Option<f64>| value.map_or("NaN".to_string(), |value| format!("{value:?}"));
    for (frame, mode) in motion_modes(&speeds, &lengths, skeletons.fps).into_iter().enumerate() {
        let text = match mode {
            Some(MotionMode::Forward) => "1",
            Some(MotionMode::Backward) => "-1",
            Some(MotionMode::Paused) => "0",
            None => "NaN",
        };
        let written = if columns {
            writeln!(out, "{}\t{}\t{text}", number(speeds[frame]), number(lengths[frame]))
        } else {
            writeln!(out, "{text}")
        };
        if written.is_err() {
            return ExitCode::FAILURE;
        }
    }
    if out.flush().is_err() { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
