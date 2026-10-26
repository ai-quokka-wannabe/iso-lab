"""PREREG43: equal-budget race — GA (fixed 10 % mutation, full lives) vs OPT2 (self-adaptive mutation + successive halving).
Both use the compiled brain, the same initial population, the same world schedule and fitness = minimum of three lives.
Budget = worm-hours. Usage: python3 e60_opt_race.py GA|OPT2"""
import sys, os, json, time, warnings, numpy as np
from multiprocessing import Pool
warnings.filterwarnings('ignore'); os.environ['LETA_LO'] = '-3'; os.environ['LETA_HI'] = '0'; sys.path.insert(0, 'src')
LIFE = 24 * 3600; CUT = 6 * 3600; SEED = 32000; BUDGET = 6 * 3 * 24 * 100      # worm-hours = 6 GA generations
def life(args):
    G, kind, side, lseed, sseed, halving = args
    import os, sys; os.environ['LETA_LO'] = '-3'; os.environ['LETA_HI'] = '0'; sys.path.insert(0, 'src')
    from steer import DirBody
    from fastbrain import FastOrganicSteerBrain
    from lifeworld import RandomHardWorld, RandomRichWorld
    from opt2 import subset
    n = len(G); lr = np.random.default_rng(lseed)
    world = RandomHardWorld(n, lr, side=side) if kind == 'hard' else RandomRichWorld(n, lr)
    rng = np.random.default_rng(sseed); body = DirBody(n, rng); body.R = 50.0; brain = FastOrganicSteerBrain(n, rng, G)
    alive = np.arange(n); fit = np.zeros(n); k = np.zeros(n); hours = 0.0
    for t in range(LIFE):
        y, b = body.sense(world); p, _ = brain.step(y); body.steer = brain.steer; body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: fit[alive] += body.E; k[alive] += 1
        if halving and t == CUT:
            prov = fit[alive] / k[alive]; keep = np.argsort(-prov)[:max(1, len(alive) // 2)]
            m = len(alive); alive = alive[keep]
            for o in (body, brain, world): subset(o, keep, m)
            brain.hist = np.ascontiguousarray(brain.hist)
        hours += len(alive) / 3600
    return fit / np.maximum(k, 1), hours
if __name__ == '__main__':
    mode = sys.argv[1]; halving = mode == 'OPT2'
    from evolve3 import random_genomes, mutate, NAMES, LO, HI, DISCRETE
    from evolve2 import C
    from opt2 import init_sigma, sa_mutate
    rng = np.random.default_rng(SEED); POP = 100; G = random_genomes(POP, rng); S = init_sigma(G, LO, HI)
    spent = 0.0; gen = 0; hist = []
    with Pool(2) as pool:
        while spent < BUDGET:
            t0 = time.time(); jobs = []
            for w, (kind, side) in enumerate((('hard', -1), ('hard', 1), ('rich', None))):
                ls = SEED + 100 * gen + w
                jobs += [(G[:50], kind, side, ls, SEED + 500 + 10 * gen + 2 * w, halving), (G[50:], kind, side, ls, SEED + 501 + 10 * gen + 2 * w, halving)]
            res = pool.map(life, jobs); spent += sum(r[1] for r in res)
            per_world = np.array([np.concatenate([res[2 * w][0], res[2 * w + 1][0]]) for w in range(3)])
            fit = per_world.min(0); order = np.argsort(-fit); E, ES = G[order[:20]], S[order[:20]]
            hist.append(dict(gen=gen, spent=spent, elite_mean=float(fit[order[:20]].mean()), best=float(fit[order[0]]), elite=E.tolist(),
                             sigma_median=float(np.median(ES / (HI - LO)))))
            print(f"{mode} gen {gen:2d}: worm-hours {spent:7.0f}/{BUDGET} elite {fit[order[:20]].mean():.3f} best {fit[order[0]]:.3f} "
                  f"median σ {np.median(ES / (HI - LO)):.3f} ({time.time()-t0:.0f}s)", flush=True)
            if mode == 'GA': G = np.vstack([E] + [mutate(E, rng) for _ in range(4)]); S = init_sigma(G, LO, HI)
            else:
                kids = [sa_mutate(E, ES, LO, HI, rng, DISCRETE, C) for _ in range(4)]
                G = np.vstack([E] + [k[0] for k in kids]); S = np.vstack([ES] + [k[1] for k in kids])
            gen += 1
            json.dump(dict(history=hist, final_genomes=G.tolist(), names=NAMES), open(f'results/e60_{mode}.json', 'w'))
