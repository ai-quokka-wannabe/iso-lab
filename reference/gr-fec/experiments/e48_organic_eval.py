"""PREREG35 test: never-seen worlds. Brains: organic v7 elites, hand-built v6 trust, e45 evolved-parameter elites, random walker."""
import sys, json, warnings, numpy as np
from multiprocessing import Pool
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
N = 50; HOURS = 48
LAYOUTS = [('hard_cold', 'hard', -1, 23000), ('hard_warm', 'hard', +1, 23001), ('rich_new', 'rich', None, 23002)]
def run(args):
    brain_name, lname, kind, side, lseed, rw = args
    import sys; sys.path.insert(0, 'src')
    from embodied import Body, Brain
    from lifeworld import RandomHardWorld, RandomRichWorld
    lr = np.random.default_rng(lseed); world = RandomHardWorld(N, lr, side=side) if kind == 'hard' else RandomRichWorld(N, lr)
    rng = np.random.default_rng(lseed + 500 + hash(brain_name) % 100); body = Body(N, rng); body.R = 50.0
    if brain_name == 'v7':
        from evolve2 import OrganicBrain
        G = np.array(json.load(open('results/e47_organic.json'))['final_genomes'])[:10]; brain = OrganicBrain(N, rng, np.repeat(G, 5, axis=0))
    elif brain_name == 'e45':
        from evolve import EvoBrain
        G = np.array(json.load(open('results/e45_evolve.json'))['final_genomes'])[:10]; brain = EvoBrain(N, rng, np.repeat(G, 5, axis=0))
    elif brain_name == 'v6':
        brain = Brain(N, rng, s0=0.01, integ=10, separate=True, trust_k=2.0)
    else:
        brain = Brain(N, rng, s0=0.01, integ=10, separate=True, random_walk_p=rw)
    E = []; pir = 0.0
    for t in range(HOURS * 3600):
        y, b = body.sense(world); p, _ = brain.step(y); body.act(p); body.metabolise(b); world.consume(body.intake)
        pir += p.mean()
        if t % 600 == 0: E.append(float(body.E.mean()))
    return brain_name, lname, E, pir / (HOURS * 3600)
if __name__ == '__main__':
    jobs = [(b, ln, k, s, ls, None) for ln, k, s, ls in LAYOUTS for b in ('v7', 'v6', 'e45')]
    with Pool(2) as pool:
        out = pool.map(run, jobs)
    rates = [r[3] for r in out if r[0] == 'v6']; rw = float(np.mean(rates))
    with Pool(2) as pool:
        out += pool.map(run, [('random', ln, k, s, ls, rw) for ln, k, s, ls in LAYOUTS])
    res = {}
    for b, ln, E, _ in out: res.setdefault(ln, {})[b] = dict(curve=E, late=float(np.mean(E[24 * 6:])))
    json.dump(res, open('results/e48_organic_eval.json', 'w'))
    for ln in res: print(ln + ': ' + '  '.join(f"{b} {v['late']:.3f}" for b, v in res[ln].items()))
    p35b = res['hard_cold']['v7']['late'] > res['hard_cold']['v6']['late'] and res['hard_warm']['v7']['late'] > res['hard_warm']['v6']['late']
    p35c = res['hard_warm']['v7']['late'] >= res['hard_warm']['e45']['late'] + 0.05
    print(f'P35b (v7 > v6 in both new harsh layouts): {"PASS" if p35b else "FAIL"}')
    print(f'P35c (v7 >= e45 + 0.05 in the warm-band layout): {"PASS" if p35c else "FAIL"}')
