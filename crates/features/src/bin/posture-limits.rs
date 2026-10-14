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

//! How often the reference worms bend further than the Grid's worm can: for every features file
//! fetched into `data/raw/owmd-n2/`, in name order, the share of its skeletonised frames in which
//! any of the seven joints of the eight-arc posture bends beyond the Grid's 0.9 rad limit, and
//! beyond the world's ceiling of pi/2 (docs/benchmark/FEATURES.md, *The Measures*, the caveat on
//! bend angles). Run from the repository's root: `cargo run --release --bin posture-limits`.
//! One line a recording: record, frame rate, frames, skeletonised frames, frames with eight
//! arcs, median length in micrometres, share beyond 0.9 rad, share beyond pi/2, and then the share
//! beyond 0.9 rad at each of the seven joints, head first.
//!
//! An optional argument, a half-width in skeleton points, first smooths every skeleton along its
//! length by `posture::smoothed_along`: `cargo run --release --bin posture-limits -- 2`. Without it,
//! or with 0, the skeletons are used as recorded.

use features::posture::{ARCS, COS_GRID_JOINT_LIMIT, eight_arcs, smoothed_along};
use features::read_skeletons;
use std::path::Path;
use std::process::ExitCode;

const DIRECTORY: &str = "data/raw/owmd-n2";
const SUFFIX: &str = "_features.hdf5";

fn main() -> ExitCode {
    let half_width = match std::env::args().nth(1).map(|argument| argument.parse::<usize>()) {
        None => 0,
        Some(Ok(half_width)) => half_width,
        Some(Err(error)) => {
            eprintln!("the argument is a half-width in skeleton points: {error}");
            return ExitCode::FAILURE;
        }
    };
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
        .collect();
    names.sort();
    let joints: String = (1..ARCS).map(|joint| format!("\tjoint{joint}_beyond_0.9rad")).collect();
    println!("record\tfps\tframes\tskeletonised\teight_arcs\tmedian_length_um\tbeyond_0.9rad\tbeyond_pi/2{joints}");
    for name in names {
        let record = name.trim_end_matches(SUFFIX);
        let skeletons = match read_skeletons(&Path::new(DIRECTORY).join(&name)) {
            Ok(skeletons) => skeletons,
            Err(error) => {
                eprintln!("{record}: {error}");
                return ExitCode::FAILURE;
            }
        };
        let (mut skeletonised, mut beyond_limit, mut beyond_ceiling) = (0_usize, 0_usize, 0_usize);
        let mut per_joint = [0_usize; ARCS - 1];
        let mut lengths = Vec::new();
        for frame in (0..skeletons.frames()).filter_map(|index| skeletons.frame(index)) {
            skeletonised += 1;
            let Some(arcs) = eight_arcs(&smoothed_along(frame, half_width)) else { continue };
            lengths.push(arcs.length);
            beyond_limit += usize::from(arcs.bends_beyond(COS_GRID_JOINT_LIMIT));
            beyond_ceiling += usize::from(arcs.bends_beyond(0.0));
            for (count, cosine) in per_joint.iter_mut().zip(arcs.joint_cosines()) {
                *count += usize::from(cosine < COS_GRID_JOINT_LIMIT);
            }
        }
        lengths.sort_by(f64::total_cmp);
        let median = match lengths.len() {
            0 => f64::NAN,
            n if n % 2 == 1 => lengths[n / 2],
            n => (lengths[n / 2 - 1] + lengths[n / 2]) / 2.0,
        };
        let share = |count: usize| if lengths.is_empty() { f64::NAN } else { count as f64 / lengths.len() as f64 };
        let joints: String = per_joint.iter().map(|&count| format!("\t{:.5}", share(count))).collect();
        println!(
            "{record}\t{:.2}\t{}\t{skeletonised}\t{}\t{median:.1}\t{:.5}\t{:.5}{joints}",
            skeletons.fps,
            skeletons.frames(),
            lengths.len(),
            share(beyond_limit),
            share(beyond_ceiling)
        );
    }
    ExitCode::SUCCESS
}
