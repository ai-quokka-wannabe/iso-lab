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

//! The gait, free of the C interface: what each joint is asked to hold at a moment.

/// Segments in the chain, the head counted: the Grid's longest (`TGL_SEGMENTS_MAX`).
pub const SEGMENTS: usize = 8;

/// The undulation's frequency, in hertz (the crate's documentation says where it comes from).
pub const FREQUENCY_HZ: f64 = 0.18;

/// The undulation's amplitude, in radians: a third of the Grid's 0.9 rad joint limit.
pub const AMPLITUDE_RAD: f64 = 0.3;

/// A sine-like wave of period one: a parabola up over the first half and down over the second,
/// 1 at a quarter, -1 at three quarters, 0 at the half periods. Only basic arithmetic and `floor`.
pub fn wave(phase: f64) -> f64 {
    let u = phase - phase.floor();
    if u < 0.5 { 16.0 * u * (0.5 - u) } else { -16.0 * (u - 0.5) * (1.0 - u) }
}

/// The angle each joint is asked to hold at `seconds` into the run, joint `k` between segments `k`
/// and `k + 1`: one wavelength along the body, travelling from head to tail, every angle within
/// `limit` of zero.
pub fn joint_targets(seconds: f64, limit: f64) -> [f64; SEGMENTS - 1] {
    let amplitude = AMPLITUDE_RAD.min(limit.max(0.0));
    std::array::from_fn(|joint| amplitude * wave(FREQUENCY_HZ * seconds - joint as f64 / (SEGMENTS - 1) as f64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wave_peaks_and_crosses_where_a_sine_does() {
        for (phase, expected) in [(0.0, 0.0), (0.25, 1.0), (0.5, 0.0), (0.75, -1.0), (1.25, 1.0), (-0.25, -1.0)] {
            assert!((wave(phase) - expected).abs() < 1e-12, "wave({phase}) = {}", wave(phase));
        }
        for i in 0..1000 {
            let w = wave(i as f64 / 997.0);
            assert!((-1.0..=1.0).contains(&w), "{w}");
        }
    }

    #[test]
    fn the_wave_travels_from_head_to_tail() {
        // A quarter period in, the head joint is at its peak; each joint behind it reaches the
        // same peak a seventh of a period later.
        let quarter = 0.25 / FREQUENCY_HZ;
        let at_quarter = joint_targets(quarter, 0.9);
        assert!((at_quarter[0] - AMPLITUDE_RAD).abs() < 1e-12, "{at_quarter:?}");
        let later = joint_targets(quarter + 1.0 / (7.0 * FREQUENCY_HZ), 0.9);
        assert!((later[1] - AMPLITUDE_RAD).abs() < 1e-9, "{later:?}");
    }

    #[test]
    fn no_target_passes_the_bodys_limit() {
        for tick in 0..2000 {
            for limit in [0.9, 0.1, 0.0] {
                let targets = joint_targets(tick as f64 / 32.0, limit);
                assert!(targets.iter().all(|t| t.abs() <= limit.min(AMPLITUDE_RAD) + 1e-15), "{targets:?} at {limit}");
            }
        }
    }
}
