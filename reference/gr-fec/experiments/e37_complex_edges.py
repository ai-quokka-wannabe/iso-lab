"""PREREG26: complex vs real edges for timing composition."""
import numpy as np
rng = np.random.default_rng(9000); L = 16; w = 2 * np.pi / L; K3 = np.arange(1, 4)
def episodes(lags, n=200):
    return np.array([rng.choice(lags) + rng.integers(-1, 2) for _ in range(n)])
def edges(samples):
    rl = np.bincount(samples, minlength=L)[:L] / len(samples)
    return dict(R0=float(rl.sum()), RL=rl, C1=np.mean(np.exp(1j * w * samples)), C3=np.array([np.mean(np.exp(1j * w * k * samples)) for k in K3]))
def compose(e1, e2):
    return dict(R0=e1['R0'] * e2['R0'], RL=np.convolve(e1['RL'], e2['RL'])[:L + 8], C1=e1['C1'] * e2['C1'], C3=e1['C3'] * e2['C3'])
def decode_c3(c3):
    t = np.arange(L); rec = 1 / L + (2 / L) * np.real(np.sum(c3[:, None] * np.exp(-1j * w * K3[:, None] * t[None]), 0)); return int(np.argmax(rec)), rec
AB = edges(episodes([3]))
for case, lags in (('U unimodal', [4]), ('M bimodal', [2, 8])):
    BF = edges(episodes(lags)); AF = compose(AB, BF)
    c1_t = (np.angle(AF['C1']) / w) % L; c1_conf = abs(AF['C1']); c3_t, rec = decode_c3(AF['C3'])
    rl = AF['RL']; peaks = [i for i in range(1, len(rl) - 1) if rl[i] >= rl[i - 1] and rl[i] >= rl[i + 1] and rl[i] > 0.05]
    print(f'{case}: R0 strength {AF["R0"]:.2f} (no timing) | RL peaks at {peaks} (16 numbers/edge) | '
          f'C1 time {c1_t:.2f}, confidence |z| {c1_conf:.2f} (2 numbers/edge) | C3 peak at {c3_t} (6 numbers/edge)')
    if case.startswith('U'): confU = c1_conf; ok_b = abs(c1_t - 7) <= .5; print(f'   P26b: C1 within ±0.5 of 7: {ok_b}')
    else:
        ok_c = abs(c1_t - 8) <= 1 and c1_conf < .5 * confU and 5 in peaks and 11 in peaks and (abs(c3_t - 5) <= 1 or abs(c3_t - 11) <= 1)
        print(f'   P26c: C1 near mean & low confidence ({c1_conf:.2f} < {.5*confU:.2f}), RL finds 5 and 11, C3 hits a true peak: {ok_c}')
