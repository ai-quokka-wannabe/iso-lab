"""PREREG27: worm v3 — re-run of all worm experiments plus the hunger-gating (hyperedge) test."""
import sys, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src'); sys.path.insert(0, 'experiments')
import worm
from worm3 import Worm3
import e33_worm as E, os, functools
GO = os.environ.get('GATES') == '1'; SB = int(os.environ.get('SB', '0'))
Worm3.__init__ = functools.partialmethod(Worm3.__init__, gates_only=GO)
E.Worms = Worm3
ns = {}; exec(open('experiments/e35_sensor_noise.py').read().split('KS = [')[0], ns); TrapPlate = ns['TrapPlate']
Plate = worm.Plate
# calibration (PREREG23 rule)
best, sc = None, -1
for s0 in (0.01, 0.03, 0.1):
    ci = E.a1((10000 + SB), s0)[0]; e, l = E.a5((10001 + SB), s0=s0); m = min(ci / .3, (e / l) / 2)
    print(f'calib s0={s0}: CI {ci:+.2f}, foraging ratio {e/l:.2f}, score {m:.2f}')
    if m > sc: best, sc = s0, m
s0 = best; print(f'-> s0 = {s0}\n'); R = []
ci, pr = E.a1((10101 + SB), s0); ci_rw, _ = E.a1((10102 + SB), s0, rw=pr); R.append(ci >= .3)
print(f'A1 odour chemotaxis: CI {ci:+.2f} (pass {ci >= .3}); random walk {ci_rw:+.2f}; pirouettes {pr*60:.2f}/min')
t = E.a2((10110 + SB), s0=s0); d = t[25].mean() - t[15].mean(); R.append(d >= 3)
print(f'A2 thermotaxis memory: Tc=15 -> {t[15].mean():.2f}, Tc=25 -> {t[25].mean():.2f}, diff {d:+.2f} °C (pass {d >= 3})')
m = E.a3((10120 + SB), s0=s0); p3 = m['mock'] >= .15 and m['salt'] <= m['mock'] - .3; R.append(p3)
print(f"A3 salt learning: mock {m['mock']:+.2f}, salt-starved {m['salt']:+.2f} (pass {p3})")
f, l, rec = E.a4((10130 + SB), s0=s0); p4 = l <= .5 * f and rec >= .8 * f; R.append(p4); print(f'A4 habituation: {f:.2f} -> {l:.2f} -> {rec:.2f} (pass {p4})')
e, la = E.a5((10140 + SB), s0=s0); R.append(e >= 2 * la); print(f'A5 foraging: {e:.2f} vs {la:.2f} /min, ratio {e/la:.2f} (pass {e >= 2*la})')
print(f'P27a: {sum(R)}/5 -> {"PASS" if sum(R) >= 4 else "FAIL"}')
t0 = E.a2((10150 + SB), eta=0.0, s0=s0); m0 = E.a3((10160 + SB), eta=0.0, s0=s0)
d0 = t0[25].mean() - t0[15].mean(); p30 = m0['mock'] >= .15 and m0['salt'] <= m0['mock'] - .3
print(f"A6 no learning: A2 diff {d0:+.2f} (fails {d0 < 3}); A3 mock {m0['mock']:+.2f} salt {m0['salt']:+.2f} (fails {not p30})")
tr = E.a2((10170 + SB), rw=pr, s0=s0); mr = E.a3((10180 + SB), rw=pr, s0=s0); er, lr = E.a5((10190 + SB), s0=s0, rw=pr)
print(f"A7 random walker: A1 {ci_rw:+.2f}; A2 diff {tr[25].mean()-tr[15].mean():+.2f}; A3 mock {mr['mock']:+.2f} salt {mr['salt']:+.2f}; A5 ratio {er/lr:.2f}")
M = {}
for kk, tau in enumerate((0.03, 0.1, 0.3, 1, 3, 10)):
    c = [E.a1((10200 + SB) + 10 * kk + i, s0, tau=tau)[0] for i in range(8)]; M[tau] = (np.mean(c), np.std(c) / np.sqrt(8))
print('A8 tau sweep: ' + '  '.join(f'{k}: {v[0]:+.3f}±{v[1]:.3f}' for k, v in M.items()))
tn = E.a2((10310 + SB), s0=s0, food=False, tcs=(20,)); tf = E.a2((10311 + SB), s0=s0, food=True, tcs=(20,))
print(f'Mohri: |T-20| fed {np.abs(tf[20]-20).mean():.2f}, starved {np.abs(tn[20]-20).mean():.2f}')
SKIP = GO
# senses-only randomness
ok = True
for name, P in ((() if SKIP else (('W1', Plate(odour=True)), ('W2', TrapPlate(odour=True))))):
    res = {}
    for kk in (0, 0.1, 0.3, 1, 3, 10, 30):
        cis = []
        for sd in range(4):
            w = Worm3(100, np.random.default_rng((10400 + SB) + int(kk * 10) * 10 + sd + (5000 if name == 'W2' else 0)), s0=s0, k=kk, det=True)
            w.place(np.zeros((100, 2)))
            for _ in range(1200): w.step(P)
            nt = (np.linalg.norm(w.xy - [15, 0], axis=1) < 8).sum(); nc = (np.linalg.norm(w.xy - [-10, 0], axis=1) < 8).sum(); cis.append((nt - nc) / 100)
        res[kk] = np.mean(cis)
    b = max(res, key=res.get); ok &= res[0] < res[b] - .1 and b not in (0, 30)
    print(f'senses-only {name}: ' + '  '.join(f'k={k}: {v:+.2f}' for k, v in res.items()) + f' -> best k={b}')
print('P27b: skipped' if SKIP else f'P27b: {"PASS" if ok else "FAIL"}')
# hunger gating
out = {}
for hyper in (True, False):
    for hv in (0.1, 0.9):
        w = Worm3(100, np.random.default_rng((10600 + SB) + 2 * hyper + (hv > .5)), s0=s0, hyper=hyper)
        for sess in range(20):
            w.set_state(hunger=w.rng.uniform(0, 1))
            for _ in range(300): w.step(Plate(salt_uniform=0.8), on_food=True, move=False)
            for _ in range(300): w.step(Plate(), on_food=False, move=False)
        w.set_state(hunger=hv, gut=0.0)
        if GO: w.eta = 0.0
        out[(hyper, hv)] = E.chemotaxis(w, 'salt')[0]
dh = out[(True, .9)] - out[(True, .1)]; dp = out[(False, .9)] - out[(False, .1)]
print(f'Hunger gating: hyperedges CI h=0.1 {out[(True,.1)]:+.2f}, h=0.9 {out[(True,.9)]:+.2f} (diff {dh:+.2f}); '
      f'pairwise CI h=0.1 {out[(False,.1)]:+.2f}, h=0.9 {out[(False,.9)]:+.2f} (diff {dp:+.2f})')
print(f'P27c: {"PASS" if dh >= .15 and abs(dp) < .1 else "FAIL"}')
