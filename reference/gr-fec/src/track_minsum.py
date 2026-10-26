"""Flooding normalised min-sum with the same tracking interface as TrackBP (for check-regular codes).
c2v = beta * prod(sign) * min|.| over the other edges of the check."""
import numpy as np
from bp_track import TrackBP


class TrackMinSum(TrackBP):
    def __init__(self, H, beta=0.75):
        super().__init__(H)
        dc = H.sum(1); assert len(set(dc)) == 1
        self.dc = int(dc[0]); self.beta = beta
        assert np.all(np.diff(self.ec) >= 0)            # edges grouped by check (row-major)

    def step(self, v2c, llr):
        B = v2c.shape[0]; m, dc = self.m, self.dc
        V = v2c.reshape(B, m, dc)
        sg = np.where(V < 0, -1.0, 1.0); mg = np.abs(V)
        sprod = np.prod(sg, axis=2, keepdims=True) * sg
        o = np.argsort(mg, axis=2)
        m1 = np.take_along_axis(mg, o[:, :, :1], 2); m2 = np.take_along_axis(mg, o[:, :, 1:2], 2)
        excl = np.where(np.arange(dc)[None, None, :] == o[:, :, :1], m2, m1)
        c2v = (self.beta * sprod * excl).reshape(B, -1)
        tot = llr + (self.Sv.T @ c2v.T).T
        return tot[:, self.ev] - c2v, tot, c2v
