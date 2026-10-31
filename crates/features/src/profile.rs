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

//! One recording's measures as one row (STATE.md item 2 (d1)), built only from the measures the
//! crate already has, in the units of docs/benchmark/FEATURES.md, *Common Units*. The rules are
//! those written down before any row was computed (docs/journal of 2026-10-31, *One Row of
//! Measures per Recording: the Rules, Written Before the Data*).
//!
//! **The window.** A recording is measured over its first [`WINDOW_SECONDS`], the protocol's
//! fifteen minutes, or all of it when shorter; the frame rate is not resampled, as every rule
//! below is stated in seconds. It is excluded as [`Exclusion::Short`] when that window lasts less
//! than [`MIN_SECONDS`], and as [`Exclusion::Gappy`] when fewer than [`MIN_EIGHT_ARC_SHARE`] of
//! the window's frames have eight arcs.
//!
//! **The measures**, each graded in FEATURES.md, *The Measures*:
//!
//! - posture, the inputs of the eigen-shape amplitudes (*Comparable*): the standard deviation over
//!   the window of each of the eight mean-removed arc angles, in radians;
//! - the midbody speed (*Comparable*), with the published choices
//!   ([`crate::locomotion::published_choice_speeds`]), in body lengths per second: its mean and the
//!   mean of its absolute value;
//! - the forward, backward and paused states (*Comparable*): the share of the window's frames in
//!   each and in none ([`crate::states::motion_modes`]);
//! - their durations and rates (*Comparable* by grade, but **provisional** until STATE.md item
//!   2(c)'s check of the published durations passes): per mode, the number of events, their mean
//!   duration and their frequency ([`crate::states::event_stats`]).
//!
//! Beside them stand context columns that are not measures: the share of frames without a
//! skeleton, the share with eight arcs, and the median eight-arc length that defines the body
//! length.
//!
//! The arc angles need `atan2`, a transcendental the rest of the crate avoids: a posture column can
//! differ between two machines' libms in its last bits, and is the same to the bit on one machine.

use crate::Skeletons;
use crate::locomotion::published_choice_speeds;
use crate::posture::{ARCS, EightArcs, eight_arcs};
use crate::states::{MotionMode, event_stats, motion_events, motion_modes};

/// The window a recording is measured over: the protocol's fifteen minutes (Yemini et al. 2013).
pub const WINDOW_SECONDS: f64 = 900.0;

/// The shortest window measured: 95% of [`WINDOW_SECONDS`].
pub const MIN_SECONDS: f64 = 855.0;

/// The smallest share of the window's frames that must have eight arcs.
pub const MIN_EIGHT_ARC_SHARE: f64 = 0.5;

/// What a recording is, measured or not: the columns that are not measures.
#[derive(Clone, PartialEq, Debug)]
pub struct Context {
    /// Frames per second.
    pub fps: f64,
    /// Frames in the window.
    pub frames: usize,
    /// The window's length in seconds, `frames / fps`.
    pub seconds: f64,
    /// The share of the window's frames without a skeleton.
    pub missing_share: f64,
    /// The share of the window's frames with eight arcs.
    pub eight_arc_share: f64,
}

/// Why a recording is not measured.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Exclusion {
    /// Its window lasts less than [`MIN_SECONDS`].
    Short,
    /// Fewer than [`MIN_EIGHT_ARC_SHARE`] of its window's frames have eight arcs.
    Gappy,
}

impl Exclusion {
    /// The word written in the reason column.
    pub fn reason(self) -> &'static str {
        match self {
            Self::Short => "short",
            Self::Gappy => "gappy",
        }
    }
}

/// One mode's events over the window. Provisional: see the module's documentation.
#[derive(Clone, PartialEq, Debug)]
pub struct ModeEvents {
    /// How many events the mode has.
    pub events: usize,
    /// Their mean duration in seconds; `None` with no event.
    pub mean_duration: Option<f64>,
    /// Events per second, by the toolbox's count; `None` with no event.
    pub frequency: Option<f64>,
}

/// The measures of one recording.
#[derive(Clone, PartialEq, Debug)]
pub struct Measures {
    /// The median eight-arc length over the window, the body length L, in the skeletons' units.
    pub length: f64,
    /// The standard deviation of each mean-removed arc angle, head first, in radians.
    pub angle_spread: [f64; ARCS],
    /// The mean midbody speed in L per second, positive towards the head; `None` without a speed.
    pub speed_mean: Option<f64>,
    /// The mean absolute midbody speed in L per second; `None` without a speed.
    pub speed_abs_mean: Option<f64>,
    /// The share of the window's frames forward, backward, paused and in no state.
    pub state_shares: [f64; 4],
    /// The events of forward, backward and paused, in that order.
    pub modes: [ModeEvents; 3],
}

/// One recording's row: its context, and its measures or why it has none.
#[derive(Clone, PartialEq, Debug)]
pub struct Profile {
    pub context: Context,
    pub measures: Result<Measures, Exclusion>,
}

/// The eight arc angles of one posture, unwrapped along the body and with their mean removed, in
/// radians. Each angle is its chord's direction; the turn from one chord to the next is taken in
/// (-pi, pi], so the result does not depend on the body's heading.
pub fn mean_removed_angles(arcs: &EightArcs) -> [f64; ARCS] {
    let mut angles = [0.0; ARCS];
    for k in 1..ARCS {
        let (ax, ay) = arcs.chords[k - 1];
        let (bx, by) = arcs.chords[k];
        angles[k] = angles[k - 1] + (ax * by - ay * bx).atan2(ax * bx + ay * by);
    }
    let mean = angles.iter().sum::<f64>() / ARCS as f64;
    angles.map(|angle| angle - mean)
}

/// The sample standard deviation (divided by n - 1); `NaN` below two values.
fn standard_deviation(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return f64::NAN;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    (values.iter().map(|value| (value - mean) * (value - mean)).sum::<f64>() / (values.len() - 1) as f64).sqrt()
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    match values.len() {
        0 => f64::NAN,
        n if n % 2 == 1 => values[n / 2],
        n => (values[n / 2 - 1] + values[n / 2]) / 2.0,
    }
}

/// The frames in the window of a recording of `frames` frames at `fps`.
pub fn window_frames(frames: usize, fps: f64) -> usize {
    frames.min((WINDOW_SECONDS * fps).round() as usize)
}

/// One recording's row, by the rules of the module's documentation.
pub fn profile(skeletons: &Skeletons) -> Profile {
    let fps = skeletons.fps;
    let frames = window_frames(skeletons.frames(), fps);
    let arcs: Vec<Option<EightArcs>> = (0..frames).map(|index| skeletons.frame(index).and_then(eight_arcs)).collect();
    let share = |count: usize| if frames == 0 { f64::NAN } else { count as f64 / frames as f64 };
    let skeletonised = (0..frames).filter(|&index| skeletons.frame(index).is_some()).count();
    let with_arcs = arcs.iter().filter(|arcs| arcs.is_some()).count();
    let context = Context {
        fps,
        frames,
        seconds: frames as f64 / fps,
        missing_share: share(frames - skeletonised),
        eight_arc_share: share(with_arcs),
    };
    // An empty window is short, so the share below is never NaN when compared.
    let measures = if context.seconds < MIN_SECONDS {
        Err(Exclusion::Short)
    } else if context.eight_arc_share < MIN_EIGHT_ARC_SHARE {
        Err(Exclusion::Gappy)
    } else {
        Ok(measure(&arcs, fps))
    };
    Profile { context, measures }
}

fn measure(arcs: &[Option<EightArcs>], fps: f64) -> Measures {
    let frames = arcs.len();
    let present: Vec<&EightArcs> = arcs.iter().flatten().collect();
    let length = median(present.iter().map(|arcs| arcs.length).collect());
    let angles: Vec<[f64; ARCS]> = present.iter().map(|arcs| mean_removed_angles(arcs)).collect();
    let angle_spread = std::array::from_fn(|k| standard_deviation(&angles.iter().map(|angles| angles[k]).collect::<Vec<_>>()));

    let speeds = published_choice_speeds(arcs, fps);
    let in_lengths: Vec<f64> = speeds.iter().flatten().map(|speed| speed / length).collect();
    let mean = |values: &[f64]| (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64);
    let speed_mean = mean(&in_lengths);
    let speed_abs_mean = mean(&in_lengths.iter().map(|speed| speed.abs()).collect::<Vec<_>>());

    let lengths: Vec<Option<f64>> = arcs.iter().map(|arcs| arcs.as_ref().map(|arcs| arcs.length)).collect();
    let mut counts = [0_usize; 4];
    for mode in motion_modes(&speeds, &lengths, fps) {
        counts[match mode {
            Some(MotionMode::Forward) => 0,
            Some(MotionMode::Backward) => 1,
            Some(MotionMode::Paused) => 2,
            None => 3,
        }] += 1;
    }
    let state_shares = counts.map(|count| count as f64 / frames as f64);
    let modes = motion_events(&speeds, &lengths, fps).map(|(_, events)| {
        let stats = event_stats(&events, frames, fps);
        ModeEvents {
            events: events.len(),
            mean_duration: mean(&stats.durations),
            frequency: stats.frequency,
        }
    });
    Measures {
        length,
        angle_spread,
        speed_mean,
        speed_abs_mean,
        state_shares,
        modes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POINTS: usize = 49;
    const LENGTH_UM: f64 = 1000.0;

    /// A straight worm along x, head first at the larger x, its head at `head_x`.
    fn straight(head_x: f64) -> Vec<f32> {
        (0..POINTS)
            .flat_map(|i| [(head_x - LENGTH_UM * i as f64 / (POINTS - 1) as f64) as f32, 0.0])
            .collect()
    }

    /// A worm bent into a circular arc turning by `turn` radians from head to tail, its 49 points
    /// on the circle at equal angles, so that each eighth of the polyline ends on one of them.
    fn arc(turn: f64) -> Vec<f32> {
        let radius = LENGTH_UM / turn;
        (0..POINTS)
            .flat_map(|i| {
                let angle = turn * i as f64 / (POINTS - 1) as f64;
                [(radius * angle.sin()) as f32, (radius * (1.0 - angle.cos())) as f32]
            })
            .collect()
    }

    fn recording(fps: f64, frames: impl IntoIterator<Item = Vec<f32>>) -> Skeletons {
        Skeletons {
            fps,
            points: POINTS,
            xy: frames.into_iter().flatten().collect(),
        }
    }

    /// A straight worm sliding along its own axis at `speed` micrometres a second, for `seconds`.
    fn sliding(fps: f64, seconds: f64, speed: f64) -> Skeletons {
        let frames = (seconds * fps).round() as usize;
        recording(fps, (0..frames).map(|t| straight(speed * t as f64 / fps)))
    }

    fn close(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() <= tolerance
    }

    #[test]
    fn a_worm_crawling_forward_is_forward_throughout() {
        // 0.2 L/s, four times the forward threshold of 0.05 L/s.
        let profile = profile(&sliding(10.0, 900.0, 200.0));
        assert_eq!(
            (profile.context.frames, profile.context.missing_share, profile.context.eight_arc_share),
            (9000, 0.0, 1.0)
        );
        let measures = profile.measures.unwrap();
        assert!(close(measures.length, LENGTH_UM, 1e-3));
        assert!(close(measures.speed_mean.unwrap(), 0.2, 1e-6), "{:?}", measures.speed_mean);
        assert!(close(measures.speed_abs_mean.unwrap(), 0.2, 1e-6));
        assert_eq!(measures.state_shares, [1.0, 0.0, 0.0, 0.0]);
        // One forward event across the whole window: it lasts 900 s, and a lone event is counted.
        assert_eq!(measures.modes[0].events, 1);
        assert!(close(measures.modes[0].mean_duration.unwrap(), 900.0, 1e-9));
        assert!(close(measures.modes[0].frequency.unwrap(), 1.0 / 900.0, 1e-12));
        for mode in &measures.modes[1..] {
            assert_eq!((mode.events, mode.mean_duration, mode.frequency), (0, None, None));
        }
        assert!(measures.angle_spread.iter().all(|&spread| spread == 0.0), "{:?}", measures.angle_spread);
    }

    #[test]
    fn a_worm_crawling_backward_is_backward_and_its_speed_negative() {
        let measures = profile(&sliding(10.0, 900.0, -200.0)).measures.unwrap();
        assert!(close(measures.speed_mean.unwrap(), -0.2, 1e-6));
        assert!(close(measures.speed_abs_mean.unwrap(), 0.2, 1e-6));
        assert_eq!(measures.state_shares, [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(measures.modes[1].events, 1);
    }

    #[test]
    fn a_still_worm_is_paused() {
        let measures = profile(&sliding(10.0, 900.0, 0.0)).measures.unwrap();
        assert_eq!(measures.speed_mean, Some(0.0));
        assert_eq!(measures.state_shares, [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(measures.modes[2].events, 1);
    }

    #[test]
    fn alternating_postures_spread_each_angle_by_half_its_swing() {
        // Half the frames straight, half a circular arc turning by 1.6 rad: the mean-removed angle
        // of arc k is 0 or 1.6 ((k + 0.5) / 8 - 0.5), so over n frames, half each, its sample
        // standard deviation is |that| / 2 * sqrt(n / (n - 1)).
        let turn = 1.6;
        let frames = 9000;
        let skeletons = recording(10.0, (0..frames).map(|t| if t % 2 == 0 { straight(0.0) } else { arc(turn) }));
        let measures = profile(&skeletons).measures.unwrap();
        let scale = (frames as f64 / (frames - 1) as f64).sqrt();
        for (k, &spread) in measures.angle_spread.iter().enumerate() {
            let swing = turn * ((k as f64 + 0.5) / ARCS as f64 - 0.5);
            assert!(close(spread, swing.abs() / 2.0 * scale, 1e-5), "arc {k}: {spread} against {swing}");
        }
    }

    #[test]
    fn mean_removed_angles_ignore_the_heading() {
        // The same arc turned by a right angle gives the same angles.
        let bent = eight_arcs(&arc(1.0)).unwrap();
        let turned: Vec<f32> = arc(1.0).chunks(2).flat_map(|xy| [-xy[1], xy[0]]).collect();
        let a = mean_removed_angles(&bent);
        let b = mean_removed_angles(&eight_arcs(&turned).unwrap());
        assert!(a.iter().zip(b).all(|(a, b)| close(*a, b, 1e-6)), "{a:?} against {b:?}");
        assert!(close(a.iter().sum::<f64>(), 0.0, 1e-12));
        assert!(close(a[7] - a[0], 1.0 * 7.0 / 8.0, 1e-5), "{a:?}");
    }

    #[test]
    fn the_window_is_the_first_900_seconds() {
        let profile = profile(&sliding(10.0, 1000.0, 200.0));
        assert_eq!(profile.context.frames, 9000);
        assert!(close(profile.context.seconds, 900.0, 1e-12));
        assert!(profile.measures.is_ok());
    }

    #[test]
    fn a_window_under_855_seconds_is_short() {
        assert_eq!(profile(&sliding(10.0, 854.9, 200.0)).measures, Err(Exclusion::Short));
        assert!(profile(&sliding(10.0, 855.0, 200.0)).measures.is_ok());
    }

    #[test]
    fn a_window_with_under_half_its_frames_in_eight_arcs_is_gappy() {
        // Frames whose index modulo 10 is below `gaps` have no skeleton.
        let gappy = |gaps: usize| {
            let mut skeletons = sliding(10.0, 900.0, 200.0);
            let width = 2 * POINTS;
            for t in (0..skeletons.frames()).filter(|t| t % 10 < gaps) {
                skeletons.xy[t * width..(t + 1) * width].fill(f32::NAN);
            }
            profile(&skeletons)
        };
        let six = gappy(6);
        assert_eq!(six.measures, Err(Exclusion::Gappy));
        assert!(close(six.context.missing_share, 0.6, 1e-12));
        let five = gappy(5);
        assert!(close(five.context.missing_share, 0.5, 1e-12));
        assert!(close(five.context.eight_arc_share, 0.5, 1e-12));
        assert!(five.measures.is_ok(), "exactly half is enough");
    }
}
