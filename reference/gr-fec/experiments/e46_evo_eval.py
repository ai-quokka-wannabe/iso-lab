"""PREREG34 follow-up: fair evaluation of the evolved elite genomes vs hand-built v6 (same protocol as PREREG33 (i))."""
import sys, json, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
from embodied import Worm
from evolve import EvoBrain, decode, NAMES
from lifeworld import HardWorld, LifeWorld
d = json.load(open('results/e45_evolve.json')); G = np.array(d['final_genomes'])[:20]; G = np.repeat(G, 5, axis=0)
def live(WC, hours, seed):
    w = Worm(100, seed); w.brain = EvoBrain(100, w.body.rng, G); w.body.R = 50.0; world = WC(100); E = []; odl = []
    for t in range(hours * 3600):
        y, b = w.body.sense(world); p, _ = w.brain.step(y); w.body.act(p); w.body.metabolise(b); world.consume(w.body.intake)
        if t % 600 == 0: E.append(w.body.E.mean()); odl.append(((world.which >= 0) & (world.which < 4)).mean() if WC is HardWorld else 0)
    return np.array(E), np.array(odl), w.body.xy
Eh, Oh, xy = live(HardWorld, 72, 21000)
print(f'evolved elites, Hardland 72 h: energy h48-72 {Eh[48*6:].mean():.3f}; odourless share h48-72 {Oh[48*6:].mean():.3f}; cold-band share at end {(xy[:,0] < -25).mean():.2f}')
print('   (hand-built v6 trust in the same protocol, PREREG33: 0.682, 0.158; v5 learner 0.426; random 0.387; instinct 0.292)')
Er, _, _ = live(LifeWorld, 48, 21001)
print(f'evolved elites, Agarland 48 h: energy h24-48 {Er[24*6:].mean():.3f} (hand-built v6 trust+curiosity 0.854, instinct-only 0.858)')
med = {n: float(np.median(v)) for n, v in decode(np.array(d['final_genomes'])[:20]).items()}
print('final elite median genes:', {k: round(v, 4) for k, v in med.items()})
json.dump(dict(hard_energy=Eh.tolist(), hard_odourless=Oh.tolist(), rich_energy=Er.tolist(), elite_median=med), open('results/e46_evo_eval.json', 'w'))
