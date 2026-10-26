"""Agarland (PREREG29): a world to live in. Depleting, regrowing bacterial patches (mostly on the warm side),
barren salt pans, a temperature gradient. Each worm lives in its own copy (per-worm food state)."""
import numpy as np

ARENA = 50.0
PATCHES = np.array([[18, 10], [25, -15], [32, 22], [38, -5], [22, 30], [30, -32], [-28, 15], [-33, -20]], float)
PATCH_R, ODOUR_SIG = 6.0, 10.0
SALT = np.array([[0, 35], [-10, -38], [-40, 0]], float); SALT_SIG = 8.0


class LifeWorld:
    def __init__(self, n):
        self.F = np.ones((n, len(PATCHES))); self.which = np.full(n, -1)

    def at(self, xy):
        n = len(xy); d = np.linalg.norm(xy[:, None] - PATCHES[None], axis=2)              # (n, k)
        inside = d < PATCH_R
        self.which = np.where(inside.any(1), inside.argmax(1), -1)
        b = np.where(self.which >= 0, self.F[np.arange(n), np.maximum(self.which, 0)], 0.0)
        o = np.clip((self.F * np.exp(-d ** 2 / (2 * ODOUR_SIG ** 2))).sum(1), 0, 1)
        ds = np.linalg.norm(xy[:, None] - SALT[None], axis=2); s = np.clip(np.exp(-ds ** 2 / (2 * SALT_SIG ** 2)).sum(1), 0, 1)
        t = 20 + 5 * xy[:, 0] / ARENA
        return o, s, np.zeros(n), t, b

    def consume(self, intake):
        self.F += (1 - self.F) / 7200
        m = self.which >= 0; idx = np.nonzero(m)[0]
        self.F[idx, self.which[idx]] = np.clip(self.F[idx, self.which[idx]] - intake[idx] / 30, 0, 1)


class HardWorld:
    """Hardland (PREREG31): odourless food in the cool band, fast-depleting smelly food, decoy smells, barren salt."""
    ODOURLESS = np.array([[-35, -24], [-35, -8], [-35, 8], [-35, 24]], float)
    SMELLY = np.array([[25, 15], [30, -20]], float)
    DECOY = np.array([[35, 0], [15, -35], [10, 30]], float)
    SALTS = np.array([[0, 40], [-5, -42]], float)

    def __init__(self, n):
        self.P = np.vstack([self.ODOURLESS, self.SMELLY]); self.k_odourless = 4
        self.F = np.ones((n, len(self.P))); self.which = np.full(n, -1)
        self.cap = np.array([30.0] * 4 + [10.0] * 2); self.regrow = np.array([7200.0] * 4 + [21600.0] * 2)

    def at(self, xy):
        n = len(xy); d = np.linalg.norm(xy[:, None] - self.P[None], axis=2); inside = d < PATCH_R
        self.which = np.where(inside.any(1), inside.argmax(1), -1)
        b = np.where(self.which >= 0, self.F[np.arange(n), np.maximum(self.which, 0)], 0.0)
        o = (self.F[:, 4:] * np.exp(-d[:, 4:] ** 2 / (2 * ODOUR_SIG ** 2))).sum(1)
        dd = np.linalg.norm(xy[:, None] - self.DECOY[None], axis=2); o = np.clip(o + np.exp(-dd ** 2 / (2 * ODOUR_SIG ** 2)).sum(1), 0, 1)
        ds = np.linalg.norm(xy[:, None] - self.SALTS[None], axis=2); s = np.clip(np.exp(-ds ** 2 / (2 * SALT_SIG ** 2)).sum(1), 0, 1)
        return o, s, np.zeros(n), 20 + 5 * xy[:, 0] / ARENA, b

    def consume(self, intake):
        self.F += (1 - self.F) / self.regrow
        idx = np.nonzero(self.which >= 0)[0]; k = self.which[idx]
        self.F[idx, k] = np.clip(self.F[idx, k] - intake[idx] / self.cap[k], 0, 1)


class RandomHardWorld(HardWorld):
    """Hardland with a random layout: odourless food band on side s (s = −1 cool, +1 warm), smelly food on the other side,
    decoys and salt pans at random places. All worms share the layout; food state is per worm."""
    def __init__(self, n, rng, side=None):
        s = rng.choice([-1, 1]) if side is None else side; self.side = s
        ys = np.array([-24, -8, 8, 24], float) + rng.uniform(-3, 3)
        self.ODOURLESS = np.c_[np.full(4, 35.0 * s), ys]
        self.SMELLY = np.c_[-s * rng.uniform(22, 32, 2), rng.uniform(-25, 25, 2)]
        def rnd(k, rmax=40):
            a = rng.uniform(0, 2 * np.pi, k); r = rmax * np.sqrt(rng.uniform(0.05, 1, k)); return np.c_[r * np.cos(a), r * np.sin(a)]
        self.DECOY = rnd(3); self.SALTS = rnd(2)
        super().__init__(n)


class RandomRichWorld(LifeWorld):
    """Agarland with a random layout: 6 patches on a random side, 2 on the other, 3 random salt pans."""
    def __init__(self, n, rng):
        s = rng.choice([-1, 1]); self.side = s
        a = np.c_[s * rng.uniform(12, 40, 6), rng.uniform(-30, 30, 6)]; b = np.c_[-s * rng.uniform(20, 38, 2), rng.uniform(-25, 25, 2)]
        self.PATCHES_ = np.vstack([a, b])
        ang = rng.uniform(0, 2 * np.pi, 3); r = 40 * np.sqrt(rng.uniform(0.05, 1, 3)); self.SALT_ = np.c_[r * np.cos(ang), r * np.sin(ang)]
        super().__init__(n)

    def at(self, xy):
        global PATCHES, SALT
        P0, S0 = PATCHES, SALT; PATCHES, SALT = self.PATCHES_, self.SALT_
        try: return super().at(xy)
        finally: PATCHES, SALT = P0, S0
