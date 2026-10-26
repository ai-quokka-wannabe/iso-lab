"""Forensics for PREREG22 failures (exploratory, not pre-registered)."""
import sys, numpy as np
sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
import warnings; warnings.filterwarnings('ignore')
from e33_worm import *
s0 = 0.003
# A2: is the thermotaxis weak because of time, or because the learned thermal edge is narrow?
for T in (1200, 3600):
    out = {}
    for k, tc in enumerate((15, 25)):
        w = Worms(N, np.random.default_rng(3000 + k), s0=s0); cultivate(w, temp=tc)
        if k == 1: prof = w.W[:, SL_T].mean(0)
        out[tc] = thermotaxis(w, T=T)
    print(f'A2 {T//60} min: Tc=15 -> {out[15].mean():.2f} (share <17.5: {(out[15]<17.5).mean():.2f}), Tc=25 -> {out[25].mean():.2f} (share >22.5: {(out[25]>22.5).mean():.2f})')
print('   learned thermal edges after cultivation at 25 °C (bins 15..25):', np.round(prof, 2))
# A5: is the search signal drowned by noise-driven turning at s0 = 0.003?
for s in (0.003, 0.01, 0.03):
    e, l = a5(3100, s0=s); print(f'A5 s0={s}: pirouettes/min 0-5 {e:.2f}, 15-20 {l:.2f}, ratio {e/l:.2f}')
# A8: seed-to-seed spread of CI per tau
for tau in (0.03, 0.3, 1, 10):
    c = [a1(3200 + i, s0, tau=tau)[0] for i in range(8)]
    print(f'A8 tau={tau}: CI mean {np.mean(c):+.3f} ± {np.std(c)/np.sqrt(8):.3f} (SE, 8 seeds)')
