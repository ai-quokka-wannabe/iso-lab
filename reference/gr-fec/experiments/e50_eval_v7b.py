"""PREREG36 test: v7b elites in the never-seen worlds of PREREG35 (same layouts/seeds); compared with stored e48 results."""
import sys, json, warnings, numpy as np
from multiprocessing import Pool
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
import os
N = 50; HOURS = 48; SRC = os.environ.get('SRC', 'results/e49_general.json'); TAG = os.environ.get('TAG', 'v7b'); BASE = os.environ.get('BASE', 'results/e48_organic_eval.json'); OUT = os.environ.get('OUT', 'results/e50_eval_all.json')
LAYOUTS = [('hard_cold', 'hard', -1, 23000), ('hard_warm', 'hard', +1, 23001), ('rich_new', 'rich', None, 23002)]
def run(args):
    lname, kind, side, lseed = args
    import sys; sys.path.insert(0, 'src')
    from embodied import Body
    from evolve2 import OrganicBrain
    from lifeworld import RandomHardWorld, RandomRichWorld
    lr = np.random.default_rng(lseed); world = RandomHardWorld(N, lr, side=side) if kind == 'hard' else RandomRichWorld(N, lr)
    rng = np.random.default_rng(lseed + 777); body = Body(N, rng); body.R = 50.0
    G = np.array(json.load(open(SRC))['final_genomes'])[:10]; brain = OrganicBrain(N, rng, np.repeat(G, 5, axis=0))
    E = []
    for t in range(HOURS * 3600):
        y, b = body.sense(world); p, _ = brain.step(y); body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: E.append(float(body.E.mean()))
    return lname, E
if __name__ == '__main__':
    with Pool(2) as pool: out = pool.map(run, LAYOUTS)
    old = json.load(open(BASE))
    for ln, E in out: old[ln][TAG] = dict(curve=E, late=float(np.mean(E[24 * 6:])))
    json.dump(old, open(OUT, 'w'))
    for ln in old: print(ln + ': ' + '  '.join(f"{b} {v['late']:.3f}" for b, v in old[ln].items()))
    v = {ln: old[ln][TAG]['late'] for ln in old}
    print(f"P36a (cold >= 0.70): {'PASS' if v['hard_cold'] >= .70 else 'FAIL'} ({v['hard_cold']:.3f})")
    print(f"P36d (>= e45 - 0.05 everywhere): {'PASS' if all(v[l] >= old[l]['e45']['late'] - .05 for l in old) else 'FAIL'}")
