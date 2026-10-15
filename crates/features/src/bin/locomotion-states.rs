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

//! `locomotion-states [--published-choices] <record>`: the locomotion state at every frame of one
//! reference recording fetched into `data/raw/owmd-n2/`, by the published rule
//! (`features::states`) applied to the lab's own eight-arc midbody speed (`features::locomotion`,
//! the published choices with the flag) and eight-arc body length. One line a frame: 1 forward,
//! -1 backward, 0 paused, `NaN` none. Run from the repository's root.

use features::locomotion::{midbody_speeds, published_choice_speeds};
use features::posture::eight_arcs;
use features::read_skeletons;
use features::states::{MotionMode, motion_modes};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const DIRECTORY: &str = "data/raw/owmd-n2";

fn main() -> ExitCode {
    let mut arguments: Vec<String> = std::env::args().skip(1).collect();
    let published = arguments.first().is_some_and(|argument| argument == "--published-choices");
    if published {
        arguments.remove(0);
    }
    let [record] = arguments.as_slice() else {
        eprintln!("usage: locomotion-states [--published-choices] <record>");
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
    let speeds = if published {
        published_choice_speeds(&arcs, skeletons.fps)
    } else {
        midbody_speeds(&arcs, skeletons.fps)
    };
    let lengths: Vec<Option<f64>> = arcs.iter().map(|arcs| arcs.as_ref().map(|arcs| arcs.length)).collect();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    for mode in motion_modes(&speeds, &lengths, skeletons.fps) {
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
