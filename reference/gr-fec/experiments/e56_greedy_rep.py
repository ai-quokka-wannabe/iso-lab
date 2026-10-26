"""PREREG39b: replicate greedy value-steering in the never-seen cold world."""
import sys, json, numpy as np
from multiprocessing import Pool
sys.path.insert(0, 'experiments')
import e55_steer_imagine as E
if __name__ == '__main__':
    jobs = [(m, 'cold_new', 48, base + j) for base in (28100, 28200) for j, m in enumerate(('plain', 'greedy'))]
    with Pool(2) as pool: out = pool.map(E.run, jobs)
    d = []
    for k in range(2):
        p, g = out[2 * k][2], out[2 * k + 1][2]; d.append(g - p); print(f'pair {k}: plain {p:.3f}  greedy {g:.3f}  diff {g - p:+.3f}', flush=True)
    print(f"P39c: {'PASS' if all(x >= .10 for x in d) else 'FAIL'}")
    json.dump(dict(diffs=d, raw=[(o[0], o[2]) for o in out]), open('results/e56_greedy_rep.json', 'w'))
