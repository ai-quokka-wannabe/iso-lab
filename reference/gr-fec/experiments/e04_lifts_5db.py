"""Re-simulate confirmation-batch lifts at 5 dB (towards the floor). Exploratory."""
import sys, time, json, numpy as np
sys.path.insert(0, 'src')
from ldpc import BP, awgn_llr
part, nparts = int(sys.argv[1]), int(sys.argv[2])
L = [r for r in json.load(open('results/e03_lifts_12.json'))]
L = L[part::nparts]
rng = np.random.default_rng(500 + part)
out = []
for r in L:
    H = np.array(r['H'], dtype=np.uint8); dec = BP(H); t = time.time()
    fails = frames = 0; wts = []
    while frames < 800_000:
        llr = awgn_llr(25000, 128, 5.0, 0.5, rng)
        hard, done, used = dec.decode(llr, iters=50)
        b = hard.sum(1) > 0
        fails += int(b.sum()); frames += 25000; wts += hard[b].sum(1).tolist()
    rec = {k: r[k] for k in r if k != 'H'}; rec.update(fer5=fails / frames, fails5=fails, frames5=frames, wts5=wts)
    out.append(rec)
    print(f"{r['name']:8s} FER4={r['fer']:.3e}  FER5={fails/frames:.3e} ({fails})  median wt={np.median(wts) if wts else 0:.0f}  {time.time()-t:.0f}s", flush=True)
    json.dump(out, open(f'results/e04_5db_part{part}.json', 'w'), default=float)
