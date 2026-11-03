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

//! A rate network trained by Hebb's rule, with the stabilisers and the "used together" rule of
//! E4 (docs/hypotheses/experiments/E1-E5.md § E4): the pilot of `notebooks/e4_pilot.ipynb`,
//! rebuilt in Rust so that its four assertions are tests.
//!
//! **The network.** `N` rate units, each rate in [0, 1]. Given an external input `u` and each
//! unit's resting drive `b`, the rates settle from zero by damped steps,
//! `r <- 0.5 r + 0.5 clip(W r + u + b, 0, 1)`. Where the notebook took 100 steps, [`Network::settle`]
//! steps until no rate moves by more than [`SETTLE_TOLERANCE`], as assertion 2 asks, so a rate near
//! the ignition threshold is not cut short.
//!
//! **Training.** Pair trials drive units 0 and 1 (`u = 1`); background trials drive [`K`] of the
//! units 2 to 79, drawn from the seed. The trials' order is a seeded shuffle. After each trial
//! settles, the weights change by the [`Rule`], using the settled rates `y`; self-weights stay zero.
//! Under the "used together" rule `y_i` is replaced by `y_i [u_i > 0]`, so only units the trial's
//! input drove take part.
//!
//! **The rules**, each the form commonly given for it (the equations follow the notebook; they were
//! not re-derived from the papers here): Hebb, `dW_ij = eta y_i y_j`; Oja
//! ([1982](https://doi.org/10.1007/BF00275687)), `dW_ij = eta y_i (y_j - y_i W_ij)`; row
//! normalisation, Hebb then each unit's incoming weights scaled so their L2 norm is at most a bound;
//! BCM ([Bienenstock, Cooper and Munro 1982](https://doi.org/10.1523/JNEUROSCI.02-01-00032.1982)),
//! `dW_ij = eta y_i (y_i - theta_i) y_j` with `theta <- 0.9 theta + 0.1 y^2` from 0.1; homeostatic
//! scaling ([Turrigiano 2008](https://doi.org/10.1016/j.cell.2008.10.008)), Hebb then each unit's
//! incoming weights multiplied by `max(0, 1 + gamma (target - r_i))`, `r_i` the unit's settled rate,
//! unmasked.
//!
//! Everything is deterministic given a seed: one SplitMix64 generator draws the order and the cues.

/// Units in the network.
pub const N: usize = 100;
/// The co-activated pair.
pub const PAIR: (usize, usize) = (0, 1);
/// Units driven on a background trial or a cue.
pub const K: usize = 5;
/// The units background trials and cues are drawn from.
pub const OTHERS: std::ops::Range<usize> = 2..80;
/// The hub: units that rest high in the hub condition.
pub const HUB: std::ops::Range<usize> = 80..100;
/// Settling stops when no rate moves by more than this in one step.
pub const SETTLE_TOLERANCE: f64 = 1e-13;
/// Settling gives up after this many steps; [`Network::settle`] reports whether it converged.
pub const SETTLE_LIMIT: usize = 20_000;

/// The pair ignites itself from rest once its mutual weight passes `w* = 1 - 2b`: with no input its
/// settled rate is `b / (1 - w)`, which is 0.5 there.
pub fn ignition_threshold(rest: f64) -> f64 {
    1.0 - 2.0 * rest
}

/// A learning rule, with its stabiliser's constants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rule {
    Hebb,
    Oja,
    /// Hebb, then each row's L2 norm brought down to at most `bound`.
    Normalised {
        bound: f64,
    },
    Bcm,
    /// Hebb, then each row multiplied by `max(0, 1 + gamma (target - r_i))`.
    Scaled {
        gamma: f64,
        target: f64,
    },
}

/// One training run's settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Training {
    pub rule: Rule,
    pub eta: f64,
    /// Every unit's resting drive, except the hub's in the hub condition.
    pub rest: f64,
    /// The hub's resting drive, or `None` when the hub rests like every other unit.
    pub hub_rest: Option<f64>,
    pub pair_trials: usize,
    pub background_trials: usize,
    /// The "used together" rule.
    pub used_together: bool,
}

impl Training {
    /// The pilot's first constants: eta 0.05, resting drive 0.05, 200 pair trials and 100
    /// background trials, no hub, with the "used together" rule.
    pub fn pilot(rule: Rule) -> Self {
        Training {
            rule,
            eta: 0.05,
            rest: 0.05,
            hub_rest: None,
            pair_trials: 200,
            background_trials: 100,
            used_together: true,
        }
    }
}

/// The weights and the resting drives.
#[derive(Clone, Debug, PartialEq)]
pub struct Network {
    /// Row-major: `w[i * N + j]` is the weight from unit `j` into unit `i`.
    pub w: Vec<f64>,
    pub rest: Vec<f64>,
}

/// A trained network, W_01 after every trial, whether each trial was a pair trial, and which units
/// any trial drove.
#[derive(Clone, Debug)]
pub struct Trained {
    pub network: Network,
    pub w01: Vec<f64>,
    pub pair_trial: Vec<bool>,
    pub driven: Vec<bool>,
}

impl Network {
    /// No weights, every unit resting at `rest`.
    pub fn resting(rest: f64) -> Self {
        Network {
            w: vec![0.0; N * N],
            rest: vec![rest; N],
        }
    }

    pub fn weight(&self, to: usize, from: usize) -> f64 {
        self.w[to * N + from]
    }

    /// The settled rates for input `u`, and whether settling converged within [`SETTLE_LIMIT`].
    pub fn settle(&self, u: &[f64]) -> (Vec<f64>, bool) {
        // The weights are sparse - a pair and a few small cliques - so the product runs over the
        // nonzero entries only, in row-major order.
        let nonzero: Vec<(usize, usize, f64)> = (0..N * N).filter(|&k| self.w[k] != 0.0).map(|k| (k / N, k % N, self.w[k])).collect();
        let mut r = vec![0.0; N];
        let mut drive = vec![0.0; N];
        for _ in 0..SETTLE_LIMIT {
            drive.iter_mut().zip(u.iter().zip(&self.rest)).for_each(|(d, (ui, bi))| *d = ui + bi);
            for &(i, j, wij) in &nonzero {
                drive[i] += wij * r[j];
            }
            let mut moved: f64 = 0.0;
            for (ri, di) in r.iter_mut().zip(&drive) {
                let next = 0.5 * *ri + 0.5 * di.clamp(0.0, 1.0);
                moved = moved.max((next - *ri).abs());
                *ri = next;
            }
            if moved <= SETTLE_TOLERANCE {
                return (r, true);
            }
        }
        (r, false)
    }

    /// The share of `cues` unrelated cues - [`K`] of the units in [`OTHERS`] driven, drawn from
    /// `seed` - on which both units of the pair settle above 0.5.
    pub fn capture_share(&self, cues: usize, seed: u64) -> f64 {
        let mut rng = SplitMix64(seed);
        let mut captured = 0;
        for _ in 0..cues {
            let mut u = vec![0.0; N];
            for unit in rng.choose(OTHERS, K) {
                u[unit] = 1.0;
            }
            let (r, converged) = self.settle(&u);
            assert!(converged, "a cue did not settle");
            if r[PAIR.0] > 0.5 && r[PAIR.1] > 0.5 {
                captured += 1;
            }
        }
        f64::from(captured) / cues as f64
    }
}

/// Trains a network from no weights by `training`, the order and the background units drawn from
/// `seed`.
pub fn train(training: &Training, seed: u64) -> Trained {
    let mut rng = SplitMix64(seed);
    let mut network = Network::resting(training.rest);
    if let Some(hub_rest) = training.hub_rest {
        network.rest[HUB].fill(hub_rest);
    }
    let mut order = vec![true; training.pair_trials];
    order.extend(std::iter::repeat_n(false, training.background_trials));
    rng.shuffle(&mut order);
    let mut theta = vec![0.1; N];
    let mut driven = vec![false; N];
    let mut w01 = Vec::with_capacity(order.len());
    for &pair_trial in &order {
        let mut u = vec![0.0; N];
        if pair_trial {
            u[PAIR.0] = 1.0;
            u[PAIR.1] = 1.0;
        } else {
            for unit in rng.choose(OTHERS, K) {
                u[unit] = 1.0;
            }
        }
        for (d, &ui) in driven.iter_mut().zip(&u) {
            *d |= ui > 0.0;
        }
        let (rate, converged) = network.settle(&u);
        assert!(converged, "a training trial did not settle");
        let y: Vec<f64> = if training.used_together {
            rate.iter().zip(&u).map(|(&r, &ui)| if ui > 0.0 { r } else { 0.0 }).collect()
        } else {
            rate.clone()
        };
        let eta = training.eta;
        for i in 0..N {
            for j in 0..N {
                if i == j {
                    continue;
                }
                let k = i * N + j;
                network.w[k] += match training.rule {
                    Rule::Hebb | Rule::Normalised { .. } | Rule::Scaled { .. } => eta * y[i] * y[j],
                    Rule::Oja => eta * y[i] * (y[j] - y[i] * network.w[k]),
                    Rule::Bcm => eta * y[i] * (y[i] - theta[i]) * y[j],
                };
            }
        }
        match training.rule {
            Rule::Bcm => theta.iter_mut().zip(&y).for_each(|(t, &yi)| *t = 0.9 * *t + 0.1 * yi * yi),
            Rule::Normalised { bound } => {
                for row in network.w.chunks_mut(N) {
                    let norm = row.iter().map(|v| v * v).sum::<f64>().sqrt();
                    if norm > bound {
                        row.iter_mut().for_each(|v| *v *= bound / norm);
                    }
                }
            }
            Rule::Scaled { gamma, target } => {
                for (row, &ri) in network.w.chunks_mut(N).zip(&rate) {
                    let factor = (1.0 + gamma * (target - ri)).max(0.0);
                    row.iter_mut().for_each(|v| *v *= factor);
                }
            }
            Rule::Hebb | Rule::Oja => {}
        }
        w01.push(network.weight(PAIR.0, PAIR.1));
    }
    Trained {
        network,
        w01,
        pair_trial: order,
        driven,
    }
}

/// SplitMix64 ([Steele, Lea and Flood 2014](https://doi.org/10.1145/2660193.2660195)): one integer
/// of state, so a seed fixes every draw.
struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A uniform index below `bound`, by rejection.
    fn below(&mut self, bound: usize) -> usize {
        let bound = bound as u64;
        let zone = u64::MAX - u64::MAX % bound;
        loop {
            let draw = self.next();
            if draw < zone {
                return (draw % bound) as usize;
            }
        }
    }

    /// Fisher-Yates.
    fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
    }

    /// `count` distinct members of `range`, by a partial Fisher-Yates.
    fn choose(&mut self, range: std::ops::Range<usize>, count: usize) -> Vec<usize> {
        let mut pool: Vec<usize> = range.collect();
        for i in 0..count {
            let j = i + self.below(pool.len() - i);
            pool.swap(i, j);
        }
        pool.truncate(count);
        pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const REST: f64 = 0.05;
    const CUES: usize = 100;

    fn w_star() -> f64 {
        ignition_threshold(REST)
    }

    /// Assertion 1: under Hebb's rule with the "used together" rule, W_01 grows by eta on every pair
    /// trial - the driven pair settles at a rate of 1 - and by nothing on a background trial, and
    /// passes w*, after which the pair captures every cue.
    #[test]
    fn assertion_1_hebb_grows_by_eta_per_pair_trial_and_passes_ignition() {
        let training = Training::pilot(Rule::Hebb);
        let trained = train(&training, 42);
        assert_eq!(trained.pair_trial.iter().filter(|&&p| p).count(), training.pair_trials);
        let mut previous = 0.0;
        for (&w, &pair_trial) in trained.w01.iter().zip(&trained.pair_trial) {
            let step = w - previous;
            if pair_trial {
                assert!((step - training.eta).abs() < 1e-9, "pair trial step {step}");
            } else {
                assert_eq!(step, 0.0, "background trial step");
            }
            previous = w;
        }
        assert!(*trained.w01.last().unwrap() > w_star());
        assert_eq!(trained.network.capture_share(CUES, 9), 1.0);
    }

    /// Assertion 2: with no input and only the pair's two weights, the pair's settled rate is
    /// b / (1 - w), clipped at 1, and passes 0.5 exactly when w passes w* = 1 - 2b - settled to
    /// convergence, a millionth either side of w*.
    #[test]
    fn assertion_2_the_settled_rate_is_b_over_one_minus_w_and_ignites_at_w_star() {
        let at = |w: f64| {
            let mut network = Network::resting(REST);
            network.w[PAIR.0 * N + PAIR.1] = w;
            network.w[PAIR.1 * N + PAIR.0] = w;
            let (r, converged) = network.settle(&[0.0; N]);
            assert!(converged, "w = {w}");
            r[PAIR.0]
        };
        for w in [0.0, 0.3, 0.6, 0.85, 0.89, 0.899, 0.9, 0.92, 0.94, 0.949] {
            let expected = (REST / (1.0 - w)).min(1.0);
            assert!((at(w) - expected).abs() < 1e-9, "w = {w}: {} against {expected}", at(w));
        }
        assert!((at(0.96) - 1.0).abs() < 1e-9, "{}", at(0.96));
        assert!(at(w_star() - 1e-6) < 0.5);
        assert!(at(w_star() + 1e-6) > 0.5);
    }

    /// Assertion 3, row normalisation: W_01 never exceeds the bound, and the pair captures if and
    /// only if the bound exceeds w*.
    #[test]
    fn assertion_3_row_normalisation_captures_iff_its_bound_exceeds_w_star() {
        for (bound, captures) in [(0.5, false), (0.8, false), (1.0, true)] {
            let trained = train(&Training::pilot(Rule::Normalised { bound }), 42);
            assert!(trained.w01.iter().all(|&w| w <= bound + 1e-12), "bound {bound}");
            assert_eq!(trained.network.capture_share(CUES, 9) == 1.0, captures, "bound {bound}");
            assert_eq!(trained.network.capture_share(CUES, 9) == 0.0, !captures, "bound {bound}");
        }
    }

    /// Assertion 3, homeostatic scaling: W_01 settles near a fixed point - over the last 100 trials
    /// it moves less than one Hebbian step - and the pair captures if and only if that point exceeds
    /// w*: not at gamma 0.5 and target 0.1, the pilot's constants, but at gamma 0.1 and target 0.5.
    #[test]
    fn assertion_3_homeostatic_scaling_captures_iff_its_fixed_point_exceeds_w_star() {
        for (gamma, target, captures) in [(0.5, 0.1, false), (0.1, 0.5, true)] {
            let training = Training::pilot(Rule::Scaled { gamma, target });
            let trained = train(&training, 42);
            let tail = &trained.w01[trained.w01.len() - 100..];
            let (low, high) = tail.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), &w| (l.min(w), h.max(w)));
            assert!(high - low < training.eta, "gamma {gamma}, target {target}: {low}..{high}");
            assert_eq!(low > w_star(), captures, "gamma {gamma}, target {target}: {low}..{high}");
            assert_eq!(high < w_star(), !captures, "gamma {gamma}, target {target}: {low}..{high}");
            assert_eq!(trained.network.capture_share(CUES, 9), if captures { 1.0 } else { 0.0 });
        }
    }

    /// Assertion 3, Oja's rule: W_01 never exceeds 1, and with the pair saturated it follows
    /// 1 - (1 - eta)^k, so it passes w* on pair trial ln(1 - w*) / ln(1 - eta), rounded up - 460 at
    /// eta 0.005 - and the pair captures once it has.
    #[test]
    fn assertion_3_oja_is_bounded_at_one_and_captures_once_past_w_star() {
        let eta: f64 = 0.005;
        let crossing = ((1.0 - w_star()).ln() / (1.0 - eta).ln()).ceil() as usize;
        assert_eq!(crossing, 460);
        for (scale, captures) in [(1, false), (2, false), (4, true)] {
            let training = Training {
                eta,
                pair_trials: 200 * scale,
                background_trials: 100 * scale,
                ..Training::pilot(Rule::Oja)
            };
            let trained = train(&training, 42);
            assert!(trained.w01.iter().all(|&w| w <= 1.0), "scale {scale}");
            let mut pairs_so_far = 0;
            for (&w, &pair_trial) in trained.w01.iter().zip(&trained.pair_trial) {
                pairs_so_far += usize::from(pair_trial);
                assert_eq!(w > w_star(), pairs_so_far >= crossing, "after {pairs_so_far} pair trials: {w}");
            }
            assert_eq!(trained.network.capture_share(CUES, 9) == 1.0, captures, "scale {scale}");
        }
    }

    /// Assertion 3, BCM, in the pilot's mix of two pair trials to one background trial: W_01 never
    /// falls within a run, ends higher the longer the training, and the pair captures given enough
    /// trials - at eta 0.05 within the pilot's 300, at
    /// eta 0.005 by four times as many. With the background held at 100 trials it grows more slowly
    /// and has not captured by 800 pair trials, which is why the assertion names its mix.
    #[test]
    fn assertion_3_bcm_grows_in_the_pilots_mix_and_captures_given_enough_trials() {
        let fast = train(&Training::pilot(Rule::Bcm), 42);
        assert_eq!(fast.network.capture_share(CUES, 9), 1.0);
        let slow = |pair_trials: usize, background_trials: usize| {
            train(
                &Training {
                    eta: 0.005,
                    pair_trials,
                    background_trials,
                    ..Training::pilot(Rule::Bcm)
                },
                42,
            )
        };
        let runs: Vec<Trained> = [1, 2, 4].iter().map(|&s| slow(200 * s, 100 * s)).collect();
        // Within each run W_01 never falls: the pair's threshold stays below its saturated rate.
        for run in &runs {
            assert!(run.w01.windows(2).all(|pair| pair[1] >= pair[0]));
        }
        let mixed: Vec<f64> = runs.iter().map(|run| *run.w01.last().unwrap()).collect();
        assert!(mixed[0] < mixed[1] && mixed[1] < mixed[2], "{mixed:?}");
        assert!(mixed[1] < w_star() && mixed[2] > w_star(), "{mixed:?}");
        assert_eq!(slow(800, 400).network.capture_share(CUES, 9), 1.0);
        let held = slow(800, 100);
        assert!(*held.w01.last().unwrap() < mixed[2] / 2.0);
        assert_eq!(held.network.capture_share(CUES, 9), 0.0);
    }

    /// Assertion 4: under the "used together" rule, every weight into or out of a unit that no trial
    /// drove stays exactly zero, under every rule and in both conditions - the pair's, and the hub's,
    /// where 20 units rest at 0.6 and no pair trial is given.
    #[test]
    fn assertion_4_used_together_keeps_undriven_units_weights_at_exactly_zero() {
        let rules = [
            Rule::Hebb,
            Rule::Oja,
            Rule::Normalised { bound: 1.0 },
            Rule::Bcm,
            Rule::Scaled { gamma: 0.5, target: 0.1 },
        ];
        for rule in rules {
            for hub in [false, true] {
                let training = Training {
                    hub_rest: hub.then_some(0.6),
                    pair_trials: if hub { 0 } else { 200 },
                    ..Training::pilot(rule)
                };
                let trained = train(&training, 7);
                let undriven: Vec<usize> = (0..N).filter(|&i| !trained.driven[i]).collect();
                assert!(undriven.len() >= HUB.len(), "{rule:?}");
                for &i in &undriven {
                    for j in 0..N {
                        assert_eq!(trained.network.weight(i, j), 0.0, "{rule:?}, hub {hub}: into {i} from {j}");
                        assert_eq!(trained.network.weight(j, i), 0.0, "{rule:?}, hub {hub}: into {j} from {i}");
                    }
                }
            }
        }
    }

    /// The control for assertion 4: the hub does rest high - its units settle at 0.6 with no input
    /// and no weights - and without the "used together" rule, Hebb's rule grows a weight between two
    /// hub units that only rested, by eta 0.6^2 on the first trial (a rest of 0.05 gives eta 0.05^2).
    #[test]
    fn without_used_together_resting_units_grow_weights() {
        let hub = Training {
            hub_rest: Some(0.6),
            pair_trials: 0,
            used_together: false,
            ..Training::pilot(Rule::Hebb)
        };
        let fresh = train(&Training { background_trials: 0, ..hub }, 7).network;
        let (rate, converged) = fresh.settle(&[0.0; N]);
        assert!(converged && rate[HUB].iter().all(|r| (r - 0.6).abs() < 1e-9), "{:?}", &rate[HUB]);
        let first = train(&Training { background_trials: 1, ..hub }, 7).network;
        assert!((first.weight(HUB.start, HUB.start + 1) - hub.eta * 0.36).abs() < 1e-9);
        assert!(train(&hub, 7).network.weight(HUB.start, HUB.start + 1) > 0.0);
    }

    #[test]
    fn training_follows_the_seed() {
        let training = Training::pilot(Rule::Bcm);
        assert_eq!(train(&training, 3).network, train(&training, 3).network);
        assert_ne!(train(&training, 3).network, train(&training, 4).network);
    }
}
