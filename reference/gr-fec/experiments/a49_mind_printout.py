"""Print an evolved organic (v7) worm's mind as a labelled hypergraph after living in a world.
Usage: python3 a49_mind_printout.py [elite_index] [hard|rich] [side] [hours]"""
import sys, json, warnings, numpy as np
warnings.filterwarnings('ignore'); sys.path.insert(0, 'src')
from embodied import Body
from evolve2 import OrganicBrain, CH, C, MXC, per, glob, NAMES
from lifeworld import RandomHardWorld, RandomRichWorld
i = int(sys.argv[1]) if len(sys.argv) > 1 else 0; kind = sys.argv[2] if len(sys.argv) > 2 else 'hard'
side = int(sys.argv[3]) if len(sys.argv) > 3 else -1; hours = int(sys.argv[4]) if len(sys.argv) > 4 else 24
G = np.array(json.load(open('results/e47_organic.json'))['final_genomes'])[i:i + 1]
rng = np.random.default_rng(24000); lr = np.random.default_rng(24001)
world = RandomHardWorld(1, lr, side=side) if kind == 'hard' else RandomRichWorld(1, lr)
body = Body(1, rng); body.R = 50.0; br = OrganicBrain(1, rng, G)
for t in range(hours * 3600):
    y, b = body.sense(world); p, _ = br.step(y); body.act(p); body.metabolise(b); world.consume(body.intake)
def label(c, v): return f'{CH[c]} ≈ {v * 5 + 20:.1f} °C' if CH[c] == 'temp' else f'{CH[c]} ≈ {v:.2f}'
print(f'=== Mind of elite {i} after {hours} h in a {kind} world (food band side {side:+d}); energy {body.E[0]:.2f} ===')
print('GENOME (what evolution decided):')
for k, nm in (('rew', 'feels good (+) / bad (−)'), ('inst', 'instinct pull'), ('trust', 'trust learning'), ('bias', 'turning bias')):
    v = per(G, k)[0]; top = np.argsort(-np.abs(v))[:4]
    print(f'  {nm:24s}: ' + ', '.join(f'{CH[c]} {v[c]:+.2f}' for c in top))
gates = [(CH[int(G[0, NAMES.index(g)])], glob(G, g + '_on')[0] > 0) for g in ('gA', 'gB')]
print('  hyperedge gates          : ' + ', '.join(f'{c} ({"on" if on else "off"})' for c, on in gates))
print('CONCEPT NODES (grown from experience) and their learned value:')
NS = br.NS
for c in range(C):
    for k in range(br.cnt[0, c]):
        j = c * MXC + k; w0, wA, wB = br.W[0, j], br.W[0, NS + j], br.W[0, 2 * NS + j]
        extra = ''.join(f', gated by {gates[q][0]} {w:+.2f}' for q, w in ((0, wA), (1, wB)) if gates[q][1] and abs(w) > 0.01)
        print(f'  [{label(c, br.proto[0, c, k])}] value {w0:+.2f}{extra}')
print('INTENSITY RELATIONS (learned value of each sense level) and current trust in instinct:')
wl = br.W[0, C * MXC:C * MXC + C]; t = np.clip(1 + per(G, 'trust')[0] * wl, 0, 1.5)
for c in range(C):
    if abs(wl[c]) > 0.01 or abs(per(G, 'inst')[0, c]) > 0.5:
        print(f'  {CH[c]:8s}: learned value {wl[c]:+.2f}; instinct {per(G, "inst")[0, c]:+.2f} × trust {t[c]:.2f}')
