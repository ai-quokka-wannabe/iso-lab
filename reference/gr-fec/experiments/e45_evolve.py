"""PREREG34: evolve the brain's genes by lifetime energy in alternating harsh / rich worlds."""
import sys, json, time, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
from embodied import Worm
from evolve import EvoBrain, random_genomes, mutate, NAMES, decode
from lifeworld import HardWorld, LifeWorld
rng = np.random.default_rng(20000); POP, GENS, LIFE = 100, int(sys.argv[1]) if len(sys.argv) > 1 else 20, 24 * 3600
G = random_genomes(POP, rng); hist = []
for gen in range(GENS):
    t0 = time.time(); WC = HardWorld if gen % 2 == 0 else LifeWorld
    w = Worm(POP, 20000 + 1000 + gen); w.brain = EvoBrain(POP, w.body.rng, G); w.body.R = 50.0; world = WC(POP); fit = np.zeros(POP); k = 0
    for t in range(LIFE):
        y, b = w.body.sense(world); p, _ = w.brain.step(y); w.body.act(p); w.body.metabolise(b); world.consume(w.body.intake)
        if t % 600 == 0: fit += w.body.E; k += 1
    fit /= k; order = np.argsort(-fit); elite = G[order[:20]]
    d = decode(elite); summ = {n: float(np.median(d[n])) for n in NAMES}
    hist.append(dict(gen=gen, world=WC.__name__, best=float(fit[order[0]]), elite_mean=float(fit[order[:20]].mean()), pop_mean=float(fit.mean()), elite_median_genes=summ))
    print(f"gen {gen:2d} [{WC.__name__:9s}] fitness: pop {fit.mean():.3f} elite {fit[order[:20]].mean():.3f} best {fit[order[0]]:.3f} ({time.time()-t0:.0f}s) | "
          + ' '.join(f"{n}={summ[n]:.3g}" for n in NAMES), flush=True)
    G = np.vstack([elite] + [mutate(elite, rng) for _ in range(4)])
    json.dump(dict(history=hist, final_genomes=G.tolist(), names=NAMES), open('results/e45_evolve.json', 'w'), indent=1)
