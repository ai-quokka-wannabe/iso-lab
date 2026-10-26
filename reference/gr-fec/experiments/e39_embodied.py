"""PREREG28: embodied worm v4 — all assays with realistic lab procedures. The brain only ever receives y."""
import sys, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
from embodied import Worm, World
import os, functools
INTEG = float(os.environ.get('INTEG', '0')); OFF = int(os.environ.get('OFF', '0'))
if INTEG: Worm.__init__ = functools.partialmethod(Worm.__init__, integ=INTEG)
N = 100
def CI(xy, target=(15, 0), control=(-15, 0)):
    return ((np.linalg.norm(xy - target, axis=1) < 8).sum() - (np.linalg.norm(xy - control, axis=1) < 8).sum()) / len(xy)
def prefeed(w, mins=30, T=20.0): w.transfer(); w.run(World(lawn=1.0, temp=T), mins * 60)
def a1(seed, s0, **kw):
    w = Worm(N, seed, s0=s0, **kw); prefeed(w); w.transfer(); p, _ = w.run(World(odour=True), 1200); return CI(w.body.xy), p.mean() * 60
def a2(seed, s0, **kw):
    out = {}
    for k, tc in enumerate((15, 25)):
        w = Worm(N, seed + k, s0=s0, **kw); w.transfer(); w.run(World(lawn=1.0, temp=tc), 7200); w.transfer(); w.run(World(thermal=True), 1200)
        out[tc] = 20 + 5 * w.body.xy[:, 0] / 25
    return out
def a3(seed, s0, **kw):
    out = {}
    for k, su in enumerate((0.0, 0.8)):
        w = Worm(N, seed + k, s0=s0, **kw); prefeed(w); w.transfer(); w.run(World(salt_uniform=su), 3600); w.transfer(); w.run(World(salt=True), 1200)
        out['salt' if su else 'mock'] = CI(w.body.xy)
    return out
def a4(seed, s0, **kw):
    w = Worm(N, seed, s0=s0, **kw); prefeed(w); w.transfer()
    _, r = w.run(World(), 300, taps=set(range(0, 300, 10))); r = r[~np.isnan(r)]
    w.run(World(), 600); _, r2 = w.run(World(), 1, taps={0}); return r[0], r[-1], r2[0]
def a5(seed, s0, **kw):
    w = Worm(N, seed, s0=s0, **kw); prefeed(w); w.transfer(); p, _ = w.run(World(), 1200); p *= 60; return p[:300].mean(), p[900:].mean()
if __name__ == '__main__' and 'gating' not in sys.argv:
    best, sc = None, -1
    for s0 in ((0.003, 0.01, 0.03) if INTEG else (0.01, 0.03, 0.1)):
        ci = a1((13000 + OFF), s0)[0]; e, l = a5((13001 + OFF), s0); m = min(ci / .3, (e / max(l, 1e-9)) / 2)
        print(f'calib s0={s0}: CI {ci:+.2f}, foraging ratio {e/max(l,1e-9):.2f}, score {m:.2f}', flush=True)
        if m > sc: best, sc = s0, m
    s0 = best; print(f'-> s0 = {s0}\n'); R = []
    ci, pr = a1((13101 + OFF), s0); R.append(ci >= .3); ci_rw, _ = a1((13102 + OFF), s0, random_walk_p=pr / 60)
    print(f'A1 odour chemotaxis: CI {ci:+.2f} (pass {ci >= .3}); random walk {ci_rw:+.2f}; pirouettes {pr:.2f}/min', flush=True)
    t = a2((13110 + OFF), s0); d = t[25].mean() - t[15].mean(); R.append(d >= 3)
    print(f'A2 thermotaxis memory: Tc=15 -> {t[15].mean():.2f}, Tc=25 -> {t[25].mean():.2f}, diff {d:+.2f} °C (pass {d >= 3})', flush=True)
    m = a3((13120 + OFF), s0); p3 = m['mock'] >= .15 and m['salt'] <= m['mock'] - .3; R.append(p3)
    print(f"A3 salt learning: mock {m['mock']:+.2f}, salt-starved {m['salt']:+.2f} (pass {p3})", flush=True)
    f, l, rec = a4((13130 + OFF), s0); p4 = l <= .5 * f and rec >= .8 * f; R.append(p4); print(f'A4 habituation: {f:.2f} -> {l:.2f} -> {rec:.2f} (pass {p4})')
    e, la = a5((13140 + OFF), s0); R.append(e >= 2 * la); print(f'A5 foraging: {e:.2f} vs {la:.2f} /min, ratio {e/max(la,1e-9):.2f} (pass {e >= 2*la})')
    print(f'P28a: {sum(R)}/5 -> {"PASS" if sum(R) >= 4 else "FAIL"}', flush=True)
    t0 = a2((13150 + OFF), s0, eta=0.0); m0 = a3((13160 + OFF), s0, eta=0.0)
    print(f"Control no-learning: A2 diff {t0[25].mean()-t0[15].mean():+.2f}; A3 mock {m0['mock']:+.2f} salt {m0['salt']:+.2f}")
    tr = a2((13170 + OFF), s0, random_walk_p=pr / 60); mr = a3((13180 + OFF), s0, random_walk_p=pr / 60); er, lr = a5((13190 + OFF), s0, random_walk_p=pr / 60)
    print(f"Control random walk: A1 {ci_rw:+.2f}; A2 diff {tr[25].mean()-tr[15].mean():+.2f}; A3 mock {mr['mock']:+.2f} salt {mr['salt']:+.2f}; A5 ratio {er/lr:.2f}", flush=True)
    res = {}
    for k, fed in enumerate((True, False)):
        w = Worm(N, (13200 + OFF) + k, s0=s0); prefeed(w); w.transfer(); w.run(World(lawn=1.0 if fed else 0.0, temp=20.0), 7200)
        w.transfer(); w.run(World(thermal=True), 1200); res[fed] = np.abs(20 + 5 * w.body.xy[:, 0] / 25 - 20).mean()
    print(f'Mohri: |T-20| fed {res[True]:.2f}, starved {res[False]:.2f}', flush=True)
if __name__ == '__main__':
    s0 = (0.03 if not INTEG else float(os.environ.get('S0', '0.01'))) if 'gating' in sys.argv else s0
    G = {}
    for hyper in (True, False):
        w = Worm(2 * N, (13300 + OFF) + hyper, s0=s0, hyper=hyper); prefeed(w)
        for rnd in range(6):
            d = w.body.rng.integers(0, 151, 2 * N) * 60; ts = 9000
            for t in range(ts):
                on_lawn = (t < ts - 600 - d) | (t >= ts - 600)
                scented = t >= ts - 600
                y_world = World(lawn=on_lawn.astype(float), N_uniform=np.where(scented, 0.8, 0.0))
                w.run(y_world, 1)
        grp = np.arange(2 * N) < N
        fed_state = {}
        for t in range(7200):                       # fed group on a lawn for the last 30 min; starved group 2 h without food
            lawn = np.where(grp, float(t >= 7200 - 1800), 0.0); w.run(World(lawn=lawn), 1)
        fed_state = (1 - w.body.E[grp].mean(), 1 - w.body.E[~grp].mean())
        w.transfer(); w.run(World(N=True), 600)
        G[hyper] = (CI(w.body.xy[grp]), CI(w.body.xy[~grp]))
        print(f"gating {'hyperedges' if hyper else 'pairwise  '}: CI fed {G[hyper][0]:+.2f}, starved {G[hyper][1]:+.2f}, diff {G[hyper][1]-G[hyper][0]:+.2f}  "
              f"(true hunger at test, for the reader only: fed {fed_state[0]:.2f}, starved {fed_state[1]:.2f})", flush=True)
    dh = G[True][1] - G[True][0]; dp = G[False][1] - G[False][0]
    print(f'P28b: {"PASS" if dh >= .15 and dh - dp >= .1 else "FAIL"}')
