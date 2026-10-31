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

//! Locomotion states - forward, backward and paused - from a midbody speed and a body length per
//! frame, by the rule the OpenWorm analysis toolbox applies to make the reference recordings'
//! published `motion_modes` (`features/locomotion_features.py`, `MotionEvent` and `MotionMode`;
//! `features/events.py`, `EventFinder`; docs/journal of 2026-10-15, *The Published Locomotion
//! States, Read at Their Source*). The thresholds are fractions of the body's length and the
//! durations are seconds, so the rule applies to a body of any size at any frame rate.

/// A frame's locomotion state.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MotionMode {
    Forward,
    Backward,
    Paused,
}

/// The state at every frame, given the midbody speed (positive towards the head) and the body's
/// length per frame, in the same unit of length, and the frame rate. `None` where a frame is in
/// no state.
///
/// It agrees with the toolbox except at exact ties in the last bit of a double: numpy sums a
/// run's distance pairwise and interpolates a length as a slope times a step, where this sums in
/// order and scales the difference, so the two can differ only where a sum or a threshold falls
/// exactly on its bound. With every length undefined the toolbox raises an error, where this
/// gives no state at all.
pub fn motion_modes(speeds: &[Option<f64>], lengths: &[Option<f64>], fps: f64) -> Vec<Option<MotionMode>> {
    let mut modes = vec![None; speeds.len()];
    // In the toolbox's order, each later mode overwriting an earlier one where their states overlap, as
    // they can when bridged gaps chain across modes.
    for (mode, events) in motion_events(speeds, lengths, fps) {
        for (start, stop) in events {
            for slot in &mut modes[start..=stop] {
                *slot = Some(mode);
            }
        }
    }
    modes
}

/// Each mode's own events, as `(first frame, last frame)` pairs, in the order forward, backward,
/// paused: what the toolbox's `MotionEvent` finds for each mode before `motion_modes` merges them,
/// and what its durations and rates are computed from (docs/research/behaviour-measures.md, *How
/// the Toolbox Turns States Into Durations and Rates*). Events of two modes may overlap.
pub fn motion_events(speeds: &[Option<f64>], lengths: &[Option<f64>], fps: f64) -> [(MotionMode, Vec<(usize, usize)>); 3] {
    let lengths = interpolated(lengths);
    let fraction = |share: f64| -> Vec<Option<f64>> { lengths.iter().map(|length| length.map(|length| share * length)).collect() };
    // The speed and distance thresholds are both 5% of the length in the toolbox's options, and
    // are kept apart here so that one cannot change with the other.
    let (speed_bound, distance_bound, pause_bound) = (fraction(0.05), fraction(0.05), fraction(0.025));
    let rules: [(MotionMode, Bound, Bound, bool); 3] = [
        (MotionMode::Forward, Bound::Above(&speed_bound), Bound::None, true),
        (MotionMode::Backward, Bound::None, Bound::Below(&speed_bound), true),
        (MotionMode::Paused, Bound::Below(&pause_bound), Bound::Above(&pause_bound), false),
    ];
    rules.map(|(mode, low, high, needs_distance)| {
        let candidate = |t: usize| -> bool {
            let Some(speed) = speeds[t] else { return false };
            low.admits_from_below(speed, t) && high.admits_from_above(speed, t)
        };
        let distance = needs_distance.then_some(&distance_bound);
        (mode, states(speeds, fps, candidate, distance))
    })
}

/// One mode's events summarised as the toolbox's `EventListWithFeatures` does, over a recording of
/// `frames` frames at `fps`.
#[derive(Clone, PartialEq, Debug)]
pub struct EventStats {
    /// Each event's duration in seconds, `(last - first + 1) / fps`, those cut off by the recording's
    /// ends included.
    pub durations: Vec<f64>,
    /// The seconds between each event and the next, `(next first - last - 1) / fps`.
    pub between: Vec<f64>,
    /// Events per second of the whole recording, every frame counted, leaving out an event under way
    /// at the first frame and one still under way at the last - but only when there are at least two
    /// events, as the toolbox's `num_events_for_stats` does. `None` when there are no events, where
    /// the toolbox gives NaN.
    pub frequency: Option<f64>,
    /// The share of the recording's time spent in the events, partial ones included; `None` when
    /// there are no events.
    pub time_ratio: Option<f64>,
}

/// The toolbox's statistics of one mode's `events`, as `motion_events` gives them, over a recording
/// of `frames` frames at `fps`.
pub fn event_stats(events: &[(usize, usize)], frames: usize, fps: f64) -> EventStats {
    let durations: Vec<f64> = events.iter().map(|&(start, stop)| (stop - start + 1) as f64 / fps).collect();
    let between = events.windows(2).map(|pair| (pair[1].0 - pair[0].1 - 1) as f64 / fps).collect();
    let (Some(&(first, _)), Some(&(_, last))) = (events.first(), events.last()) else {
        return EventStats {
            durations,
            between,
            frequency: None,
            time_ratio: None,
        };
    };
    let total_time = frames as f64 / fps;
    let mut counted = events.len();
    if counted > 1 {
        counted -= usize::from(first == 0) + usize::from(last + 1 == frames);
    }
    let time_ratio = Some(durations.iter().sum::<f64>() / total_time);
    EventStats {
        durations,
        between,
        frequency: Some(counted as f64 / total_time),
        time_ratio,
    }
}

/// A per-frame bound on the speed: `Above(b)` admits a speed of at least `b`, `Below(b)` of at
/// least `-b` as a lower bound or at most `-b` as an upper one, as the toolbox's signed thresholds
/// do; an undefined bound admits nothing.
#[derive(Clone, Copy)]
enum Bound<'a> {
    None,
    Above(&'a [Option<f64>]),
    Below(&'a [Option<f64>]),
}

impl Bound<'_> {
    fn admits_from_below(self, speed: f64, t: usize) -> bool {
        match self {
            Bound::None => true,
            Bound::Above(bound) => bound[t].is_some_and(|bound| speed >= bound),
            Bound::Below(bound) => bound[t].is_some_and(|bound| speed >= -bound),
        }
    }

    fn admits_from_above(self, speed: f64, t: usize) -> bool {
        match self {
            Bound::None => true,
            Bound::Above(bound) => bound[t].is_some_and(|bound| speed <= bound),
            Bound::Below(bound) => bound[t].is_some_and(|bound| speed <= -bound),
        }
    }
}

/// The toolbox's `EventFinder` for one mode: runs of candidate frames, the first reaching back to
/// frame 0 and the last forward to the end when only undefined speeds lie beyond them; runs less
/// than a quarter of a second apart joined; runs shorter than half a second dropped; and, given a
/// distance bound, runs dropped unless the distance covered exceeds the bound's mean over the run.
fn states(speeds: &[Option<f64>], fps: f64, candidate: impl Fn(usize) -> bool, distance: Option<&Vec<Option<f64>>>) -> Vec<(usize, usize)> {
    let mut runs: Vec<(usize, usize)> = Vec::new();
    for t in (0..speeds.len()).filter(|&t| candidate(t)) {
        match runs.last_mut() {
            Some((_, stop)) if *stop + 1 == t => *stop = t,
            _ => runs.push((t, t)),
        }
    }
    let (Some(&(first, _)), Some(&(_, last))) = (runs.first(), runs.last()) else {
        return runs;
    };
    if speeds[..first].iter().all(Option::is_none) {
        runs[0].0 = 0;
    }
    if speeds[last + 1..].iter().all(Option::is_none) {
        let end = runs.len() - 1;
        runs[end].1 = speeds.len() - 1;
    }
    let mut joined: Vec<(usize, usize)> = Vec::new();
    for (start, stop) in runs {
        match joined.last_mut() {
            Some((_, previous)) if ((start - *previous - 1) as f64) < fps * 0.25 => *previous = stop,
            _ => joined.push((start, stop)),
        }
    }
    joined.retain(|&(start, stop)| ((stop - start + 1) as f64) >= fps * 0.5);
    if let Some(bound) = distance {
        joined.retain(|&(start, stop)| {
            let covered: f64 = speeds[start..=stop].iter().flatten().map(|speed| (speed / fps).abs()).sum();
            let defined: Vec<f64> = bound[start..=stop].iter().flatten().copied().collect();
            // An undefined mean, as numpy's nanmean of nothing, fails the comparison and keeps the run.
            defined.is_empty() || covered > defined.iter().sum::<f64>() / defined.len() as f64
        });
    }
    joined
}

/// Lengths with every run of undefined frames between two defined ones filled linearly, as the
/// toolbox's `interpolate_with_threshold` with no threshold; those before the first defined length
/// and after the last stay undefined.
fn interpolated(lengths: &[Option<f64>]) -> Vec<Option<f64>> {
    let mut out = lengths.to_vec();
    let known: Vec<usize> = (0..lengths.len()).filter(|&t| lengths[t].is_some()).collect();
    for pair in known.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let (ya, yb) = (lengths[a].unwrap_or_default(), lengths[b].unwrap_or_default());
        for (t, slot) in out.iter_mut().enumerate().take(b).skip(a + 1) {
            *slot = Some(ya + (yb - ya) * (t - a) as f64 / (b - a) as f64);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const FPS: f64 = 10.0;

    fn constant(speed: f64, frames: usize) -> Vec<Option<f64>> {
        vec![Some(speed); frames]
    }

    fn length(frames: usize) -> Vec<Option<f64>> {
        vec![Some(1000.0); frames]
    }

    #[test]
    fn steady_crawling_is_forward_and_backing_is_backward() {
        // 10% of a 1000-unit body a second, for two seconds.
        assert!(motion_modes(&constant(100.0, 20), &length(20), FPS).iter().all(|&m| m == Some(MotionMode::Forward)));
        assert!(motion_modes(&constant(-100.0, 20), &length(20), FPS).iter().all(|&m| m == Some(MotionMode::Backward)));
    }

    #[test]
    fn stillness_within_two_and_a_half_percent_is_paused_and_between_is_none() {
        assert!(motion_modes(&constant(25.0, 20), &length(20), FPS).iter().all(|&m| m == Some(MotionMode::Paused)));
        assert!(motion_modes(&constant(-25.0, 20), &length(20), FPS).iter().all(|&m| m == Some(MotionMode::Paused)));
        assert!(motion_modes(&constant(40.0, 20), &length(20), FPS).iter().all(|&m| m.is_none()), "3% is no state");
    }

    #[test]
    fn a_state_shorter_than_half_a_second_is_dropped() {
        // Four frames of crawling at 10 fps is 0.4 s, among frames in no state.
        let mut speeds = constant(40.0, 20);
        for speed in &mut speeds[8..12] {
            *speed = Some(200.0);
        }
        assert!(motion_modes(&speeds, &length(20), FPS).iter().all(|&m| m.is_none()));
        for speed in &mut speeds[8..13] {
            *speed = Some(200.0);
        }
        let modes = motion_modes(&speeds, &length(20), FPS);
        assert!(modes[8..13].iter().all(|&m| m == Some(MotionMode::Forward)), "five frames, 0.5 s, is enough");
    }

    #[test]
    fn a_gap_under_a_quarter_second_is_bridged() {
        // Two runs of crawling with two frames (0.2 s) of 3% between: one forward state across them.
        let mut speeds = constant(100.0, 20);
        speeds[9] = Some(30.0);
        speeds[10] = Some(30.0);
        assert!(motion_modes(&speeds, &length(20), FPS).iter().all(|&m| m == Some(MotionMode::Forward)));
        // Three frames (0.3 s) are not bridged; each side, a second long, stands alone.
        speeds[11] = Some(30.0);
        let modes = motion_modes(&speeds, &length(20), FPS);
        assert_eq!(modes[10], None);
        assert_eq!((modes[8], modes[12]), (Some(MotionMode::Forward), Some(MotionMode::Forward)));
    }

    #[test]
    fn crawling_must_cover_five_percent_of_the_length() {
        // Exactly 5% a second for 0.9 s covers 4.5% of the length: not forward. For 1.1 s, 5.5%: forward.
        let mut speeds = constant(0.0, 30);
        for speed in &mut speeds[10..19] {
            *speed = Some(50.0);
        }
        let modes = motion_modes(&speeds, &length(30), FPS);
        assert!(modes[10..19].iter().all(|&m| m.is_none()), "{modes:?}");
        for speed in &mut speeds[10..21] {
            *speed = Some(50.0);
        }
        let modes = motion_modes(&speeds, &length(30), FPS);
        assert!(modes[10..21].iter().all(|&m| m == Some(MotionMode::Forward)), "{modes:?}");
    }

    #[test]
    fn a_short_lull_is_bridged_and_a_long_one_paused() {
        // A 0.2 s lull inside crawling is bridged by the forward state and, too short, is not paused.
        let mut speeds = constant(100.0, 30);
        for speed in &mut speeds[12..14] {
            *speed = Some(0.0);
        }
        let modes = motion_modes(&speeds, &length(30), FPS);
        assert!(modes.iter().all(|&m| m == Some(MotionMode::Forward)));
        // A 0.5 s lull, with a frame of 3% on either side, is paused, the crawling on either side forward.
        let mut speeds = constant(100.0, 30);
        speeds[12] = Some(30.0);
        for speed in &mut speeds[13..18] {
            *speed = Some(0.0);
        }
        speeds[18] = Some(30.0);
        let modes = motion_modes(&speeds, &length(30), FPS);
        assert!(modes[13..18].iter().all(|&m| m == Some(MotionMode::Paused)), "{modes:?}");
        assert_eq!(
            (modes[11], modes[12], modes[18], modes[19]),
            (Some(MotionMode::Forward), None, None, Some(MotionMode::Forward))
        );
    }

    #[test]
    fn a_gap_of_exactly_a_quarter_second_is_not_bridged() {
        // At 8 fps a quarter of a second is exactly two frames, and only a shorter gap is bridged.
        let mut speeds = constant(100.0, 20);
        speeds[9] = Some(30.0);
        speeds[10] = Some(30.0);
        let modes = motion_modes(&speeds, &length(20), 8.0);
        assert_eq!((modes[9], modes[10]), (None, None), "{modes:?}");
        speeds[10] = Some(100.0);
        assert_eq!(motion_modes(&speeds, &length(20), 8.0)[9], Some(MotionMode::Forward), "one frame is bridged");
    }

    #[test]
    fn covering_exactly_five_percent_is_not_enough() {
        // Exactly 5% a second for exactly one second: 5% of the length, which must be exceeded.
        let mut speeds = constant(0.0, 30);
        for speed in &mut speeds[10..20] {
            *speed = Some(50.0);
        }
        assert!(motion_modes(&speeds, &length(30), FPS)[10..20].iter().all(|&m| m.is_none()));
    }

    #[test]
    fn where_states_overlap_paused_wins_over_backward_over_forward() {
        // Crawling for a second, two frames still, a two-frame forward blip, then still for 0.6 s:
        // forward bridges the stillness to the blip (frames 0 to 13), paused bridges the blip
        // (frames 10 to 19), and paused, written last, holds the overlap.
        let mut speeds = constant(100.0, 20);
        for t in [10, 11, 14, 15, 16, 17, 18, 19] {
            speeds[t] = Some(0.0);
        }
        let modes = motion_modes(&speeds, &length(20), FPS);
        assert!(modes[..10].iter().all(|&m| m == Some(MotionMode::Forward)), "{modes:?}");
        assert!(modes[10..].iter().all(|&m| m == Some(MotionMode::Paused)), "{modes:?}");
        // The same with backing for stillness: backward, written after forward, holds the overlap.
        for t in [10, 11, 14, 15, 16, 17, 18, 19] {
            speeds[t] = Some(-100.0);
        }
        let modes = motion_modes(&speeds, &length(20), FPS);
        assert!(modes[..10].iter().all(|&m| m == Some(MotionMode::Forward)), "{modes:?}");
        assert!(modes[10..].iter().all(|&m| m == Some(MotionMode::Backward)), "{modes:?}");
        // Backing for a second, two still frames, a two-frame backward blip, then still: paused holds the overlap.
        let mut speeds = constant(-100.0, 20);
        for t in [10, 11, 14, 15, 16, 17, 18, 19] {
            speeds[t] = Some(0.0);
        }
        let modes = motion_modes(&speeds, &length(20), FPS);
        assert!(modes[..10].iter().all(|&m| m == Some(MotionMode::Backward)), "{modes:?}");
        assert!(modes[10..].iter().all(|&m| m == Some(MotionMode::Paused)), "{modes:?}");
    }

    #[test]
    fn backing_at_exactly_five_percent_is_backward() {
        assert!(motion_modes(&constant(-50.0, 20), &length(20), FPS).iter().all(|&m| m == Some(MotionMode::Backward)));
    }

    #[test]
    fn interpolated_lengths_follow_a_ramp() {
        // The length rises from 1000 to 3000 over twenty frames, so 5% of it is 50 + 5t: a speed of 104
        // reaches it up to frame 10 and not from frame 11. A step fill or a wrong denominator moves that edge.
        let mut lengths = vec![None; 21];
        lengths[0] = Some(1000.0);
        lengths[20] = Some(3000.0);
        let modes = motion_modes(&constant(104.0, 21), &lengths, FPS);
        assert!(modes[..=10].iter().all(|&m| m == Some(MotionMode::Forward)), "{modes:?}");
        assert!(modes[11..].iter().all(|&m| m.is_none()), "{modes:?}");
    }

    #[test]
    fn lengths_before_the_first_known_one_stay_undefined() {
        // No threshold is defined before frame 5, so those frames are in no state, however fast.
        let mut lengths = length(20);
        for length in &mut lengths[..5] {
            *length = None;
        }
        let modes = motion_modes(&constant(100.0, 20), &lengths, FPS);
        assert!(modes[..5].iter().all(|&m| m.is_none()), "{modes:?}");
        assert!(modes[5..].iter().all(|&m| m == Some(MotionMode::Forward)), "{modes:?}");
    }

    #[test]
    fn the_distance_bound_is_averaged_over_defined_lengths_only() {
        // Undefined speeds and lengths at the start join the first state; ten frames at exactly 5% cover
        // exactly 5% of the length, averaged over the ten frames whose lengths are defined, so it is no state.
        let mut speeds = constant(50.0, 15);
        let mut lengths = length(15);
        for t in 0..5 {
            speeds[t] = None;
            lengths[t] = None;
        }
        assert!(motion_modes(&speeds, &lengths, FPS).iter().all(|&m| m.is_none()));
    }

    #[test]
    fn missing_speeds_at_the_ends_join_the_first_and_last_states() {
        let mut speeds = constant(100.0, 20);
        speeds[0] = None;
        speeds[1] = None;
        speeds[19] = None;
        assert!(motion_modes(&speeds, &length(20), FPS).iter().all(|&m| m == Some(MotionMode::Forward)));
    }

    #[test]
    fn missing_lengths_are_interpolated_between_known_ones() {
        // The length is known only at the ends; between, it is interpolated, so the thresholds hold.
        let mut lengths = vec![None; 20];
        lengths[0] = Some(1000.0);
        lengths[19] = Some(1000.0);
        assert!(motion_modes(&constant(100.0, 20), &lengths, FPS).iter().all(|&m| m == Some(MotionMode::Forward)));
    }

    #[test]
    fn motion_events_keep_each_mode_and_motion_modes_merge_them() {
        // Two seconds forward, then two seconds paused, at 10 fps.
        let mut speeds = constant(100.0, 20);
        speeds.extend(constant(0.0, 20));
        let events = motion_events(&speeds, &length(40), FPS);
        assert_eq!(events[0], (MotionMode::Forward, vec![(0, 19)]));
        assert_eq!(events[1], (MotionMode::Backward, vec![]));
        assert_eq!(events[2], (MotionMode::Paused, vec![(20, 39)]));
    }

    #[test]
    fn durations_count_whole_frames_and_the_gaps_between() {
        let stats = event_stats(&[(10, 19), (30, 34)], 100, FPS);
        assert_eq!(stats.durations, vec![1.0, 0.5]);
        assert_eq!(stats.between, vec![1.0]);
        // 1.5 s of 10 s; two events, neither at an edge.
        assert_eq!(stats.time_ratio, Some(0.15));
        assert_eq!(stats.frequency, Some(0.2));
    }

    #[test]
    fn events_cut_off_at_the_edges_are_left_out_of_the_frequency_only_among_two_or_more() {
        // Three events, the first from frame 0 and the last to frame 99: one is counted.
        assert_eq!(event_stats(&[(0, 9), (40, 49), (90, 99)], 100, FPS).frequency, Some(0.1));
        // A lone event touching an edge is counted, as the toolbox counts it.
        assert_eq!(event_stats(&[(0, 9)], 100, FPS).frequency, Some(0.1));
        // Two events, both partial: none counted.
        assert_eq!(event_stats(&[(0, 9), (90, 99)], 100, FPS).frequency, Some(0.0));
    }

    #[test]
    fn no_events_have_an_undefined_frequency_and_time_ratio() {
        let stats = event_stats(&[], 100, FPS);
        assert_eq!((stats.frequency, stats.time_ratio), (None, None));
        assert!(stats.durations.is_empty() && stats.between.is_empty());
    }
}
