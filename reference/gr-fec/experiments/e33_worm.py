"""PREREG22: worm-level hypergraph agent vs classic C. elegans assays. Usage: python3 e33_worm.py [calib|main]"""
import sys, json, numpy as np
sys.path.insert(0, 'src')
from worm import Worms, Plate, chemotaxis_index, SL_T, TB
N = 100


def chemotaxis(w, kind='odour', T=1200):
    w.place(np.zeros((w.n, 2))); p = Plate(odour=kind == 'odour', salt=kind == 'salt'); pirs = 0
    for _ in range(T):
        pir, _ = w.step(p); pirs += pir.mean()
    return chemotaxis_index(w.xy), pirs / T


def thermotaxis(w, T=1200):
    w.place(np.zeros((w.n, 2))); p = Plate(thermal=True)
    for _ in range(T): w.step(p)
    return 20 + 5 * w.xy[:, 0] / 25


def cultivate(w, temp=20.0, food=True, secs=7200, salt_uniform=None):
    p = Plate(temp=temp, salt_uniform=salt_uniform)
    for _ in range(secs): w.step(p, on_food=food, move=False)


def a1(seed, s0, tau=0.3, rw=None):
    w = Worms(N, np.random.default_rng(seed), s0=s0, tau=tau, random_walk_p=rw); return chemotaxis(w, 'odour')


def a2(seed, eta=5e-4, rw=None, s0=0.01, food=True, tcs=(15, 25)):
    out = {}
    for k, tc in enumerate(tcs):
        w = Worms(N, np.random.default_rng(seed + k), s0=s0, eta=eta, random_walk_p=rw)
        cultivate(w, temp=tc, food=food); out[tc] = thermotaxis(w)
    return out


def a3(seed, eta=5e-4, rw=None, s0=0.01):
    out = {}
    for k, cond in enumerate(['mock', 'salt']):
        w = Worms(N, np.random.default_rng(seed + k), s0=s0, eta=eta, random_walk_p=rw)
        cultivate(w, food=False, secs=3600, salt_uniform=0.8 if cond == 'salt' else None)
        out[cond] = chemotaxis(w, 'salt')[0]
    return out


def a4(seed, s0=0.01):
    w = Worms(N, np.random.default_rng(seed), s0=s0); p = Plate(); r = []
    for t in range(300):
        _, rev = w.step(p, tap=(t % 10 == 0))
        if t % 10 == 0: r.append(rev.mean())
    for _ in range(600): w.step(p)
    _, rev = w.step(p, tap=True)
    return r[0], r[-1], rev.mean()


def a5(seed, s0=0.01, rw=None):
    w = Worms(N, np.random.default_rng(seed), s0=s0, random_walk_p=rw); cultivate(w, food=True, secs=1800)
    w.place(np.zeros((N, 2))); p = Plate(); pr = []
    for _ in range(1200):
        pir, _ = w.step(p); pr.append(pir.mean())
    pr = np.array(pr) * 60
    return pr[:300].mean(), pr[900:].mean()


if __name__ == '__main__':
    mode = sys.argv[1] if len(sys.argv) > 1 else 'main'
    if mode == 'calib':
        for s0 in (0.003, 0.01, 0.03):
            ci, pr = a1(2200, s0); print(f'calib s0={s0}: CI={ci:+.2f}, pirouettes/min={pr*60:.2f}')
        sys.exit()
    s0 = float(sys.argv[2]); R = {}
    ci, pr = a1(2201, s0); R['A1'] = ci; prw = pr
    ci_rw, _ = a1(2202, s0, rw=prw)
    print(f'A1 odour chemotaxis: CI={ci:+.2f} (pass {ci >= .3}); random walk CI={ci_rw:+.2f} (pass {abs(ci_rw) < .15}); mean pirouette rate {prw*60:.2f}/min')
    t = a2(2210, s0=s0); d = t[25].mean() - t[15].mean()
    print(f'A2 thermotaxis memory: final T  Tc=15: {t[15].mean():.2f}  Tc=25: {t[25].mean():.2f}  diff={d:+.2f} °C (pass {d >= 3})')
    m = a3(2220, s0=s0)
    print(f"A3 salt learning: CI mock={m['mock']:+.2f}, salt-starved={m['salt']:+.2f} (pass {m['mock'] >= .15 and m['salt'] <= m['mock'] - .3})")
    f, l, rec = a4(2230, s0=s0)
    print(f'A4 habituation: tap1 {f:.2f}, tap30 {l:.2f}, after rest {rec:.2f} (pass {l <= .5*f and rec >= .8*f})')
    e, la = a5(2240, s0=s0)
    print(f'A5 foraging: pirouettes/min 0-5 min {e:.2f}, 15-20 min {la:.2f} (pass {e >= 2*la})')
    t0 = a2(2250, eta=0.0, s0=s0); d0 = t0[25].mean() - t0[15].mean(); m0 = a3(2260, eta=0.0, s0=s0)
    print(f"A6 no-learning ablation: A2 diff={d0:+.2f} (fails {d0 < 3}); A3 mock {m0['mock']:+.2f} salt {m0['salt']:+.2f} (fails {not (m0['mock'] >= .15 and m0['salt'] <= m0['mock'] - .3)})")
    tr = a2(2270, rw=prw, s0=s0); dr = tr[25].mean() - tr[15].mean(); mr = a3(2280, rw=prw, s0=s0); er, lr = a5(2290, s0=s0, rw=prw)
    print(f"A7 random walker: A1 CI {ci_rw:+.2f}; A2 diff {dr:+.2f}; A3 mock {mr['mock']:+.2f} salt {mr['salt']:+.2f}; A5 {er:.2f} vs {lr:.2f}")
    cis = {}
    for k, tau in enumerate((0.03, 0.1, 0.3, 1, 3, 10)):
        cis[tau] = a1(2300 + k, s0, tau=tau)[0]
    best = max(cis, key=cis.get)
    print('A8 exploration temperature: ' + '  '.join(f'tau={k}: CI {v:+.2f}' for k, v in cis.items()) + f' -> best tau={best} (interior: {best not in (0.03, 10)})')
    tn = a2(2310, s0=s0, food=False, tcs=(20,)); tf = a2(2311, s0=s0, food=True, tcs=(20,))
    print(f'Exploratory (Mohri): mean |T-20| after cultivation at 20 °C  fed {np.abs(tf[20]-20).mean():.2f}  starved {np.abs(tn[20]-20).mean():.2f}')
