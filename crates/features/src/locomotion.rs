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

//! Locomotion at the eight-arc resolution both bodies share (docs/benchmark/FEATURES.md, *Common
//! Units*): the midbody's speed along the body, positive towards the head, so that forward
//! crawling is positive and a reversal negative.
//!
//! The midbody is the middle end of the eight arcs, half the body's length from the head; the
//! body's direction there is the chord from the end three eighths along to the end five eighths
//! along, pointing to the head. The speed at a frame is the midbody's displacement from a frame
//! before it to a frame after it, projected on that frame's direction and divided by the time
//! between: a centred difference over [`SPEED_SPAN_SECONDS`]. [`published_choice_speeds`] is
//! the same speed with the published speed's choices instead. Only basic arithmetic and `sqrt`
//! are used, as in `posture`.

use crate::posture::EightArcs;

/// The span a speed is taken over, centred on its frame. Chosen on one reference recording
/// against the dataset's own published midbody speed (docs/journal of 2026-10-14, *A Midbody
/// Speed*), and since found to be the published speed's own (`velocity_body_diff`; docs/journal
/// of 2026-10-15, *The Published Midbody Speed, Read at Its Source*).
pub const SPEED_SPAN_SECONDS: f64 = 0.5;

/// Frames on either side of a speed's own frame at a frame rate: half the span, rounded to the
/// nearest frame, and at least one.
pub fn half_span_frames(fps: f64) -> usize {
    ((SPEED_SPAN_SECONDS * fps / 2.0).round() as usize).max(1)
}

/// The midbody's speed along the body at the frame `now`, from the frames `before` and `after`,
/// `seconds` apart: positive towards the head, in the skeletons' units per second. `None` when the
/// body has no direction at the midbody, folded so that the ends three and five eighths along meet.
pub fn midbody_speed(before: &EightArcs, now: &EightArcs, after: &EightArcs, seconds: f64) -> Option<f64> {
    let middle = crate::posture::ARCS / 2;
    let (dx, dy) = (after.ends[middle].0 - before.ends[middle].0, after.ends[middle].1 - before.ends[middle].1);
    let (hx, hy) = (now.ends[middle - 1].0 - now.ends[middle + 1].0, now.ends[middle - 1].1 - now.ends[middle + 1].1);
    let length = (hx * hx + hy * hy).sqrt();
    if length == 0.0 {
        return None;
    }
    Some((dx * hx + dy * hy) / length / seconds)
}

/// The midbody speed at every frame of a recording, given each frame's eight arcs (`None` where
/// the frame has none) and the frame rate: `None` where the frame itself or either frame half a
/// span away has no eight arcs, and at the recording's ends.
pub fn midbody_speeds(frames: &[Option<EightArcs>], fps: f64) -> Vec<Option<f64>> {
    let half = half_span_frames(fps);
    let seconds = (2 * half) as f64 / fps;
    (0..frames.len())
        .map(|t| {
            let before = frames.get(t.checked_sub(half)?)?.as_ref()?;
            let after = frames.get(t + half)?.as_ref()?;
            midbody_speed(before, frames[t].as_ref()?, after, seconds)
        })
        .collect()
}

/// Frames on either side of a frame in the published speed's window at a frame rate: a quarter
/// of a second in frames, rounded half to even as the toolbox's Python `round` does
/// (`features/velocity.py`, `get_frames_per_sample`), and at least one.
pub fn published_half_window(fps: f64) -> usize {
    (SPEED_SPAN_SECONDS * fps / 2.0).round_ties_even().max(1.0) as usize
}

/// The body's direction for the published choices: the chord from the end seven eighths along to
/// the end one eighth along, tail to head, nearest at eight arcs to the toolbox's points 41 and 9
/// of 49. `None` when those ends meet.
fn published_body_direction(arcs: &EightArcs) -> Option<(f64, f64)> {
    let (x, y) = (
        arcs.ends[1].0 - arcs.ends[crate::posture::ARCS - 1].0,
        arcs.ends[1].1 - arcs.ends[crate::posture::ARCS - 1].1,
    );
    (x != 0.0 || y != 0.0).then_some((x, y))
}

/// The midbody speed with the published speed's choices (docs/journal of 2026-10-15, *The
/// Midbody Speed With the Published Choices, at Eight Arcs*; the toolbox's `compute_speed` and
/// `h__getSpeedIndices` in `features/velocity.py`), at every frame of a recording, given each
/// frame's eight arcs (`None` where the frame has none) and the frame rate.
///
/// Each end of a frame's window lies [`published_half_window`] frames away, or, where that frame
/// has no eight arcs or no body direction, the nearest frame further out that has both, up to
/// twice as far; a frame needs no eight arcs of its own. The speed is the whole distance the
/// middle end of the eight arcs moves between the window's ends over the time between them,
/// negative when that motion lies more than 90 degrees from the body's direction at the first end.
pub fn published_choice_speeds(frames: &[Option<EightArcs>], fps: f64) -> Vec<Option<f64>> {
    let half = published_half_window(fps);
    let middle = crate::posture::ARCS / 2;
    let usable = |index: usize| {
        let arcs = frames.get(index)?.as_ref()?;
        Some((arcs.ends[middle], published_body_direction(arcs)?))
    };
    (0..frames.len())
        .map(|t| {
            let first = (half..=2 * half).find_map(|step| Some((t.checked_sub(step)?, usable(t.checked_sub(step)?)?)))?;
            let second = (half..=2 * half).find_map(|step| Some((t + step, usable(t + step)?)))?;
            let ((from, (start, (hx, hy))), (to, (end, _))) = (first, second);
            let (dx, dy) = (end.0 - start.0, end.1 - start.1);
            let speed = (dx * dx + dy * dy).sqrt() / ((to - from) as f64 / fps);
            Some(if dx * hx + dy * hy < 0.0 { -speed } else { speed })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::posture::eight_arcs;

    // A straight worm along x, head at x = offset, tail at x = offset + 48.
    fn straight(offset: f32, y: f32) -> EightArcs {
        let skeleton: Vec<f32> = (0..49).flat_map(|i| [offset + i as f32, y]).collect();
        eight_arcs(&skeleton).unwrap()
    }

    #[test]
    fn crawling_towards_the_head_is_positive_and_away_negative() {
        // The head is at the low-x end, so moving to lower x is moving forwards.
        let forward = midbody_speed(&straight(0.0, 0.0), &straight(-1.0, 0.0), &straight(-2.0, 0.0), 0.5).unwrap();
        assert!((forward - 4.0).abs() < 1e-9, "2 units in 0.5 s forwards: {forward}");
        let backward = midbody_speed(&straight(0.0, 0.0), &straight(1.0, 0.0), &straight(2.0, 0.0), 0.5).unwrap();
        assert!((backward + 4.0).abs() < 1e-9, "2 units in 0.5 s backwards: {backward}");
    }

    #[test]
    fn sliding_sideways_is_no_speed_along_the_body() {
        let sideways = midbody_speed(&straight(0.0, 0.0), &straight(0.0, 1.0), &straight(0.0, 2.0), 0.5).unwrap();
        assert!(sideways.abs() < 1e-9, "{sideways}");
    }

    #[test]
    fn the_half_span_is_a_quarter_second_in_frames() {
        assert_eq!(half_span_frames(25.575), 6);
        assert_eq!(half_span_frames(30.0), 8, "7.5 frames round to 8");
        assert_eq!(half_span_frames(1.0), 1, "never fewer than one");
    }

    #[test]
    fn speeds_need_a_frame_half_a_span_on_either_side() {
        // At 4 fps the half span is one frame; the worm moves one unit forwards a frame.
        let mut frames: Vec<Option<EightArcs>> = (0..6).map(|t| Some(straight(-(t as f32), 0.0))).collect();
        frames[4] = None;
        let speeds = midbody_speeds(&frames, 4.0);
        assert_eq!(speeds.len(), 6);
        assert_eq!(speeds[0], None, "no frame before the first");
        assert!((speeds[1].unwrap() - 4.0).abs() < 1e-9 && (speeds[2].unwrap() - 4.0).abs() < 1e-9);
        assert_eq!((speeds[3], speeds[4], speeds[5]), (None, None, None), "frame 4 missing, and the last has no after");
    }

    #[test]
    fn a_missing_frame_before_is_no_speed_in_the_middle_too() {
        let mut frames: Vec<Option<EightArcs>> = (0..8).map(|t| Some(straight(-(t as f32), 0.0))).collect();
        frames[3] = None;
        let speeds = midbody_speeds(&frames, 4.0);
        assert_eq!((speeds[2], speeds[4]), (None, None), "frame 3 is the after of 2 and the before of 4");
        assert!((speeds[5].unwrap() - 4.0).abs() < 1e-9);
    }

    #[test]
    fn a_body_with_no_direction_at_the_midbody_has_no_speed() {
        // A fold that brings the ends three and five eighths along together.
        let mut folded = straight(0.0, 0.0);
        folded.ends[5] = folded.ends[3];
        assert_eq!(midbody_speed(&straight(0.0, 0.0), &folded, &straight(-2.0, 0.0), 0.5), None);
    }

    #[test]
    fn the_published_half_window_rounds_half_to_even() {
        assert_eq!(published_half_window(25.575), 6);
        assert_eq!(published_half_window(26.0), 6, "6.5 rounds to the even 6, as Python's round does");
        assert_eq!(published_half_window(30.0), 8, "7.5 rounds to the even 8");
        assert_eq!(published_half_window(1.0), 1, "never fewer than one");
    }

    #[test]
    fn the_published_choices_sign_the_whole_distance_by_the_body() {
        // At 4 fps the half window is one frame; one unit a frame is 4 units a second.
        let forward: Vec<Option<EightArcs>> = (0..3).map(|t| Some(straight(-(t as f32), 0.0))).collect();
        assert!((published_choice_speeds(&forward, 4.0)[1].unwrap() - 4.0).abs() < 1e-9);
        let backward: Vec<Option<EightArcs>> = (0..3).map(|t| Some(straight(t as f32, 0.0))).collect();
        assert!((published_choice_speeds(&backward, 4.0)[1].unwrap() + 4.0).abs() < 1e-9);
        // Sliding sideways is the whole distance, not its projection, and at exactly 90 degrees not negative.
        let sideways: Vec<Option<EightArcs>> = (0..3).map(|t| Some(straight(0.0, t as f32))).collect();
        assert!((published_choice_speeds(&sideways, 4.0)[1].unwrap() - 4.0).abs() < 1e-9);
        // Diagonally, three forwards and four sideways a frame: five a frame, forwards.
        let diagonal: Vec<Option<EightArcs>> = (0..3).map(|t| Some(straight(-3.0 * t as f32, 4.0 * t as f32))).collect();
        assert!((published_choice_speeds(&diagonal, 4.0)[1].unwrap() - 20.0).abs() < 1e-9);
    }

    #[test]
    fn the_sign_comes_from_the_body_at_the_windows_first_end() {
        // The worm moves to lower x throughout; only the first end's body points its head that way.
        let mut frames: Vec<Option<EightArcs>> = (0..3).map(|t| Some(straight(-(t as f32), 0.0))).collect();
        let turned = |offset: f32| {
            let skeleton: Vec<f32> = (0..49).flat_map(|i| [offset - i as f32, 0.0]).collect();
            eight_arcs(&skeleton).unwrap()
        };
        frames[1] = Some(turned(47.0));
        frames[2] = Some(turned(46.0));
        assert!(published_choice_speeds(&frames, 4.0)[1].unwrap() > 0.0, "the first end's body decides");
        frames[0] = Some(turned(48.0));
        frames[1] = Some(straight(-1.0, 0.0));
        assert!(published_choice_speeds(&frames, 4.0)[1].unwrap() < 0.0, "not the middle frame's");
    }

    #[test]
    fn a_missing_end_moves_outwards_up_to_twice_as_far() {
        let mut frames: Vec<Option<EightArcs>> = (0..9).map(|t| Some(straight(-(t as f32), 0.0))).collect();
        frames[3] = None;
        let speeds = published_choice_speeds(&frames, 4.0);
        assert!((speeds[3].unwrap() - 4.0).abs() < 1e-9, "a frame with no eight arcs of its own still has a speed");
        assert!(
            (speeds[4].unwrap() - 4.0).abs() < 1e-9,
            "the first end moves from frame 3 to frame 2: 3 units in 0.75 s"
        );
        assert!((speeds[2].unwrap() - 4.0).abs() < 1e-9, "the second end moves from frame 3 to frame 4");
        assert_eq!(speeds[0], None, "no frame before the first, at any distance");
        frames[1] = None;
        frames[4] = None;
        assert_eq!(
            published_choice_speeds(&frames, 4.0)[2],
            None,
            "frames 3 and 4 missing: no second end within twice the half window"
        );
    }

    #[test]
    fn an_end_with_no_body_direction_is_no_end() {
        let mut frames: Vec<Option<EightArcs>> = (0..5).map(|t| Some(straight(-(t as f32), 0.0))).collect();
        let mut folded = straight(-1.0, 0.0);
        folded.ends[7] = folded.ends[1];
        frames[1] = Some(folded);
        let speeds = published_choice_speeds(&frames, 4.0);
        assert!((speeds[2].unwrap() - 4.0).abs() < 1e-9, "the first end moves past the fold to frame 0");
    }
}
