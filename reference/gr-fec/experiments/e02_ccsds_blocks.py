"""CCSDS TC LDPC (128,64): per-circulant-block failure participation."""
import sys, time, json, numpy as np
sys.path.insert(0, 'src')
from alist import read_alist
from ldpc import BP, awgn_llr
H = read_alist('results/CCSDS_64_128.alist'); n = 128
dec = BP(H); rng = np.random.default_rng(7)
out = {}
for eb, target, cap in [(3.0, 3000, 2_000_000), (4.0, 3000, 4_000_000), (5.0, 1500, 12_000_000)]:
    cnt = np.zeros(n); fails = 0; frames = 0; t = time.time(); wts = []; nonconv = 0; undet = 0
    while fails < target and frames < cap:
        llr = awgn_llr(50000, n, eb, 0.5, rng)
        hard, done, used = dec.decode(llr, iters=50)
        bad = hard.sum(1) > 0
        frames += len(llr)
        if bad.any():
            cnt += hard[bad].sum(0); fails += int(bad.sum()); wts += hard[bad].sum(1).tolist()
            nonconv += int((bad & ~done).sum()); undet += int((bad & done).sum())
    out[eb] = {'frames': frames, 'fails': fails, 'cnt': cnt.tolist(), 'weights': wts, 'nonconv': nonconv, 'undetected': undet}
    blk = cnt.reshape(8, 16).sum(1)
    print(f'Eb/N0 {eb}: FER={fails/frames:.2e} ({fails}/{frames}) nonconv={nonconv} undetected={undet} median wt={np.median(wts):.0f}  {time.time()-t:.0f}s', flush=True)
    print('   per-block bit-errors:', blk.astype(int).tolist(), flush=True)
    json.dump(out, open('results/e02_ccsds_blocks.json', 'w'))
