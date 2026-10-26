"""PREREG5 analysis: Nishimori line vs FER and decoder chaos."""
import numpy as np
from scipy.stats import spearmanr

A = [0.5, 0.7, 0.85, 1.0, 1.2, 1.5, 2.0]
seed = 500
D = {a: dict(np.load(f'results/e07_alpha{a}_s{seed}.npz')) for a in A}


def wilson(k, n, z=1.96):
    p = k / n; d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d; h = z * np.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return c - h, c + h


print(f"{'alpha':>6} {'FER':>8} {'95% CI':>20} {'undet':>6} {'FTLE(fail) med':>15} {'fixed%':>7} {'drift%':>7} {'osc%':>6} {'entropy@50 fail':>16}")
rows = []
for a in A:
    d = D[a]; fail = d['conv'] > 50; n = len(fail); k = fail.sum()
    lo, hi = wilson(k, n)
    cls = np.bincount(d['endclass'][fail], minlength=3) / max(k, 1)
    rows.append((a, k / n, lo, hi, cls))
    print(f"{a:6.2f} {k/n:8.4f}   [{lo:.4f}, {hi:.4f}] {d['undet'].sum():6d} {np.median(d['ftle'][fail]):+15.3f} "
          f"{cls[0]*100:7.1f} {cls[1]*100:7.1f} {cls[2]*100:6.1f} {np.median(d['ent_end'][fail]):16.2f}")

# P5: Spearman(alpha, FTLE) over all failing frames, one-sided permutation test
xs = np.concatenate([[a] * int((D[a]['conv'] > 50).sum()) for a in A])
ys = np.concatenate([D[a]['ftle'][D[a]['conv'] > 50] for a in A])
rho = spearmanr(xs, ys).statistic
rng = np.random.default_rng(0)
null = np.array([spearmanr(xs, rng.permutation(ys)).statistic for _ in range(5000)])
p = (np.sum(null >= rho) + 1) / 5001
osc05, osc20 = rows[0][4][2], rows[-1][4][2]
print(f"\nP5: Spearman(alpha, FTLE | failing frames) = {rho:+.3f}, one-sided p = {p:.4f};  oscillating share {osc05*100:.1f}% (a=0.5) vs {osc20*100:.1f}% (a=2.0)")
print('    P5 PASSES' if (rho > 0 and p < 0.01 and osc20 > osc05) else '    P5 FAILS')

# S5: FER minimised inside [0.85, 1.2]
inside = [r for r in rows if 0.85 <= r[0] <= 1.2]; best = min(inside, key=lambda r: r[1]); glob = min(rows, key=lambda r: r[1])
s5 = best[3] < rows[0][2] and best[3] < rows[-1][2]
print(f"S5: global FER minimum at alpha={glob[0]}; best inside window alpha={best[0]} FER={best[1]:.4f} [{best[2]:.4f},{best[3]:.4f}]; "
      f"edges: a=0.5 [{rows[0][2]:.4f},{rows[0][3]:.4f}], a=2.0 [{rows[-1][2]:.4f},{rows[-1][3]:.4f}] -> {'PASSES' if s5 and glob[0] in (0.85, 1.0, 1.2) else 'FAILS'}")

# S6: fixed-point share decreases with alpha
fixed = [r[4][0] for r in rows]
print(f"S6: Spearman(alpha, fixed-point share) = {spearmanr(A, fixed).statistic:+.3f} -> {'PASSES' if spearmanr(A, fixed).statistic < 0 else 'FAILS'}")

# exploratory: does FTLE at alpha=1 separate failing from succeeding-late frames? (sanity link to PREREG2)
d = D[1.0]; late_ok = (d['conv'] > 30) & (d['conv'] <= 50); fail = d['conv'] > 50
print(f"\n(exploratory) alpha=1: frames converging after it.30: {late_ok.sum()}, failing: {fail.sum()}")
