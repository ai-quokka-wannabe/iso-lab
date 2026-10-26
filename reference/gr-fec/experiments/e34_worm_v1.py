"""PREREG23: worm agent v1 (rectified senses, two-assay calibration). Reuses the e33 assays with fresh seeds."""
import sys, functools, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
import worm
worm.Worms.__init__ = functools.partialmethod(worm.Worms.__init__, rectify=True)
import e33_worm as E
E.Worms = worm.Worms
best, sc = None, -1
for s0 in (0.003, 0.01, 0.03, 0.1):
    ci = E.a1(4000, s0)[0]; e, l = E.a5(4001, s0=s0); m = min(ci / .3, (e / l) / 2)
    print(f'calib s0={s0}: CI {ci:+.2f}, foraging ratio {e/l:.2f}, score {m:.2f}')
    if m > sc: best, sc = s0, m
s0 = best; print(f'-> s0 = {s0}\n')
ci, pr = E.a1(4101, s0); ci_rw, _ = E.a1(4102, s0, rw=pr)
print(f'A1 odour chemotaxis: CI={ci:+.2f} (pass {ci >= .3}); random walk {ci_rw:+.2f} (pass {abs(ci_rw) < .15}); pirouettes {pr*60:.2f}/min')
t = E.a2(4110, s0=s0); d = t[25].mean() - t[15].mean()
print(f'A2 thermotaxis memory: Tc=15 -> {t[15].mean():.2f}, Tc=25 -> {t[25].mean():.2f}, diff {d:+.2f} °C (pass {d >= 3})')
m = E.a3(4120, s0=s0); print(f"A3 salt learning: mock {m['mock']:+.2f}, salt-starved {m['salt']:+.2f} (pass {m['mock'] >= .15 and m['salt'] <= m['mock'] - .3})")
f, l, rec = E.a4(4130, s0=s0); print(f'A4 habituation: {f:.2f} -> {l:.2f} -> rest {rec:.2f} (pass {l <= .5*f and rec >= .8*f})')
e, la = E.a5(4140, s0=s0); print(f'A5 foraging: {e:.2f} vs {la:.2f} per min, ratio {e/la:.2f} (pass {e >= 2*la})')
t0 = E.a2(4150, eta=0.0, s0=s0); m0 = E.a3(4160, eta=0.0, s0=s0)
print(f"A6 no learning: A2 diff {t0[25].mean()-t0[15].mean():+.2f}; A3 mock {m0['mock']:+.2f} salt {m0['salt']:+.2f}")
tr = E.a2(4170, rw=pr, s0=s0); mr = E.a3(4180, rw=pr, s0=s0); er, lr = E.a5(4190, s0=s0, rw=pr)
print(f"A7 random walker: A1 {ci_rw:+.2f}; A2 diff {tr[25].mean()-tr[15].mean():+.2f}; A3 mock {mr['mock']:+.2f} salt {mr['salt']:+.2f}; A5 ratio {er/lr:.2f}")
M = {}
for k, tau in enumerate((0.03, 0.1, 0.3, 1, 3, 10)):
    c = [E.a1(4200 + 10 * k + i, s0, tau=tau)[0] for i in range(8)]; M[tau] = (np.mean(c), np.std(c) / np.sqrt(8))
b = max(M, key=lambda k: M[k][0]); ok = b not in (0.03, 10) and all(M[b][0] - M[e][0] > 2 * np.hypot(M[b][1], M[e][1]) for e in (0.03, 10))
print('A8: ' + '  '.join(f'tau={k}: {v[0]:+.3f}±{v[1]:.3f}' for k, v in M.items()) + f' -> best {b}, pass {ok}')
tn = E.a2(4310, s0=s0, food=False, tcs=(20,)); tf = E.a2(4311, s0=s0, food=True, tcs=(20,))
print(f'Exploratory Mohri: |T-20| fed {np.abs(tf[20]-20).mean():.2f}, starved {np.abs(tn[20]-20).mean():.2f}')
w = worm.Worms(100, np.random.default_rng(4320), s0=s0); E.cultivate(w, temp=25)
print('learned thermal edges after 2 h at 25 °C (15..25):', np.round(w.W[:, worm.SL_T].mean(0), 2))
