"""Organic v8 (PREREG42): organic v7 + evolvable steering (value-steering, innate lateral preference, steering noise)."""
import numpy as np
import evolve2 as E2
from evolve2 import OrganicBrain, C, MXC

NAMES = E2.NAMES + ['k_value', 'k_innate', 'lsnoise']
LO = np.r_[E2.LO, -5, -5, -3]; HI = np.r_[E2.HI, 5, 5, 0]; DISCRETE = E2.DISCRETE


def random_genomes(n, rng):
    G = LO + (HI - LO) * rng.random((n, len(NAMES))); G[:, DISCRETE] = rng.integers(0, C, (n, 2)); return G


def mutate(G, rng, sigma=0.1, p_disc=0.1):
    M = np.clip(G + sigma * (HI - LO) * rng.standard_normal(G.shape), LO, HI); M[:, DISCRETE] = G[:, DISCRETE]
    flip = rng.random((len(G), 2)) < p_disc
    M[:, DISCRETE] = np.where(flip, rng.integers(0, C, (len(G), 2)), M[:, DISCRETE]); return M


class OrganicSteerBrain(OrganicBrain):
    def __init__(self, n, rng, G):
        super().__init__(n, rng, G[:, :len(E2.NAMES)])
        self.kv, self.ki, self.snoise = G[:, -3], G[:, -2], 10 ** G[:, -1]
        self.lat = np.zeros(n); self.steer = np.zeros(n)

    def value_slope(self, y):
        """dV/dz of the brain's learned value of temperature at the current reading (z = (T − 20)/5)."""
        z = (y[:, 3] - 20) / 5; c = 3; w = self.width[:, c]; exist = np.arange(MXC)[None] < self.cnt[:, c][:, None]
        P = self.proto[:, c]; act = np.exp(-(P - z[:, None]) ** 2 / (2 * w[:, None] ** 2)) * exist
        Wn = self.W[:, c * MXC:(c + 1) * MXC]; slope = (Wn * act * (-(z[:, None] - P) / w[:, None] ** 2)).sum(1)
        return slope + self.W[:, C * MXC + c]

    def step(self, y):
        pir, rev = super().step(y)
        self.lat += (y[:, 8] / 0.02 - self.lat) / 10
        drive = self.kv * self.lat * self.value_slope(y) + self.ki * self.lat
        self.steer = np.clip(np.tanh(drive) + self.snoise * self.rng.standard_normal(self.n), -1, 1)
        return pir, rev
