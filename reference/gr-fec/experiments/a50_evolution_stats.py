"""Statistics of evolution itself (Matej: optimise evolution, not just the worm).
Works on e47 (elites per generation; full populations if logged) and e45 (elite medians)."""
import json, sys, numpy as np
sys.path.insert(0, 'src')
from evolve2 import NAMES, LO, HI, DISCRETE
d = json.load(open('results/e47_organic.json')); H = d['history']
cont = [i for i in range(len(NAMES)) if i not in DISCRETE]
span = (HI - LO)[cont]
print('=== Organic evolution (PREREG35): fitness trajectory ===')
for h in H: print(f"gen {h['gen']:2d} {h['kind']:4s} side {h['side']:+d}: pop {h['pop_mean']:.3f} elite {h['elite_mean']:.3f} best {h['best']:.3f}  "
                  f"selection differential {h['elite_mean'] - h['pop_mean']:+.3f}")
hard = [h for h in H if h['kind'] == 'hard']; rich = [h for h in H if h['kind'] == 'rich']
def conv(hs, key):
    v = np.array([h[key] for h in hs]); thr = v[0] + 0.9 * (v.max() - v[0]); k = int(np.argmax(v >= thr)); return hs[k]['gen'], v
for name, hs in (('harsh', hard), ('rich', rich)):
    g, v = conv(hs, 'elite_mean'); print(f'{name}: elite mean {v[0]:.3f} → {v[-1]:.3f}; 90 % of the gain reached by generation {g}')
E = [np.array(h['elite'])[:, cont] for h in H]
div = np.array([e.std(0) / span for e in E])                       # gene diversity among elites, relative to range
print('\n=== Gene diversity among elites (std / range); low = converged under selection, high = drifting ===')
first, last = div[0], div[-1]
order = np.argsort(last)
print('most converged genes :', ', '.join(f'{NAMES[cont[i]]} {last[i]:.2f} (gen0 {first[i]:.2f})' for i in order[:10]))
print('still drifting genes :', ', '.join(f'{NAMES[cont[i]]} {last[i]:.2f}' for i in order[-10:]))
med = np.array([np.median(e, 0) for e in E])
moved = np.abs(med[-1] - med[0]) / span
print('\n=== Biggest directional moves of elite medians (fraction of range) ===')
for i in np.argsort(-moved)[:12]: print(f'  {NAMES[cont[i]]:16s} {med[0, i]:+7.3f} → {med[-1, i]:+7.3f}  ({moved[i]:.2f} of range)')
hm = np.array([np.median(np.array(h['elite'])[:, cont], 0) for h in hard]); rm = np.array([np.median(np.array(h['elite'])[:, cont], 0) for h in rich])
print('\n=== Does world type pull genes differently? mean |hard − rich| elite median over paired generations (fraction of range) ===')
k = min(len(hm), len(rm)); diff = np.abs(hm[:k] - rm[:k]).mean(0) / span
for i in np.argsort(-diff)[:8]: print(f'  {NAMES[cont[i]]:16s} {diff[i]:.2f}')
gA = [np.bincount(np.array(h['elite'])[:, NAMES.index('gA')].astype(int), minlength=9) for h in H]
print('\nwasted search estimate: share of genes still above half their initial diversity at the end:',
      f'{(last > 0.5 * first).mean():.2f}')
if 'all_fitness' in H[-1]:
    print('(full populations logged: gene–fitness correlations available)')
json.dump(dict(diversity=div.tolist(), medians=med.tolist(), names=[NAMES[i] for i in cont]), open('results/a50_evolution_stats.json', 'w'))
