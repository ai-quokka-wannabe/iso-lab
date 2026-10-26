"""PREREG35: evolve organic brains (architecture in the genome) in randomised worlds, 2 worker processes."""
import sys, json, time, warnings, numpy as np
from multiprocessing import Pool
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
LIFE = 24 * 3600
def life(args):
    G, kind, lseed, sseed = args
    import sys; sys.path.insert(0, 'src')
    from embodied import Body
    from evolve2 import OrganicBrain
    from lifeworld import RandomHardWorld, RandomRichWorld
    n = len(G); lr = np.random.default_rng(lseed)
    world = RandomHardWorld(n, lr) if kind == 'hard' else RandomRichWorld(n, lr)
    rng = np.random.default_rng(sseed); body = Body(n, rng); body.R = 50.0; brain = OrganicBrain(n, rng, G)
    fit = np.zeros(n); k = 0
    for t in range(LIFE):
        y, b = body.sense(world); p, _ = brain.step(y); body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: fit += body.E; k += 1
    return fit / k, world.side
if __name__ == '__main__':
    from evolve2 import random_genomes, mutate, NAMES
    GENS = int(sys.argv[1]) if len(sys.argv) > 1 else 30
    rng = np.random.default_rng(22000); POP = 100; G = random_genomes(POP, rng); hist = []
    with Pool(2) as pool:
        for gen in range(GENS):
            t0 = time.time(); kind = 'hard' if gen % 2 == 0 else 'rich'; lseed = 22000 + 100 * gen
            res = pool.map(life, [(G[:50], kind, lseed, 22500 + 2 * gen), (G[50:], kind, lseed, 22501 + 2 * gen)])
            fit = np.concatenate([r[0] for r in res]); side = res[0][1]
            order = np.argsort(-fit); elite = G[order[:20]]
            hist.append(dict(gen=gen, kind=kind, side=int(side), elite_mean=float(fit[order[:20]].mean()), pop_mean=float(fit.mean()),
                             best=float(fit[order[0]]), elite=elite.tolist(), all_genomes=G.tolist(), all_fitness=fit.tolist()))
            print(f"gen {gen:2d} [{kind} side {int(side):+d}] pop {fit.mean():.3f} elite {fit[order[:20]].mean():.3f} best {fit[order[0]]:.3f} ({time.time()-t0:.0f}s)", flush=True)
            G = np.vstack([elite] + [mutate(elite, rng) for _ in range(4)])
            json.dump(dict(history=hist, final_genomes=G.tolist(), names=NAMES), open('results/e47_organic.json', 'w'))
