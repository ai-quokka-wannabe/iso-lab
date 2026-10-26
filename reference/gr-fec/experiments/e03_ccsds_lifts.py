"""Random QC lifts of the CCSDS (128,64) protograph: does curvature rank their FER?"""
import sys, time, json, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist, gf2_rank
from ldpc import BP, awgn_llr, girth
from curvature import node_ollivier, resistance_curvature, nb_closed_walks
P = 16
H0 = read_alist('results/CCSDS_64_128.alist')
base = [[np.nonzero(H0[r*P:(r+1)*P, c*P:(c+1)*P][0])[0].tolist() for c in range(8)] for r in range(4)]

def build(shifts):
    H = np.zeros((64, 128), dtype=np.uint8)
    I = np.eye(P, dtype=np.uint8)
    for r in range(4):
        for c in range(8):
            for s in shifts[r][c]:
                H[r*P:(r+1)*P, c*P:(c+1)*P] ^= np.roll(I, s, axis=1)
    return H

def no4(H):
    G = H.T.astype(int) @ H.astype(int); np.fill_diagonal(G, 0)
    return G.max() < 2

def features(H):
    f = nb_closed_walks(H, lengths=(6, 8, 10))
    o2, o3, rc = node_ollivier(H, 2), node_ollivier(H, 3), resistance_curvature(H)
    return {'c6': f[6].sum(), 'c8': f[8].sum(), 'c10': f[10].sum(),
            'oll2_mean': o2.mean(), 'oll2_min': o2.min(), 'oll3_mean': o3.mean(), 'oll3_min': o3.min(),
            'res_mean': rc.mean(), 'res_min': rc.min(), 'girth': girth(H), 'rank': gf2_rank(H)}

rng = np.random.default_rng(int(sys.argv[1]) if len(sys.argv) > 1 else 11)
NL = int(sys.argv[2]) if len(sys.argv) > 2 else 30
EB, TARGET, CAP = 4.0, 10**9, 150_000
lifts = [('ccsds', H0)]
def bad_blocks(H):
    G = H.T.astype(int) @ H.astype(int); np.fill_diagonal(G, 0)
    v1, v2 = np.nonzero(G >= 2)
    return sorted(set((v1 // P).tolist()) | set((v2 // P).tolist()))

def random_lift(rng, max_steps=5000):
    sh = [[sorted(rng.choice(P, size=len(b), replace=False).tolist()) for b in row] for row in base]
    for _ in range(max_steps):
        H = build(sh)
        bc = bad_blocks(H)
        if not bc:
            return sh, H
        c = int(rng.choice(bc))
        r = int(rng.choice([r for r in range(4) if base[r][c]]))
        sh[r][c] = sorted(rng.choice(P, size=len(base[r][c]), replace=False).tolist())
    return None, None

res = []
def lift_stream():
    yield 'ccsds', H0
    k = 1
    while k < NL:
        sh, H = random_lift(rng, max_steps=400)
        if H is not None:
            yield f'lift{k}', H
            k += 1

for name, H in lift_stream():
    t = time.time()
    F = features(H)
    dec = BP(H); fails = frames = 0
    while fails < TARGET and frames < CAP:
        llr = awgn_llr(25000, 128, EB, 0.5, rng)
        hard, done, used = dec.decode(llr, iters=50)
        fails += int((hard.sum(1) > 0).sum()); frames += 25000
    F.update(name=name, fer=fails / frames, fails=fails, frames=frames, H=H.tolist())
    res.append(F)
    print(f"{name:8s} girth={F['girth']} c6={F['c6']:.0f} c8={F['c8']:.0f} oll3_mean={F['oll3_mean']:.4f} res_min={F['res_min']:.4f}  FER={F['fer']:.3e} ({fails})  {time.time()-t:.0f}s", flush=True)
    json.dump(res, open(f'results/e03_lifts_{sys.argv[1] if len(sys.argv)>1 else 11}.json', 'w'), default=float)
