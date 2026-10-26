"""Embodied worm v4 (PREREG28). Strict layers: World (physics/chemistry), Body (hidden state, metabolism, sensory
neurons, motor), Brain (hypergraph; sees ONLY the sensory signal vector y, outputs only motor commands).

Signal vector y (columns): 0 odour, 1 salt, 2 neutral odour N, 3 temperature, 4 touch, 5 pharynx (eating),
6 gut stretch, 7 hunger hormone.
"""
import numpy as np

R_PLATE, SPEED, HEAD_SD = 25.0, 0.2, 0.05


def _per(x, n):
    return np.full(n, float(x)) if np.isscalar(x) else np.asarray(x, float)


class World:
    """A plate. Any uniform quantity may be a per-worm array (= different worms sitting on different plates)."""
    def __init__(self, odour=False, salt=False, N=False, thermal=False, temp=20.0, lawn=0.0,
                 salt_uniform=0.0, N_uniform=0.0, peak=(15.0, 0.0), sigma=12.0, decoy=None):
        self.odour, self.salt, self.N, self.thermal, self.temp, self.lawn = odour, salt, N, thermal, temp, lawn
        self.salt_uniform, self.N_uniform, self.peak, self.sigma, self.decoy = salt_uniform, N_uniform, np.array(peak), sigma, decoy

    def at(self, xy):
        n = len(xy); bump = np.exp(-((xy - self.peak) ** 2).sum(1) / (2 * self.sigma ** 2))
        o = bump.copy() if self.odour else np.zeros(n)
        if self.odour and self.decoy is not None:
            o += self.decoy[1] * np.exp(-((xy - np.array(self.decoy[0])) ** 2).sum(1) / (2 * self.decoy[2] ** 2))
        s = bump.copy() if self.salt else _per(self.salt_uniform, n)
        N = bump.copy() if self.N else _per(self.N_uniform, n)
        t = 20 + 5 * xy[:, 0] / R_PLATE if self.thermal else _per(self.temp, n)
        return o, s, N, t, _per(self.lawn, n)


class Body:
    def __init__(self, n, rng):
        self.n, self.rng = n, rng
        self.G = np.zeros(n); self.E = np.full(n, 0.6); self.hormone = 1 - self.E.copy(); self.intake = np.zeros(n)
        self.place()

    def place(self, xy=None):
        self.xy = np.zeros((self.n, 2)) if xy is None else xy.copy(); self.head = self.rng.uniform(0, 2 * np.pi, self.n)

    def sense(self, world, tap=False):
        o, s, N, t, b = world.at(self.xy); r = self.rng.standard_normal
        n = self.n
        y = np.c_[o + .01 * r(n), s + .01 * r(n), N + .01 * r(n), t + .05 * r(n),
                  (1.0 if tap else 0.0) + .05 * r(n), self.intake / 0.02 + .02 * r(n),
                  self.G + .02 * r(n), self.hormone + .02 * r(n)]
        return y, b

    def metabolise(self, b):
        self.intake = 0.02 * b * (1 - self.G)
        self.G += self.intake - 0.004 * self.G
        self.E += 0.003 * self.G * (1 - self.E) - 1 / 5400; self.E = np.clip(self.E, 0, 1)
        self.hormone += ((1 - self.E) - self.hormone) / 120

    def act(self, pirouette, move=True):
        self.head = np.where(pirouette, self.rng.uniform(0, 2 * np.pi, self.n), self.head) + HEAD_SD * self.rng.standard_normal(self.n)
        self.xy += SPEED * np.c_[np.cos(self.head), np.sin(self.head)]
        R = getattr(self, 'R', R_PLATE)
        rr = np.linalg.norm(self.xy, axis=1); out = rr > R
        if out.any(): self.xy[out] *= (R / rr[out])[:, None]; self.head[out] += np.pi


MX = 30; NCH = 4                                    # tuned-node pools: odour, salt, N, temperature
WID = np.array([0.15, 0.15, 0.15, 1.0]); NOISE = np.array([0.01, 0.01, 0.01, 0.05])
NT = NCH * MX + 3                                   # + intensity nodes for odour, salt, N


class Brain:
    """Sees only y. Genome: sensor tuning widths, intensity-node edges, reward wiring, drive wiring, touch reflex,
    learning rate, turning rule. Everything else is learned."""
    def __init__(self, n, rng, s0=0.03, tau=0.3, eta=5e-4, hyper=True, random_walk_p=None, integ=0.0, separate=False, trust_k=0.0, curiosity=0.0, roam=0.0):
        self.n, self.rng, self.s0, self.tau, self.eta, self.hyper, self.rw = n, rng, s0, tau, eta, hyper, random_walk_p
        self.integ, self.Dbar, self.lag = integ, None, (10 if integ else 4)
        self.separate = separate
        self.trust_k, self.curiosity, self.nov, self.births = trust_k, curiosity, np.zeros(n), np.zeros(n)
        self.roam = roam
        if separate:                                                         # v5: instinct pathway outside learning
            self.I_w = np.zeros(2 * NT); self.I_w[NCH * MX:NCH * MX + 3] = [2.354, 0.706, 0.0]
        self.W = np.zeros((n, 2 * NT)); self.W[:, NCH * MX:NCH * MX + 3] = [0.0, 0.0, 0.0] if separate else [2.354, 0.706, 0.0]
        self.proto = np.zeros((n, NCH, MX)); self.cnt = np.zeros((n, NCH), int)
        self.touch_w = np.full(n, 0.9); self.hist = []

    def features(self, y):
        S = np.zeros((self.n, NT)); ar = np.arange(self.n)
        for m in range(NCH):
            x = y[:, m]; exist = np.arange(MX)[None] < self.cnt[:, m][:, None]
            d = np.where(exist, np.abs(self.proto[:, m] - x[:, None]), np.inf); dmin = d.min(1); j = d.argmin(1)
            event = (x > 3 * NOISE[m]) if m < 3 else np.ones(self.n, bool)
            new = event & (dmin > WID[m]) & (self.cnt[:, m] < MX)
            k = self.cnt[new, m]; self.proto[ar[new], m, k] = x[new]; self.cnt[new, m] += 1; self.births += new
            upd = event & ~new & np.isfinite(dmin)
            self.proto[ar[upd], m, j[upd]] += 0.005 * (x[upd] - self.proto[ar[upd], m, j[upd]])
            exist = np.arange(MX)[None] < self.cnt[:, m][:, None]
            act = np.exp(-(self.proto[:, m] - x[:, None]) ** 2 / (2 * WID[m] ** 2)) * exist
            if m < 3: act = act * event[:, None]
            S[:, m * MX:(m + 1) * MX] = act
        S[:, NCH * MX:] = np.clip(y[:, :3], 0, None)
        h = np.clip(y[:, 7], 0, 1)
        return np.hstack([S, S * h[:, None] if self.hyper else np.zeros_like(S)])

    def step(self, y):
        self.births = np.zeros(self.n); F = self.features(y)
        r = 4 * np.clip(y[:, 5], 0, None) - np.clip(y[:, 7], 0, 1)          # genome: eating good, hunger bad
        pred = (self.W * F).sum(1)
        self.W += self.eta * (r - pred)[:, None] * F                        # error never includes the instinct (v5)
        D = pred + np.clip(y[:, 6], 0, None)                                # genome: gut stretch feeds the drive
        if self.separate:
            if self.trust_k:                                                 # v6: learned trust in each instinct
                c0 = NCH * MX; h = np.clip(y[:, 7], 0, 1); x = np.clip(y[:, :2], 0, None)
                wl = self.W[:, c0:c0 + 2] + h[:, None] * self.W[:, NT + c0:NT + c0 + 2]
                t = np.clip(1 + self.trust_k * wl, 0, 1.5)
                D = D + (x * t * np.array([2.354, 0.706])).sum(1)
            else: D = D + F @ self.I_w                                       # v5: instinct drive added outside learning
        if self.curiosity:                                                   # v6: novelty (node births) feeds the drive
            self.nov = self.nov * np.exp(-1 / 300) + self.births; D = D + self.curiosity * self.nov
        if self.integ:                                                       # v4b: leaky integrator before the trend
            self.Dbar = D.copy() if self.Dbar is None else self.Dbar + (D - self.Dbar) / self.integ; D = self.Dbar
        self.hist.append(D)
        L = self.lag
        dD = D - self.hist[-(L + 1)] if len(self.hist) >= L + 1 else np.zeros(self.n)
        if len(self.hist) > L + 1: self.hist.pop(0)
        if self.rw is not None: P = np.full(self.n, self.rw)
        else: P = 0.3 / (1 + np.exp(np.clip((dD / self.s0 + 1) / self.tau, -50, 50)))
        if self.roam:                                                        # day 2: hungry, empty-gutted worms roam
            P = P / (1 + self.roam * np.clip(y[:, 7], 0, 1) * (1 - np.clip(y[:, 6], 0, 1)))
        pir = self.rng.random(self.n) < P
        self.touch_w += (0.9 - self.touch_w) / 300
        tapped = y[:, 4] > 0.5
        rev = tapped & (self.rng.random(self.n) < self.touch_w); self.touch_w = np.where(tapped, self.touch_w * 0.88, self.touch_w)
        return pir, rev

    def new_place(self):
        pass                                                                 # nothing is told to the brain


class Worm:
    def __init__(self, n, seed, **brain_kw):
        rng = np.random.default_rng(seed); self.body = Body(n, rng); self.brain = Brain(n, rng, **brain_kw); self.n = n

    def transfer(self, xy=None):                                             # experimenter picks worms up
        self.body.place(xy); self.brain.new_place()

    def run(self, world, secs, taps=None):
        pirs = []; revs = []
        for t in range(secs):
            tap = taps is not None and t in taps
            y, b = self.body.sense(world, tap)
            pir, rev = self.brain.step(y)
            self.body.act(pir); self.body.metabolise(b)
            if hasattr(world, 'consume'): world.consume(self.body.intake)
            pirs.append(pir.mean()); revs.append(rev.mean() if tap else np.nan)
        return np.array(pirs), np.array(revs)
