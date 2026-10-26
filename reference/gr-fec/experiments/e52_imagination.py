"""PREREG38: imagination as random walks on the concept graph vs the same brain without imagination."""
import sys, os, json, warnings, numpy as np
EV = os.environ.get('EVENTS') == '1'; ND = os.environ.get('NODIR') == '1'; OFFS = int(os.environ.get('OFFS', '0')); OUTJ = os.environ.get('OUTJ', 'results/e52_imagination.json')
from multiprocessing import Pool
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
def run(args):
    name, world_kind, hours, seed = args
    import sys; sys.path.insert(0, 'src')
    from embodied import Body
    from imagine import ImaginingBrain, MX
    from lifeworld import HardWorld, RandomHardWorld
    n = 100; rng = np.random.default_rng(seed); body = Body(n, rng); body.R = 50.0
    world = HardWorld(n) if world_kind == 'hardland' else RandomHardWorld(n, np.random.default_rng(23000), side=-1)
    br = ImaginingBrain(n, rng, imagine=(name == 'imagine'), events=EV, nodir=ND, s0=0.01, integ=10, separate=True, trust_k=2.0)
    E = []; cold = []
    for t in range(hours * 3600):
        y, b = body.sense(world); p, _ = br.step(y); body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: E.append(float(body.E.mean())); cold.append(float((body.xy[:, 0] < -25).mean()))
    sample = []
    if name == 'imagine' and br.last_walks is not None:
        for a in (0, 1):
            w = br.last_walks[a][0, 0]
            lab = [f"{br.proto[0, 3, s // 3]:.1f}°C{'↓·↑'[s % 3]}" for s in w]
            sample.append(('keep going' if a == 0 else 'turn') + ': ' + ' → '.join(lab))
    return name, world_kind, E, cold, sample
if __name__ == '__main__':
    jobs = [(nm, wk, h, 27000 + OFFS + 10 * i + j) for i, (wk, h) in enumerate((('hardland', 72), ('cold_new', 48))) for j, nm in enumerate(('plain', 'imagine'))]
    with Pool(2) as pool: out = pool.map(run, jobs)
    res = {}
    for nm, wk, E, cold, sample in out:
        late = np.mean(E[48 * 6:]) if wk == 'hardland' else np.mean(E[24 * 6:])
        res.setdefault(wk, {})[nm] = dict(E=E, cold=cold, late=float(late), sample=sample)
        print(f'{wk:9s} {nm:8s}: late energy {late:.3f}; cold-band share at end {cold[-1]:.2f}', flush=True)
        for s in sample: print('    imagined walk —', s)
    json.dump(res, open(OUTJ, 'w'))
    a = res['hardland']['imagine']['late'] - res['hardland']['plain']['late']; b = res['cold_new']['imagine']['late'] - res['cold_new']['plain']['late']
    print(f"P38a (Hardland, ≥ +0.05): {a:+.3f} -> {'PASS' if a >= .05 else 'FAIL'}"); print(f"P38b (new cold band, ≥ +0.05): {b:+.3f} -> {'PASS' if b >= .05 else 'FAIL'}")
