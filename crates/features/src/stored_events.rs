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

//! The durations and rates of the locomotion states as the toolbox stored them in a reference
//! recording, `/features_events/worm_1/<mode>_time` and `<mode>_frequency`, and their comparison
//! with `states::event_stats` (docs/journal of 2026-11-04, *The Published Durations and Rates,
//! Reproduced? The Plan, Written Before the Numbers*). A mode with no events is stored as a single
//! NaN in both arrays.

use crate::states::{EventStats, MotionMode};
use crate::{ReadError, read_f64_column};
use std::path::Path;

/// The largest relative difference at which a stored duration or frequency counts as reproduced,
/// fixed before block 1 was compared.
pub const TOLERANCE: f64 = 1e-9;

/// One mode's events as the toolbox stored them.
#[derive(Clone, PartialEq, Debug)]
pub struct StoredEvents {
    /// Each event's duration in seconds, in order; empty when the mode has no events.
    pub durations: Vec<f64>,
    /// Events per second; `None` when stored as NaN.
    pub frequency: Option<f64>,
}

const MODES: [(MotionMode, &str); 3] = [(MotionMode::Forward, "forward"), (MotionMode::Backward, "backward"), (MotionMode::Paused, "paused")];

/// The stored events of each mode of the features file at `path`, in the order forward, backward,
/// paused, as `states::motion_events` gives them.
pub fn read_stored_events(path: &Path) -> Result<[(MotionMode, StoredEvents); 3], ReadError> {
    let mut read = MODES.iter().map(|&(mode, name)| {
        let column = |field: &str| read_f64_column(path, &format!("/features_events/worm_1/{name}_{field}"));
        Ok((mode, stored_events(column("time")?, column("frequency")?).map_err(ReadError::Format)?))
    });
    let mut next = || read.next().expect("three modes");
    Ok([next()?, next()?, next()?])
}

/// One mode's stored arrays as `StoredEvents`: a single NaN duration is no events, and any other
/// NaN duration, or a frequency that is not one value, is refused.
fn stored_events(durations: Vec<f64>, frequency: Vec<f64>) -> Result<StoredEvents, String> {
    let [frequency] = frequency[..] else {
        return Err(format!("{} frequencies, not one", frequency.len()));
    };
    let durations = match durations[..] {
        [only] if only.is_nan() => Vec::new(),
        _ if durations.iter().any(|duration| !duration.is_finite()) => return Err("a duration that is not finite".to_string()),
        _ => durations,
    };
    Ok(StoredEvents {
        durations,
        frequency: (!frequency.is_nan()).then_some(frequency),
    })
}

/// How one mode's events, recomputed, compare with the stored ones.
#[derive(Clone, PartialEq, Debug)]
pub struct Agreement {
    /// The number of events recomputed and stored.
    pub counts: (usize, usize),
    /// The mean duration in seconds, recomputed and stored; `None` with no events.
    pub mean_durations: (Option<f64>, Option<f64>),
    /// The frequency, recomputed and stored.
    pub frequencies: (Option<f64>, Option<f64>),
    /// The largest relative difference between paired durations, `None` when the counts differ.
    pub worst_duration: Option<f64>,
    /// The relative difference between the frequencies; 0 when both are `None`, infinite when one is.
    pub frequency_difference: f64,
}

impl Agreement {
    /// Whether the counts are equal and every duration and the frequency within `TOLERANCE`.
    pub fn reproduced(&self) -> bool {
        self.counts.0 == self.counts.1 && self.worst_duration.is_some_and(|worst| worst <= TOLERANCE) && self.frequency_difference <= TOLERANCE
    }
}

fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

/// `|ours - stored| / |stored|`, or the plain difference where the stored value is 0.
fn relative(ours: f64, stored: f64) -> f64 {
    let difference = (ours - stored).abs();
    if stored == 0.0 { difference } else { difference / stored.abs() }
}

/// Compares one mode's recomputed statistics with its stored events.
pub fn compare(ours: &EventStats, stored: &StoredEvents) -> Agreement {
    let counts = (ours.durations.len(), stored.durations.len());
    let worst_duration = (counts.0 == counts.1).then(|| {
        ours.durations
            .iter()
            .zip(&stored.durations)
            .map(|(&ours, &stored)| relative(ours, stored))
            .fold(0.0, f64::max)
    });
    let frequency_difference = match (ours.frequency, stored.frequency) {
        (None, None) => 0.0,
        (Some(ours), Some(stored)) => relative(ours, stored),
        _ => f64::INFINITY,
    };
    Agreement {
        counts,
        mean_durations: (mean(&ours.durations), mean(&stored.durations)),
        frequencies: (ours.frequency, stored.frequency),
        worst_duration,
        frequency_difference,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::states::event_stats;

    #[test]
    fn a_single_nan_is_no_events_and_other_shapes_are_refused() {
        let none = stored_events(vec![f64::NAN], vec![f64::NAN]).unwrap();
        assert_eq!(none.durations, Vec::<f64>::new());
        assert_eq!(none.frequency, None);
        let some = stored_events(vec![1.5, 0.5], vec![0.25]).unwrap();
        assert_eq!((some.durations, some.frequency), (vec![1.5, 0.5], Some(0.25)));
        assert!(stored_events(vec![1.0, f64::NAN], vec![0.1]).is_err());
        assert!(stored_events(vec![1.0], vec![0.1, 0.2]).is_err());
        assert!(stored_events(vec![1.0], vec![]).is_err());
    }

    // Two events of 4 and 2 frames at 2 frames per second in 10 frames, neither at an edge.
    fn two_events() -> EventStats {
        event_stats(&[(1, 4), (7, 8)], 10, 2.0)
    }

    #[test]
    fn the_same_events_are_reproduced() {
        let stored = StoredEvents {
            durations: vec![2.0, 1.0],
            frequency: Some(2.0 / 5.0),
        };
        let agreement = compare(&two_events(), &stored);
        assert_eq!(agreement.counts, (2, 2));
        assert_eq!(agreement.mean_durations, (Some(1.5), Some(1.5)));
        assert_eq!(agreement.worst_duration, Some(0.0));
        assert!(agreement.reproduced());
    }

    #[test]
    fn a_frame_too_many_a_missing_event_or_a_missing_frequency_is_not() {
        let longer = StoredEvents {
            durations: vec![2.0, 1.5],
            frequency: Some(0.4),
        };
        let agreement = compare(&two_events(), &longer);
        assert_eq!(agreement.worst_duration, Some(1.0 / 3.0));
        assert!(!agreement.reproduced());
        let fewer = StoredEvents {
            durations: vec![2.0],
            frequency: Some(0.4),
        };
        assert_eq!(compare(&two_events(), &fewer).worst_duration, None);
        assert!(!compare(&two_events(), &fewer).reproduced());
        let unrated = StoredEvents {
            durations: vec![2.0, 1.0],
            frequency: None,
        };
        assert!(!compare(&two_events(), &unrated).reproduced());
        let rarer = StoredEvents {
            durations: vec![2.0, 1.0],
            frequency: Some(0.4 * (1.0 + 1e-8)),
        };
        assert!(!compare(&two_events(), &rarer).reproduced());
    }

    #[test]
    fn no_events_on_both_sides_are_reproduced() {
        let none = StoredEvents {
            durations: Vec::new(),
            frequency: None,
        };
        let agreement = compare(&event_stats(&[], 10, 2.0), &none);
        assert_eq!((agreement.counts, agreement.mean_durations), ((0, 0), (None, None)));
        assert!(agreement.reproduced());
    }
}
