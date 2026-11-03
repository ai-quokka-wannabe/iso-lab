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

//! A recording's missing frames by cause (STATE.md item 2 (d3)), as far as a features file can
//! tell them apart (docs/journal of 2026-11-03, *The Gaps by Cause: the Plan, Written Before the
//! Numbers*). Tierpsy builds a file's rows from the first to the last video frame with a good
//! skeleton; within them, a row is one of the [`Row`] kinds. Frames of the video before the first
//! row are a fourth kind of gap, which only the [`FrameIndex`]'s first frame reveals.
//!
//! Beside the shares, [`Edges`] finds the frames on either side of a run of one kind of gap and
//! the frames far from any gap, so that a measure can be compared between the two.

use crate::posture::eight_arcs;
use crate::{FrameIndex, Skeletons};

/// What one row of a features file holds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Row {
    /// A skeleton that resamples into eight arcs.
    EightArcs,
    /// A skeleton that does not give eight arcs.
    SkeletonOnly,
    /// A good skeleton blanked afterwards: a stage movement (a skeleton id, no coordinates).
    Stage,
    /// No good skeleton: a dropped frame, failed extraction or a rejected skeleton (no skeleton id).
    NoGoodSkeleton,
}

impl Row {
    /// Whether the row has no skeleton.
    pub fn missing(self) -> bool {
        matches!(self, Self::Stage | Self::NoGoodSkeleton)
    }
}

/// Why the skeletons and their index disagree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RowCountMismatch {
    pub skeletons: usize,
    pub index: usize,
}

/// Each of the first `rows` rows' kind.
pub fn classify(skeletons: &Skeletons, index: &FrameIndex, rows: usize) -> Result<Vec<Row>, RowCountMismatch> {
    if skeletons.frames() != index.skeleton_ids.len() {
        return Err(RowCountMismatch {
            skeletons: skeletons.frames(),
            index: index.skeleton_ids.len(),
        });
    }
    Ok((0..rows.min(skeletons.frames()))
        .map(|row| match (skeletons.frame(row), index.skeleton_ids[row]) {
            (Some(frame), _) if eight_arcs(frame).is_some() => Row::EightArcs,
            (Some(_), _) => Row::SkeletonOnly,
            (None, Some(_)) => Row::Stage,
            (None, None) => Row::NoGoodSkeleton,
        })
        .collect())
}

/// The runs of consecutive rows of `kind`, as (first row, length).
pub fn runs(rows: &[Row], kind: Row) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut start = None;
    for (index, &row) in rows.iter().chain(std::iter::once(&Row::EightArcs)).enumerate() {
        match (row == kind && index < rows.len(), start) {
            (true, None) => start = Some(index),
            (false, Some(first)) => {
                runs.push((first, index - first));
                start = None;
            }
            _ => {}
        }
    }
    runs
}

/// The frames beside one kind of gap and the frames far from every gap.
#[derive(Clone, PartialEq, Debug)]
pub struct Edges {
    /// Rows with a skeleton within `border` rows before a run of the kind starts or after it ends.
    pub border: Vec<bool>,
    /// Rows with a skeleton at least `far` rows from every missing row; when the file starts after
    /// the video does, the row before the first counts as missing. The file's end and the cut at
    /// the window's end do not count as gaps.
    pub far: Vec<bool>,
}

/// The [`Edges`] of runs of `kind` among `rows`, `border` and `far` counted in rows.
pub fn edges(rows: &[Row], kind: Row, border: usize, far: usize, starts_late: bool) -> Edges {
    let n = rows.len();
    let mut near_kind = vec![false; n];
    for (first, length) in runs(rows, kind) {
        near_kind[first.saturating_sub(border)..first].fill(true);
        near_kind[first + length..(first + length + border).min(n)].fill(true);
    }
    // Distance to the nearest missing row, by a pass each way.
    let mut distance = vec![usize::MAX; n];
    let mut last: Option<usize> = starts_late.then_some(0);
    for row in 0..n {
        if rows[row].missing() {
            last = Some(0);
        } else if let Some(gap) = last.as_mut() {
            *gap += 1;
        }
        distance[row] = if rows[row].missing() { 0 } else { last.unwrap_or(usize::MAX) };
    }
    let mut next: Option<usize> = None;
    for row in (0..n).rev() {
        if rows[row].missing() {
            next = Some(0);
        } else if let Some(gap) = next.as_mut() {
            *gap += 1;
        }
        distance[row] = distance[row].min(next.unwrap_or(usize::MAX));
    }
    let has_skeleton = |row: usize| !rows[row].missing();
    Edges {
        border: (0..n).map(|row| near_kind[row] && has_skeleton(row)).collect(),
        far: (0..n).map(|row| has_skeleton(row) && distance[row] >= far).collect(),
    }
}

/// The mean of `values` over the rows `selected` where a value exists; `None` when there is none.
pub fn mean_over(values: &[Option<f64>], selected: &[bool]) -> Option<f64> {
    let chosen: Vec<f64> = values.iter().zip(selected).filter(|&(_, &keep)| keep).filter_map(|(value, _)| *value).collect();
    (!chosen.is_empty()).then(|| chosen.iter().sum::<f64>() / chosen.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Row::{EightArcs as A, NoGoodSkeleton as N, SkeletonOnly as O, Stage as S};

    #[test]
    fn rows_are_told_apart_by_skeleton_and_id() {
        let straight: Vec<f32> = (0..49).flat_map(|i| [i as f32 * 20.0, 0.0]).collect();
        let mut doubled = straight.clone();
        // A skeleton that folds back on itself onto one point cannot be cut into eight arcs.
        doubled.iter_mut().for_each(|value| *value = 0.0);
        let nan = vec![f32::NAN; 98];
        let skeletons = Skeletons {
            fps: 10.0,
            points: 49,
            xy: [straight.clone(), doubled, nan.clone(), nan].concat(),
        };
        let index = FrameIndex {
            first_frame: 0,
            skeleton_ids: vec![Some(0), Some(1), Some(2), None],
        };
        assert_eq!(classify(&skeletons, &index, 4).unwrap(), vec![A, O, S, N]);
        assert_eq!(classify(&skeletons, &index, 2).unwrap(), vec![A, O]);
        let short = FrameIndex {
            first_frame: 0,
            skeleton_ids: vec![None],
        };
        assert_eq!(classify(&skeletons, &short, 4), Err(RowCountMismatch { skeletons: 4, index: 1 }));
    }

    #[test]
    fn runs_are_found_at_both_ends() {
        let rows = [N, N, A, S, N, A, N];
        assert_eq!(runs(&rows, N), vec![(0, 2), (4, 1), (6, 1)]);
        assert_eq!(runs(&rows, S), vec![(3, 1)]);
        assert_eq!(runs(&[A, A], N), vec![]);
    }

    #[test]
    fn border_and_far_rows() {
        // Rows 0-9: a no-good-skeleton run at 4-5, a stage row at 9.
        let rows = [A, A, A, A, N, N, A, A, A, S];
        let e = edges(&rows, N, 2, 3, false);
        assert_eq!(e.border, vec![false, false, true, true, false, false, true, true, false, false]);
        // Distances to the nearest missing row: 4 3 2 1 0 0 1 2 1 0; far needs at least 3.
        assert_eq!(e.far, vec![true, true, false, false, false, false, false, false, false, false]);
        // A late start puts a gap before row 0: distances become 1 2 2 1 ...
        let late = edges(&rows, N, 2, 3, true);
        assert_eq!(late.far, vec![false; 10]);
        let stage = edges(&rows, S, 1, 3, false);
        assert_eq!(stage.border, vec![false, false, false, false, false, false, false, false, true, false]);
    }

    #[test]
    fn means_skip_absent_values_and_unselected_rows() {
        let values = [Some(1.0), None, Some(3.0), Some(100.0)];
        assert_eq!(mean_over(&values, &[true, true, true, false]), Some(2.0));
        assert_eq!(mean_over(&values, &[false, true, false, false]), None);
    }
}
