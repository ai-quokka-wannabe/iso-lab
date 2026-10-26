"""PREREG39: plain vs greedy vs imagination, with a steerable body."""
import sys, os, json, warnings, numpy as np
STABLE = os.environ.get('STABLE') == '1'; REL = os.environ.get('REL') == '1'; SEED0 = int(os.environ.get('SEED0', '28000')); OUTS = os.environ.get('OUTS', 'results/e55_steer.json')
MODES = tuple(os.environ.get('MODES', 'plain,greedy,imagine').split(','))
from multiprocessing import Pool
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
def run(args):
    mode, wk, hours, seed = args
    import sys; sys.path.insert(0, 'src')
    from steer import DirBody, SteeringBrain
    from lifeworld import HardWorld, RandomHardWorld
    n = 100; rng = np.random.default_rng(seed); body = DirBody(n, rng); body.R = 50.0
    world = HardWorld(n) if wk == 'hardland' else RandomHardWorld(n, np.random.default_rng(23000), side=-1)
    br = SteeringBrain(n, rng, mode=mode, stable=STABLE, relational=REL, s0=0.01, integ=10, separate=True, trust_k=2.0); E = []; cold = []
    for t in range(hours * 3600):
        y, b = body.sense(world); p, _ = br.step(y); body.steer = br.steer; body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: E.append(float(body.E.mean())); cold.append(float((body.xy[:, 0] < -25).mean()))
    sample = []
    if mode == 'imagine' and br.last_walks is not None:
        for a, nm in ((0, 'steer right'), (1, 'straight'), (2, 'steer left')):
            w = br.last_walks[a][0, 0]; sample.append(nm + ': ' + ' → '.join(f"{br.proto[0, 3, q // 3]:.1f}°C{'R·L'[q % 3]}" for q in w))
    late = float(np.mean(E[48 * 6:]) if wk == 'hardland' else np.mean(E[24 * 6:]))
    return mode, wk, late, cold[-1], sample, E
if __name__ == '__main__':
    jobs = [(m, wk, h, SEED0 + 10 * i + j) for i, (wk, h) in enumerate((('hardland', 72), ('cold_new', 48))) for j, m in enumerate(MODES)]
    with Pool(2) as pool: out = pool.map(run, jobs)
    res = {}
    for m, wk, late, cold, sample, E in out:
        res.setdefault(wk, {})[m] = dict(late=late, cold=cold, E=E)
        print(f'{wk:9s} {m:8s}: late energy {late:.3f}; cold-band share at end {cold:.2f}', flush=True)
        for s in sample: print('    imagined —', s)
    json.dump(res, open(OUTS, 'w'))
    if 'plain' in MODES:
        for wk in res:
            r = res[wk]; print(f"{wk}: imagine − plain {r['imagine']['late'] - r['plain']['late']:+.3f}; imagine − greedy {r['imagine']['late'] - r['greedy']['late']:+.3f}; greedy − plain {r['greedy']['late'] - r['plain']['late']:+.3f}")
        a = all(res[w]['imagine']['late'] >= res[w]['plain']['late'] + .08 for w in res); b = all(res[w]['imagine']['late'] >= res[w]['greedy']['late'] + .03 for w in res)
        print(f"P39a: {'PASS' if a else 'FAIL'}   P39b: {'PASS' if b else 'FAIL'}")
    else:
        old = json.load(open(os.environ.get('PREV', 'results/e55_steer.json')))
        a = all(res[w]['imagine']['late'] >= old[w]['imagine']['late'] + .05 for w in res)
        print(f"P40a (stable imagine ≥ flickering imagine + 0.05 in both): {'PASS' if a else 'FAIL'} " + ', '.join(f"{w}: {res[w]['imagine']['late']:.3f} vs {old[w]['imagine']['late']:.3f}" for w in res))
        b = res['cold_new']['imagine']['late'] >= res['cold_new']['greedy']['late'] - .03
        print(f"P40b (cold: imagine ≥ greedy − 0.03): {'PASS' if b else 'FAIL'} ({res['cold_new']['imagine']['late']:.3f} vs {res['cold_new']['greedy']['late']:.3f})")
