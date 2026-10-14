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
//! between: a centred difference over [`SPEED_SPAN_SECONDS`]. Only basic arithmetic and `sqrt`
//! are used, as in `posture`.

use crate::posture::EightArcs;

/// The span a speed is taken over, centred on its frame. Chosen on one reference recording
/// against the dataset's own published midbody speed (docs/journal of 2026-10-14, *A Midbody
/// Speed*), whose definition the lab has not read.
pub const SPEED_SPAN_SECONDS: f64 = 0.5;

/// Frames on either side of a speed's own frame at a frame rate: half the span, rounded to the
/// nearest frame, and at least one.
pub fn half_span_frames(fps: f64) -> usize {
    ((SPEED_SPAN_SECONDS * fps / 2.0).round() as usize).max(1)
}

/// The midbody's speed along the body at the frame `now`, from the frames `before` and `after`,
/// `seconds` apart: positive towards the head, in the skeletons' units per second.
pub fn midbody_speed(before: &EightArcs, now: &EightArcs, after: &EightArcs, seconds: f64) -> f64 {
    let middle = crate::posture::ARCS / 2;
    let (dx, dy) = (after.ends[middle].0 - before.ends[middle].0, after.ends[middle].1 - before.ends[middle].1);
    let (hx, hy) = (now.ends[middle - 1].0 - now.ends[middle + 1].0, now.ends[middle - 1].1 - now.ends[middle + 1].1);
    (dx * hx + dy * hy) / (hx * hx + hy * hy).sqrt() / seconds
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
            Some(midbody_speed(before, frames[t].as_ref()?, after, seconds))
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
        let forward = midbody_speed(&straight(0.0, 0.0), &straight(-1.0, 0.0), &straight(-2.0, 0.0), 0.5);
        assert!((forward - 4.0).abs() < 1e-9, "2 units in 0.5 s forwards: {forward}");
        let backward = midbody_speed(&straight(0.0, 0.0), &straight(1.0, 0.0), &straight(2.0, 0.0), 0.5);
        assert!((backward + 4.0).abs() < 1e-9, "2 units in 0.5 s backwards: {backward}");
    }

    #[test]
    fn sliding_sideways_is_no_speed_along_the_body() {
        let sideways = midbody_speed(&straight(0.0, 0.0), &straight(0.0, 1.0), &straight(0.0, 2.0), 0.5);
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
}
