"""PREREG38d: replicate concept-only event-based imagination (38c) with three seed pairs per world."""
import sys, os, json, numpy as np
from multiprocessing import Pool
os.environ['EVENTS'] = '1'; os.environ['NODIR'] = '1'
sys.path.insert(0, 'experiments')
import e52_imagination as E
if __name__ == '__main__':
    jobs = [(nm, wk, h, 27000 + off + 10 * i + j) for off in (300, 400, 500) for i, (wk, h) in enumerate((('hardland', 72), ('cold_new', 48))) for j, nm in enumerate(('plain', 'imagine'))]
    with Pool(2) as pool: out = pool.map(E.run, jobs)
    res = {}
    for (nm, wk, h, seed), (_, _, Ecurve, cold, _) in zip(jobs, out):
        late = float(np.mean(Ecurve[48 * 6:]) if wk == 'hardland' else np.mean(Ecurve[24 * 6:]))
        res.setdefault(wk, {}).setdefault((seed - 27000) // 100, {})[nm] = late
    for wk, pairs in res.items():
        d = [p['imagine'] - p['plain'] for p in pairs.values()]
        print(f"{wk}: pairs " + ', '.join(f"plain {p['plain']:.3f} / imagine {p['imagine']:.3f}" for p in pairs.values())
              + f" | differences {np.round(d, 3).tolist()} mean {np.mean(d):+.3f} -> {'PASS' if np.mean(d) >= .03 and sum(x > 0 for x in d) >= 2 else 'FAIL'}")
    json.dump({wk: {str(k): v for k, v in p.items()} for wk, p in res.items()}, open('results/e54_imagination_rep.json', 'w'))
