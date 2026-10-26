"""PREREG36: organic evolution selecting for generality — three balanced lives per genome per generation."""
import sys, json, time, warnings, numpy as np
from multiprocessing import Pool
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
import os
LIFE = int(os.environ.get('LIFE_H', '12')) * 3600; AGG = os.environ.get('AGG', 'mean'); OUTF = os.environ.get('OUTF', 'results/e49_general.json'); SEED = int(os.environ.get('SEED', '25000'))
def life(args):
    G, kind, side, lseed, sseed = args
    import sys; sys.path.insert(0, 'src')
    from embodied import Body
    from evolve2 import OrganicBrain
    from lifeworld import RandomHardWorld, RandomRichWorld
    n = len(G); lr = np.random.default_rng(lseed)
    world = RandomHardWorld(n, lr, side=side) if kind == 'hard' else RandomRichWorld(n, lr)
    rng = np.random.default_rng(sseed); body = Body(n, rng); body.R = 50.0; brain = OrganicBrain(n, rng, G)
    fit = np.zeros(n); k = 0
    for t in range(LIFE):
        y, b = body.sense(world); p, _ = brain.step(y); body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: fit += body.E; k += 1
    return fit / k
if __name__ == '__main__':
    from evolve2 import random_genomes, mutate, NAMES
    GENS = int(sys.argv[1]) if len(sys.argv) > 1 else 14
    rng = np.random.default_rng(SEED); POP = 100; G = random_genomes(POP, rng); hist = []
    with Pool(2) as pool:
        for gen in range(GENS):
            t0 = time.time(); jobs = []
            for w, (kind, side) in enumerate((('hard', -1), ('hard', 1), ('rich', None))):
                ls = SEED + 100 * gen + w
                jobs += [(G[:50], kind, side, ls, SEED + 500 + 10 * gen + 2 * w), (G[50:], kind, side, ls, SEED + 501 + 10 * gen + 2 * w)]
            res = pool.map(life, jobs)
            per_world = np.array([np.concatenate([res[2 * w], res[2 * w + 1]]) for w in range(3)])     # (3, POP)
            fit = per_world.min(0) if AGG == 'min' else per_world.mean(0); order = np.argsort(-fit); elite = G[order[:20]]
            hist.append(dict(gen=gen, elite_mean=float(fit[order[:20]].mean()), pop_mean=float(fit.mean()), best=float(fit[order[0]]),
                             elite_by_world=per_world[:, order[:20]].mean(1).tolist(), elite=elite.tolist(),
                             all_genomes=G.tolist(), all_fitness=fit.tolist(), all_by_world=per_world.tolist()))
            print(f"gen {gen:2d}: pop {fit.mean():.3f} elite {fit[order[:20]].mean():.3f} (cold {hist[-1]['elite_by_world'][0]:.3f}, "
                  f"warm {hist[-1]['elite_by_world'][1]:.3f}, rich {hist[-1]['elite_by_world'][2]:.3f}) best {fit[order[0]]:.3f} ({time.time()-t0:.0f}s)", flush=True)
            G = np.vstack([elite] + [mutate(elite, rng) for _ in range(4)])
            json.dump(dict(history=hist, final_genomes=G.tolist(), names=NAMES), open(OUTF, 'w'))
