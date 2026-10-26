"""Never-seen worlds (PREREG35 layouts) for steering-genome brains (v8 / race GA / race OPT2). Usage: e61_eval_steer.py TAG SRC"""
import sys, os, json, warnings, numpy as np
from multiprocessing import Pool
warnings.filterwarnings('ignore'); os.environ['LETA_LO'] = '-3'; os.environ['LETA_HI'] = '0'; sys.path.insert(0, 'src')
N = 50; HOURS = 48
LAYOUTS = [('hard_cold', 'hard', -1, 23000), ('hard_warm', 'hard', +1, 23001), ('rich_new', 'rich', None, 23002)]
def run(args):
    lname, kind, side, lseed, src = args
    import os, sys; os.environ['LETA_LO'] = '-3'; os.environ['LETA_HI'] = '0'; sys.path.insert(0, 'src')
    from steer import DirBody
    from fastbrain import FastOrganicSteerBrain
    from lifeworld import RandomHardWorld, RandomRichWorld
    lr = np.random.default_rng(lseed); world = RandomHardWorld(N, lr, side=side) if kind == 'hard' else RandomRichWorld(N, lr)
    rng = np.random.default_rng(lseed + 999); body = DirBody(N, rng); body.R = 50.0
    G = np.array(json.load(open(src))['final_genomes'])[:10]; brain = FastOrganicSteerBrain(N, rng, np.repeat(G, 5, axis=0)); E = []
    for t in range(HOURS * 3600):
        y, b = body.sense(world); p, _ = brain.step(y); body.steer = brain.steer; body.act(p); body.metabolise(b); world.consume(body.intake)
        if t % 600 == 0: E.append(float(body.E.mean()))
    return lname, float(np.mean(E[24 * 6:]))
if __name__ == '__main__':
    tag, src = sys.argv[1], sys.argv[2]
    with Pool(2) as pool: out = pool.map(run, [(*L, src) for L in LAYOUTS])
    allr = json.load(open('results/e62_eval_all.json')) if os.path.exists('results/e62_eval_all.json') else json.load(open('results/e53_eval_all.json'))
    for ln, v in out: allr[ln][tag] = dict(late=v)
    json.dump(allr, open('results/e62_eval_all.json', 'w'))
    print(tag + ': ' + '  '.join(f'{ln} {v:.3f}' for ln, v in out) + f'  | worst case {min(v for _, v in out):.3f}')
