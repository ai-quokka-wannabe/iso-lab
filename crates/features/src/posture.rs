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

//! Posture at the resolution both bodies share (docs/benchmark/FEATURES.md, *Common Units*): a
//! skeleton resampled into eight arcs of equal length, as the Grid's worm is a chain of at most
//! eight rigid segments. Each arc is represented by its chord, which is what a rigid segment
//! reports as its direction, and the bend between two neighbouring chords is the joint the
//! Grid's servo would have to make.
//!
//! Only IEEE basic arithmetic and `sqrt` are used, never a transcendental function, so a result
//! is the same to the bit on any machine: a joint is compared with an angle through the cosine of
//! that angle, written down as a constant, rather than by computing the joint's angle.

/// Arcs the skeleton is resampled into: the Grid's longest chain (`SEGMENTS_MAX` in link's
/// `src/protocol.rs`).
pub const ARCS: usize = 8;

/// The Grid's bend limit for the worm class, 0.9 rad per joint (`roster.cpp` in tron-grid-lite;
/// docs/benchmark/FEATURES.md, *The Two Bodies*). The Grid stores it as the `f32` 0.9, about 2.4e-8 below this, and the
/// world's ceiling of pi/2 as an `f32` slightly above pi/2; differences that small cannot move a share reported to five
/// decimals.
pub const GRID_JOINT_LIMIT_RAD: f64 = 0.9;

/// cos(0.9), to the nearest double: 0.62160996827066445648... (its Taylor series, summed to 40
/// digits). A joint bends beyond 0.9 rad exactly when the cosine of its angle is below this.
pub const COS_GRID_JOINT_LIMIT: f64 = 0.621_609_968_270_664_5;

/// One skeleton at the eight-arc resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct EightArcs {
    /// The skeleton's arc length, in the skeleton's units (micrometres for the reference worms).
    pub length: f64,
    /// The ends of the arcs, head first: the head's tip, the seven joints, and the tail's tip.
    pub ends: [(f64, f64); ARCS + 1],
    /// The chord of each arc, head first, as (dx, dy).
    pub chords: [(f64, f64); ARCS],
}

impl EightArcs {
    /// The cosine of each of the seven joints: of the angle between neighbouring chords, so 1 for
    /// a straight joint and -1 for one folded back on itself.
    pub fn joint_cosines(&self) -> [f64; ARCS - 1] {
        std::array::from_fn(|joint| {
            let (ax, ay) = self.chords[joint];
            let (bx, by) = self.chords[joint + 1];
            ((ax * bx + ay * by) / ((ax * ax + ay * ay).sqrt() * (bx * bx + by * by).sqrt())).clamp(-1.0, 1.0)
        })
    }

    /// Whether any joint bends further than the angle whose cosine is `cos_limit`.
    pub fn bends_beyond(&self, cos_limit: f64) -> bool {
        self.joint_cosines().iter().any(|&cosine| cosine < cos_limit)
    }
}

/// Resamples a skeleton, given as (x, y) pairs head first, into eight arcs of equal length.
/// `None` when it has fewer than two points, a non-finite coordinate, or a chord of zero length
/// (a skeleton folded so that an arc's ends coincide has no direction there).
pub fn eight_arcs(skeleton: &[f32]) -> Option<EightArcs> {
    if skeleton.len() < 4 || !skeleton.len().is_multiple_of(2) || !skeleton.iter().all(|value| value.is_finite()) {
        return None;
    }
    let points: Vec<(f64, f64)> = skeleton.as_chunks::<2>().0.iter().map(|&[x, y]| (f64::from(x), f64::from(y))).collect();
    // The arc length at each point, from the head.
    let mut along = Vec::with_capacity(points.len());
    along.push(0.0);
    for pair in points.windows(2) {
        let (dx, dy) = (pair[1].0 - pair[0].0, pair[1].1 - pair[0].1);
        along.push(along[along.len() - 1] + (dx * dx + dy * dy).sqrt());
    }
    let length = along[along.len() - 1];
    if length <= 0.0 {
        return None;
    }
    // The point at arc length `s`, by linear interpolation along the polyline.
    let at = |s: f64| -> (f64, f64) {
        let edge = along.partition_point(|&a| a <= s).clamp(1, points.len() - 1);
        let (a0, a1) = (along[edge - 1], along[edge]);
        let t = if a1 > a0 { ((s - a0) / (a1 - a0)).clamp(0.0, 1.0) } else { 0.0 };
        let (p0, p1) = (points[edge - 1], points[edge]);
        (p0.0 + t * (p1.0 - p0.0), p0.1 + t * (p1.1 - p0.1))
    };
    let ends: [(f64, f64); ARCS + 1] = std::array::from_fn(|k| {
        if k == ARCS {
            points[points.len() - 1]
        } else {
            at(length * k as f64 / ARCS as f64)
        }
    });
    let chords: [(f64, f64); ARCS] = std::array::from_fn(|k| (ends[k + 1].0 - ends[k].0, ends[k + 1].1 - ends[k].1));
    if chords.iter().any(|&(dx, dy)| dx * dx + dy * dy == 0.0) {
        return None;
    }
    Some(EightArcs { length, ends, chords })
}

/// A skeleton, given as (x, y) pairs, smoothed along its length: each point replaced by the mean of
/// itself and the `half_width` points on either side. Near the ends the window shrinks to what fits
/// symmetrically, so the end points stay where they are and the window stays centred. The mean is
/// taken in `f64` and stored back as `f32`, the skeletons' own precision. An input of odd length is
/// not a list of points and comes back unchanged, for `eight_arcs` to refuse.
pub fn smoothed_along(skeleton: &[f32], half_width: usize) -> Vec<f32> {
    if !skeleton.len().is_multiple_of(2) {
        return skeleton.to_vec();
    }
    let points = skeleton.as_chunks::<2>().0;
    let last = points.len().saturating_sub(1);
    points
        .iter()
        .enumerate()
        .flat_map(|(index, _)| {
            let reach = half_width.min(index).min(last - index);
            let window = &points[index - reach..=index + reach];
            let (x, y) = window.iter().fold((0.0, 0.0), |(x, y), &[px, py]| (x + f64::from(px), y + f64::from(py)));
            let count = window.len() as f64;
            [(x / count) as f32, (y / count) as f32]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(points: usize) -> Vec<f32> {
        (0..points).flat_map(|i| [i as f32, 0.0]).collect()
    }

    #[test]
    fn a_straight_skeleton_has_straight_joints_and_its_length() {
        let arcs = eight_arcs(&line(49)).unwrap();
        assert_eq!(arcs.length, 48.0);
        for (dx, dy) in arcs.chords {
            assert!((dx - 6.0).abs() < 1e-12 && dy == 0.0, "each chord is an eighth: ({dx}, {dy})");
        }
        assert!(arcs.joint_cosines().iter().all(|&c| c == 1.0));
        assert!(!arcs.bends_beyond(COS_GRID_JOINT_LIMIT));
    }

    // Points unevenly spaced along a straight line must still give eight equal chords: the
    // resampling is by arc length, not by point index.
    #[test]
    fn resamples_by_arc_length_not_by_point() {
        let xs = [0.0_f32, 0.5, 1.0, 10.0, 11.0, 30.0, 31.0, 32.0];
        let skeleton: Vec<f32> = xs.iter().flat_map(|&x| [x, 0.0]).collect();
        let arcs = eight_arcs(&skeleton).unwrap();
        for (dx, _) in arcs.chords {
            assert!((dx - 4.0).abs() < 1e-12, "chord {dx}, not 4");
        }
    }

    // A skeleton with one sharp corner in the middle: the joint there bends by the corner's
    // angle, and the test of a limit decides on which side of it the corner lies.
    fn corner(radians: f64) -> Vec<f32> {
        let (c, s) = (radians.cos() as f32, radians.sin() as f32);
        let mut skeleton: Vec<f32> = (0..=24).flat_map(|i| [i as f32 - 24.0, 0.0]).collect();
        skeleton.extend((1..=24).flat_map(|i| [i as f32 * c, i as f32 * s]));
        skeleton
    }

    #[test]
    fn a_corner_bends_its_middle_joint_by_its_angle() {
        for (radians, beyond) in [(0.85, false), (0.95, true), (2.0, true)] {
            let arcs = eight_arcs(&corner(radians)).unwrap();
            let cosines = arcs.joint_cosines();
            assert!((cosines[3] - radians.cos()).abs() < 1e-6, "joint 4 of a {radians} rad corner: {}", cosines[3]);
            assert!(cosines.iter().enumerate().all(|(joint, &c)| joint == 3 || (c - 1.0).abs() < 1e-12));
            assert_eq!(arcs.bends_beyond(COS_GRID_JOINT_LIMIT), beyond, "{radians} rad against the 0.9 rad limit");
        }
    }

    #[test]
    fn the_limit_constant_is_the_nearest_double_to_the_cosine() {
        // Rust parses a decimal to the nearest double, so this pins the constant to cos(0.9) to
        // the last bit; the platform's own cos(0.9) is one unit in the last place away from it
        // on the machine this was written on, which is why the module never calls it.
        assert_eq!(COS_GRID_JOINT_LIMIT, "0.6216099682706644564847161514071335087220".parse::<f64>().unwrap());
        assert!((COS_GRID_JOINT_LIMIT - GRID_JOINT_LIMIT_RAD.cos()).abs() <= f64::EPSILON);
    }

    #[test]
    fn smoothing_a_zigzag_flattens_its_middle_and_keeps_its_ends() {
        // y alternates 1, -1, 1, ...: a three-point mean leaves a third, with the neighbours' sign,
        // as two of the three points are neighbours; the end points have no symmetric window and stay.
        let zigzag: Vec<f32> = (0..9).flat_map(|i| [i as f32, if i % 2 == 0 { 1.0 } else { -1.0 }]).collect();
        let smooth = smoothed_along(&zigzag, 1);
        assert_eq!(smooth.len(), zigzag.len());
        assert_eq!((smooth[0], smooth[1], smooth[16], smooth[17]), (0.0, 1.0, 8.0, 1.0), "the ends stay");
        for i in 1..8 {
            let expected = if i % 2 == 0 { -1.0 / 3.0 } else { 1.0 / 3.0 };
            assert!((smooth[2 * i] - i as f32).abs() < 1e-6, "x of point {i}: {}", smooth[2 * i]);
            assert!((smooth[2 * i + 1] - expected).abs() < 1e-6, "y of point {i}: {}", smooth[2 * i + 1]);
        }
    }

    #[test]
    fn the_window_shrinks_symmetrically_at_the_ends() {
        // On a parabola y = x^2 a centred mean of half-width h raises y by h(h+1)/3; the second
        // point can only take h = 1 however wide the window asked for.
        let parabola: Vec<f32> = (0..9).flat_map(|i| [i as f32, (i * i) as f32]).collect();
        let smooth = smoothed_along(&parabola, 3);
        assert!((smooth[3] - (1.0 + 2.0 / 3.0)).abs() < 1e-6, "point 1, h = 1: {}", smooth[3]);
        assert!((smooth[9] - 20.0).abs() < 1e-5, "point 4, h = 3: {}", smooth[9]);
        assert_eq!(smoothed_along(&parabola, 0), parabola, "half-width 0 changes nothing");
    }

    #[test]
    fn an_odd_length_is_left_for_eight_arcs_to_refuse() {
        // A trailing lone value is not a point; dropping it would turn a malformed frame into a
        // well-formed one, so the input comes back unchanged and `eight_arcs` refuses it as before.
        let odd = [0.0_f32, 0.0, 1.0, 1.0, 2.0];
        assert_eq!(smoothed_along(&odd, 1), odd);
        assert_eq!(eight_arcs(&smoothed_along(&odd, 1)), None);
    }

    #[test]
    fn refuses_what_has_no_eight_arcs() {
        assert_eq!(eight_arcs(&[0.0, 0.0]), None, "one point");
        assert_eq!(eight_arcs(&[0.0, 0.0, 1.0]), None, "an odd count");
        assert_eq!(eight_arcs(&[0.0, 0.0, f32::NAN, 1.0]), None, "a NaN");
        assert_eq!(eight_arcs(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0]), None, "no length");
    }
}
