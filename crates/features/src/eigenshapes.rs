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

//! The eigen-shapes of the eight-arc posture (docs/benchmark/FEATURES.md, *Common Units*): the
//! principal components of the eight mean-removed arc angles
//! ([`crate::profile::mean_removed_angles`]), as Stephens et al. 2008 fit their eigenworms on 100
//! angles ([doi:10.1371/journal.pcbi.1000028](https://doi.org/10.1371/journal.pcbi.1000028)).
//!
//! Each recording is summarised by its [`Moments`]: how many frames it has, and the sums of the
//! angles and of their products. The covariance is then built from those, in one of two
//! [`Weighting`]s: every recording counting equally, whatever its frame count, or every frame
//! counting equally. Removing each frame's mean angle makes the eight angles sum to zero, so the
//! covariance has rank at most seven and its eighth eigenvalue is zero up to rounding.
//!
//! The eigenvectors come from the cyclic Jacobi method on the 8 × 8 matrix, in a fixed order of
//! rotations, so one input gives one answer to the bit on one machine. An eigenvector's sign is
//! arbitrary; it is fixed here by making its largest component positive (the first, if two are
//! equally large).

use crate::posture::ARCS;

/// One recording's frame count and the sums of its angles and of their pairwise products.
#[derive(Clone, PartialEq, Debug)]
pub struct Moments {
    /// The number of frames added.
    pub frames: usize,
    /// The sum of each angle over the frames.
    pub sum: [f64; ARCS],
    /// The sum over the frames of each product of two angles.
    pub products: [[f64; ARCS]; ARCS],
}

impl Default for Moments {
    fn default() -> Self {
        Self {
            frames: 0,
            sum: [0.0; ARCS],
            products: [[0.0; ARCS]; ARCS],
        }
    }
}

impl Moments {
    /// Adds one frame's eight angles.
    pub fn add(&mut self, angles: &[f64; ARCS]) {
        self.frames += 1;
        for i in 0..ARCS {
            self.sum[i] += angles[i];
            for j in 0..ARCS {
                self.products[i][j] += angles[i] * angles[j];
            }
        }
    }
}

/// How the recordings' frames are weighted in the covariance.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Weighting {
    /// Each recording counts equally: each of its frames weighs one over its frame count.
    Recordings,
    /// Each frame counts equally, so a recording with more frames counts for more.
    Frames,
}

/// The weighted covariance of the angles over the recordings, about their weighted mean, divided
/// by the total weight (not by one less). `None` when no recording has a frame.
pub fn covariance(recordings: &[&Moments], weighting: Weighting) -> Option<[[f64; ARCS]; ARCS]> {
    let used: Vec<&&Moments> = recordings.iter().filter(|moments| moments.frames > 0).collect();
    if used.is_empty() {
        return None;
    }
    let weight = |moments: &Moments| match weighting {
        Weighting::Recordings => 1.0 / moments.frames as f64,
        Weighting::Frames => 1.0,
    };
    let total: f64 = used.iter().map(|moments| weight(moments) * moments.frames as f64).sum();
    let mut mean = [0.0; ARCS];
    let mut second = [[0.0; ARCS]; ARCS];
    for moments in &used {
        let w = weight(moments) / total;
        for (i, row) in second.iter_mut().enumerate() {
            mean[i] += w * moments.sum[i];
            for (cell, product) in row.iter_mut().zip(&moments.products[i]) {
                *cell += w * product;
            }
        }
    }
    Some(std::array::from_fn(|i| std::array::from_fn(|j| second[i][j] - mean[i] * mean[j])))
}

/// The eigen-shapes of a covariance: its eigenvalues, largest first, and a unit eigenvector for
/// each, `shapes[k]` belonging to `values[k]`.
#[derive(Clone, PartialEq, Debug)]
pub struct EigenShapes {
    pub values: [f64; ARCS],
    pub shapes: [[f64; ARCS]; ARCS],
}

impl EigenShapes {
    /// The share of the total variance (the trace) each eigen-shape carries, a negative rounding
    /// residue counted as zero.
    pub fn shares(&self) -> [f64; ARCS] {
        let trace: f64 = self.values.iter().map(|value| value.max(0.0)).sum();
        self.values.map(|value| value.max(0.0) / trace)
    }

    /// The fewest eigen-shapes, largest first, whose shares add up to at least `share`.
    pub fn carrying(&self, share: f64) -> usize {
        let mut cumulative = 0.0;
        for (k, part) in self.shares().iter().enumerate() {
            cumulative += part;
            if cumulative >= share {
                return k + 1;
            }
        }
        ARCS
    }
}

/// The eigen-shapes of a symmetric matrix, by cyclic Jacobi rotations until the off-diagonal part
/// is negligible against the whole matrix (Jacobi's method in its cyclic form: each sweep
/// zeroes every off-diagonal pair once, in a fixed order).
pub fn eigen_shapes(matrix: &[[f64; ARCS]; ARCS]) -> EigenShapes {
    let mut a = *matrix;
    let mut v: [[f64; ARCS]; ARCS] = std::array::from_fn(|i| std::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
    let off = |a: &[[f64; ARCS]; ARCS]| {
        (0..ARCS)
            .flat_map(|i| (0..ARCS).filter(move |&j| j != i).map(move |j| (i, j)))
            .map(|(i, j)| a[i][j] * a[i][j])
            .sum::<f64>()
    };
    let scale: f64 = (0..ARCS).flat_map(|i| (0..ARCS).map(move |j| (i, j))).map(|(i, j)| a[i][j] * a[i][j]).sum();
    for _sweep in 0..64 {
        if off(&a) <= scale * 1e-30 {
            break;
        }
        for p in 0..ARCS {
            for q in p + 1..ARCS {
                if a[p][q] == 0.0 {
                    continue;
                }
                // The rotation that zeroes a[p][q]: tan of its angle is the smaller root of
                // t^2 + 2 theta t - 1 = 0.
                let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
                // At theta = 0, signum gives 1: the rotation by pi / 4 that equal diagonals need.
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                for row in &mut a {
                    let (akp, akq) = (row[p], row[q]);
                    row[p] = c * akp - s * akq;
                    row[q] = s * akp + c * akq;
                }
                let (row_p, row_q) = (a[p], a[q]);
                a[p] = std::array::from_fn(|k| c * row_p[k] - s * row_q[k]);
                a[q] = std::array::from_fn(|k| s * row_p[k] + c * row_q[k]);
                for row in &mut v {
                    let (vp, vq) = (row[p], row[q]);
                    row[p] = c * vp - s * vq;
                    row[q] = s * vp + c * vq;
                }
            }
        }
    }
    let mut order: [usize; ARCS] = std::array::from_fn(|k| k);
    order.sort_by(|&x, &y| a[y][y].total_cmp(&a[x][x]).then(x.cmp(&y)));
    let values = order.map(|k| a[k][k]);
    let shapes = order.map(|k| {
        let mut shape: [f64; ARCS] = std::array::from_fn(|i| v[i][k]);
        let largest = (0..ARCS).fold(0, |best, i| if shape[i].abs() > shape[best].abs() { i } else { best });
        if shape[largest] < 0.0 {
            shape = shape.map(|x| -x);
        }
        shape
    });
    EigenShapes { values, shapes }
}

/// How many of block 1's eigen-shapes are measures: the fewest that carry 95% of the variance
/// (docs/journal of 2026-11-03, *The Reference Statistics of Block 1*).
pub const REFERENCE_COMPONENTS: usize = 4;

/// The frozen basis both bodies are projected on: block 1's first [`REFERENCE_COMPONENTS`]
/// eigen-shapes, head first, PLACEHOLDER.
pub const REFERENCE_SHAPES: [[f64; ARCS]; REFERENCE_COMPONENTS] = [[0.0; ARCS]; REFERENCE_COMPONENTS];

/// One frame's amplitudes on [`REFERENCE_SHAPES`]: its mean-removed angles projected on each shape.
pub fn amplitudes(angles: &[f64; ARCS]) -> [f64; REFERENCE_COMPONENTS] {
    REFERENCE_SHAPES.map(|shape| shape.iter().zip(angles).map(|(e, a)| e * a).sum())
}

/// The first [`REFERENCE_COMPONENTS`] shapes as the Rust source of [`REFERENCE_SHAPES`]'s value,
/// each loading in Rust's shortest form that reads back as the same `f64`.
pub fn basis_source(shapes: &EigenShapes) -> String {
    let rows: Vec<String> = shapes.shapes[..REFERENCE_COMPONENTS]
        .iter()
        .map(|shape| format!("    [{}],", shape.iter().map(|x| format!("{x:?}")).collect::<Vec<_>>().join(", ")))
        .collect();
    format!("[\n{}\n]", rows.join("\n"))
}

/// The absolute cosine between two eigen-shapes: 1 when they are the same up to sign.
pub fn alignment(a: &[f64; ARCS], b: &[f64; ARCS]) -> f64 {
    let dot: f64 = a.iter().zip(b).map(|(x, y)| x * y).sum();
    let norm = |v: &[f64; ARCS]| v.iter().map(|x| x * x).sum::<f64>().sqrt();
    (dot / (norm(a) * norm(b))).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, tolerance: f64) -> bool {
        (a - b).abs() <= tolerance
    }

    /// Two orthonormal shapes that sum to zero along the body, as mean-removed angles do.
    fn two_shapes() -> ([f64; ARCS], [f64; ARCS]) {
        let first: [f64; ARCS] = std::array::from_fn(|k| (k as f64 - 3.5) / 42.0_f64.sqrt());
        let second: [f64; ARCS] = [1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0].map(|x: f64| x / 8.0_f64.sqrt());
        (first, second)
    }

    #[test]
    fn two_known_shapes_are_found_with_their_variances() {
        // Frames a * first + b * second, a in {-3, 3} and b in {-1, 1}, every combination equally
        // often: variances 9 and 1 along the two shapes, none elsewhere.
        let (first, second) = two_shapes();
        let mut moments = Moments::default();
        for a in [-3.0, 3.0] {
            for b in [-1.0, 1.0] {
                moments.add(&std::array::from_fn(|k| a * first[k] + b * second[k]));
            }
        }
        let shapes = eigen_shapes(&covariance(&[&moments], Weighting::Recordings).unwrap());
        assert!(close(shapes.values[0], 9.0, 1e-12), "{:?}", shapes.values);
        assert!(close(shapes.values[1], 1.0, 1e-12), "{:?}", shapes.values);
        assert!(shapes.values[2..].iter().all(|value| value.abs() < 1e-12), "{:?}", shapes.values);
        assert!(close(alignment(&shapes.shapes[0], &first), 1.0, 1e-12));
        assert!(close(alignment(&shapes.shapes[1], &second), 1.0, 1e-12));
        assert!(close(shapes.shares()[0], 0.9, 1e-12));
        assert_eq!(shapes.carrying(0.95), 2);
        assert_eq!(shapes.carrying(0.9), 1);
    }

    #[test]
    fn an_eigen_shape_has_unit_length_and_its_largest_component_positive() {
        let (first, second) = two_shapes();
        let mut moments = Moments::default();
        for a in [-2.0, 2.0] {
            for b in [-1.0, 1.0] {
                moments.add(&std::array::from_fn(|k| a * first[k] + b * second[k]));
            }
        }
        let shapes = eigen_shapes(&covariance(&[&moments], Weighting::Frames).unwrap());
        for shape in &shapes.shapes[..2] {
            assert!(close(shape.iter().map(|x| x * x).sum::<f64>(), 1.0, 1e-12));
            let largest = shape.iter().fold(0.0_f64, |m, x| if x.abs() > m.abs() { *x } else { m });
            assert!(largest > 0.0, "{shape:?}");
        }
    }

    #[test]
    fn the_covariance_is_about_the_mean() {
        // A constant offset in every frame moves the mean and leaves no variance.
        let (first, _) = two_shapes();
        let mut moments = Moments::default();
        for _ in 0..5 {
            moments.add(&first.map(|x| 2.0 * x));
        }
        let cov = covariance(&[&moments], Weighting::Frames).unwrap();
        assert!(cov.iter().flatten().all(|x| x.abs() < 1e-12), "{cov:?}");
    }

    #[test]
    fn recordings_weigh_equally_or_by_their_frames() {
        // One recording of 1 frame along +first and 1 along -first (variance 1 along first); one
        // of 2 frames along +second, 2 along -second, scaled by 2 (variance 4 along second).
        // Equal recordings: 1/2 and 4/2. Frames pooled: 2/6 and 16/6.
        let (first, second) = two_shapes();
        let mut short = Moments::default();
        short.add(&first);
        short.add(&first.map(|x| -x));
        let mut long = Moments::default();
        for sign in [1.0, 1.0, -1.0, -1.0] {
            long.add(&second.map(|x| 2.0 * sign * x));
        }
        let by_recording = eigen_shapes(&covariance(&[&short, &long], Weighting::Recordings).unwrap());
        assert!(
            close(by_recording.values[0], 2.0, 1e-12) && close(by_recording.values[1], 0.5, 1e-12),
            "{:?}",
            by_recording.values
        );
        let by_frame = eigen_shapes(&covariance(&[&short, &long], Weighting::Frames).unwrap());
        assert!(
            close(by_frame.values[0], 16.0 / 6.0, 1e-12) && close(by_frame.values[1], 2.0 / 6.0, 1e-12),
            "{:?}",
            by_frame.values
        );
    }

    #[test]
    fn no_frames_give_no_covariance() {
        assert_eq!(covariance(&[&Moments::default()], Weighting::Recordings), None);
    }
}
