"""PREREG42: organic v8 evolution with steering genes; three 24 h lives, fitness = minimum."""
import sys, json, time, warnings, numpy as np
from multiprocessing import Pool
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
import os
os.environ['LETA_LO'] = '-3'; os.environ['LETA_HI'] = '0'
LIFE = 24 * 3600; SEED = 31000
def life(args):
    G, kind, side, lseed, sseed = args
    import os, sys; os.environ['LETA_LO'] = '-3'; os.environ['LETA_HI'] = '0'; sys.path.insert(0, 'src')
    from steer import DirBody
    from evolve3 import OrganicSteerBrain
    from lifeworld import RandomHardWorld, RandomRichWorld
    n = len(G); lr = np.random.default_rng(lseed)
    world = RandomHardWorld(n, lr, side=side) if kind == 'hard' else RandomRichWorld(n, lr)
    rng = np.random.default_rng(sseed); body = DirBody(n, rng); body.R = 50.0; brain = OrganicSteerBrain(n, rng, G)
    fit = np.zeros(n); k = 0
    for t in range(LIFE):
        y, b = body.sense(world); p, _ = brain.step(y); body.steer = brain.steer; body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: fit += body.E; k += 1
    return fit / k
if __name__ == '__main__':
    from evolve3 import random_genomes, mutate, NAMES
    GENS = int(sys.argv[1]) if len(sys.argv) > 1 else 10
    rng = np.random.default_rng(SEED); POP = 100; G = random_genomes(POP, rng); hist = []
    with Pool(2) as pool:
        for gen in range(GENS):
            t0 = time.time(); jobs = []
            for w, (kind, side) in enumerate((('hard', -1), ('hard', 1), ('rich', None))):
                ls = SEED + 100 * gen + w
                jobs += [(G[:50], kind, side, ls, SEED + 500 + 10 * gen + 2 * w), (G[50:], kind, side, ls, SEED + 501 + 10 * gen + 2 * w)]
            res = pool.map(life, jobs)
            per_world = np.array([np.concatenate([res[2 * w], res[2 * w + 1]]) for w in range(3)])
            fit = per_world.min(0); order = np.argsort(-fit); elite = G[order[:20]]
            ev = np.median(elite[:, -3:], 0)
            hist.append(dict(gen=gen, elite_mean=float(fit[order[:20]].mean()), pop_mean=float(fit.mean()), elite_by_world=per_world[:, order[:20]].mean(1).tolist(),
                             elite=elite.tolist(), all_genomes=G.tolist(), all_fitness=fit.tolist(), all_by_world=per_world.tolist()))
            print(f"gen {gen:2d}: pop {fit.mean():.3f} elite {fit[order[:20]].mean():.3f} (cold {hist[-1]['elite_by_world'][0]:.3f}, warm {hist[-1]['elite_by_world'][1]:.3f}, "
                  f"rich {hist[-1]['elite_by_world'][2]:.3f}) | k_value {ev[0]:+.2f} k_innate {ev[1]:+.2f} steer-noise 10^{ev[2]:.2f} ({time.time()-t0:.0f}s)", flush=True)
            G = np.vstack([elite] + [mutate(elite, rng) for _ in range(4)])
            json.dump(dict(history=hist, final_genomes=G.tolist(), names=NAMES), open('results/e59_v8.json', 'w'))
