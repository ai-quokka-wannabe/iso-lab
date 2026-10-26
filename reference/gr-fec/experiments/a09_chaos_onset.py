"""PREREG6 analysis: does chaos switch on at the Nishimori line at other SNRs?"""
import numpy as np
from scipy.stats import spearmanr

G = [0.6, 0.75, 0.85, 0.92, 1.0, 1.08, 1.2, 1.4]


def wilson(k, n, z=1.96):
    p = k / n; d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d; h = z * np.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return c - h, c + h


ok_all = True
for eb in [2.5, 3.5]:
    fer, ft, fixed, osc, ci = [], [], [], [], []
    print(f'\n=== {eb} dB (seed 600) ===')
    for a in G:
        d = np.load(f'results/e07_eb{eb}_alpha{a}_s600.npz'); f = d['conv'] > 50; n = len(f); k = f.sum()
        fer.append(k / n); ci.append(wilson(k, n)); ft.append(np.median(d['ftle'][f]))
        cls = np.bincount(d['endclass'][f], minlength=3) / max(k, 1); fixed.append(cls[0]); osc.append(cls[2])
        print(f'  alpha={a:4.2f} FER={k/n:.4f} [{ci[-1][0]:.4f},{ci[-1][1]:.4f}] fails={k:5d}  FTLE med={ft[-1]:+.3f}  fixed={cls[0]*100:5.1f}%  osc={cls[2]*100:5.1f}%')
    la = np.log(G); s = np.sign(ft)
    idx = [i for i in range(len(G) - 1) if s[i] < 0 <= s[i + 1]]
    if idx:
        i = idx[0]; ac = float(np.exp(la[i] + (0 - ft[i]) * (la[i + 1] - la[i]) / (ft[i + 1] - ft[i])))
    else:
        ac = float('nan')
    p6 = 0.8 <= ac <= 1.25
    imin = int(np.argmin(fer)); i1 = G.index(1.0)
    s7 = imin == i1 or ci[imin][1] >= fer[i1] >= ci[imin][0] or ci[i1][0] <= fer[imin]
    r_ft = spearmanr(G, ft).statistic; r_fx = spearmanr(G, fixed).statistic
    print(f'  P6: alpha_c = {ac:.3f} -> {"inside" if p6 else "OUTSIDE"} [0.8, 1.25]')
    print(f'  S7: lowest FER at alpha={G[imin]} ({fer[imin]:.4f}); FER(alpha=1)={fer[i1]:.4f} -> {"PASS" if s7 else "FAIL"}')
    print(f'  S8: Spearman(alpha, FTLE)={r_ft:+.3f}, Spearman(alpha, fixed share)={r_fx:+.3f} -> {"PASS" if (r_ft > 0 and r_fx < 0) else "FAIL"}')
    ok_all &= p6
print(f'\nP6 overall: {"PASSES" if ok_all else "FAILS"}')
