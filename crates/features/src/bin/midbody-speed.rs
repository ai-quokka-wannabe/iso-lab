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

//! The midbody's speed along the body (`features::locomotion`) in the reference recordings. Run
//! from the repository's root.
//!
//! `cargo run --release --bin midbody-speed`: for every features file fetched into
//! `data/raw/owmd-n2/`, in name order, one line: record, frame rate, half span in frames,
//! frames, frames with a speed, median length in micrometres, median absolute speed in
//! micrometres per second and in lengths per second, and the share of frames with a speed whose
//! speed is negative (towards the tail).
//!
//! `cargo run --release --bin midbody-speed -- <record>`: that record's speed at every frame, one
//! line a frame, `NaN` where it has none, for comparison with the dataset's own table.

use features::locomotion::{half_span_frames, midbody_speeds};
use features::posture::eight_arcs;
use features::read_skeletons;
use std::path::Path;
use std::process::ExitCode;

const DIRECTORY: &str = "data/raw/owmd-n2";
const SUFFIX: &str = "_features.hdf5";

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    match values.len() {
        0 => f64::NAN,
        n if n % 2 == 1 => values[n / 2],
        n => (values[n / 2 - 1] + values[n / 2]) / 2.0,
    }
}

fn main() -> ExitCode {
    let only = std::env::args().nth(1);
    let entries = match std::fs::read_dir(DIRECTORY) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("cannot read {DIRECTORY}: {error}");
            return ExitCode::FAILURE;
        }
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .filter(|name| name.ends_with(SUFFIX))
        .filter(|name| only.as_ref().is_none_or(|record| name.trim_end_matches(SUFFIX) == record))
        .collect();
    names.sort();
    if names.is_empty() {
        eprintln!("no features file in {DIRECTORY} matches");
        return ExitCode::FAILURE;
    }
    if only.is_none() {
        println!("record\tfps\thalf_span\tframes\twith_speed\tmedian_length_um\tmedian_abs_speed_um_s\tmedian_abs_speed_L_s\tbackward");
    } else {
        println!("frame\tspeed_um_s");
    }
    for name in names {
        let record = name.trim_end_matches(SUFFIX);
        let skeletons = match read_skeletons(&Path::new(DIRECTORY).join(&name)) {
            Ok(skeletons) => skeletons,
            Err(error) => {
                eprintln!("{record}: {error}");
                return ExitCode::FAILURE;
            }
        };
        let arcs: Vec<_> = (0..skeletons.frames()).map(|index| skeletons.frame(index).and_then(eight_arcs)).collect();
        let speeds = midbody_speeds(&arcs, skeletons.fps);
        if only.is_some() {
            for (frame, speed) in speeds.iter().enumerate() {
                println!("{frame}\t{}", speed.map_or("NaN".to_string(), |speed| format!("{speed:.4}")));
            }
            continue;
        }
        let mut lengths: Vec<f64> = arcs.iter().flatten().map(|arcs| arcs.length).collect();
        let length = median(&mut lengths);
        let mut magnitudes: Vec<f64> = speeds.iter().flatten().map(|speed| speed.abs()).collect();
        let backward = speeds.iter().flatten().filter(|&&speed| speed < 0.0).count();
        let with_speed = magnitudes.len();
        let speed = median(&mut magnitudes);
        let share = if with_speed == 0 { f64::NAN } else { backward as f64 / with_speed as f64 };
        println!(
            "{record}\t{:.2}\t{}\t{}\t{with_speed}\t{length:.1}\t{speed:.2}\t{:.4}\t{share:.4}",
            skeletons.fps,
            half_span_frames(skeletons.fps),
            skeletons.frames(),
            speed / length
        );
    }
    ExitCode::SUCCESS
}
