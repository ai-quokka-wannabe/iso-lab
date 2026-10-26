import sys, time, numpy as np
sys.path.insert(0, 'src')
from ldpc import random_regular_no4cycles, girth, BP, awgn_llr
rng = np.random.default_rng(1)
t=time.time(); H = random_regular_no4cycles(200, 3, 6, rng); print('build', round(time.time()-t,1),'s', H.shape, 'girth', girth(H))
rank = np.linalg.matrix_rank(H.astype(float)); print('rank(real, approx)', rank)
dec = BP(H)
for eb in [1.5, 2.5, 3.5]:
    t=time.time(); llr = awgn_llr(4000, 200, eb, 0.5, rng)
    hard, done, used = dec.decode(llr, iters=50)
    errs = hard.sum(1)
    fer = np.mean(errs>0)
    print(f'Eb/N0={eb}: FER={fer:.4f}  nonconverged={np.mean(~done):.4f}  undetected={np.mean(done&(errs>0)):.4f} mean iters={used.mean():.1f}  t={time.time()-t:.1f}s')
