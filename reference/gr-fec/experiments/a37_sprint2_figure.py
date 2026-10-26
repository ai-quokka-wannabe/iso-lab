"""Figure: worm sprint 2 — noise threshold, grown grounding, complex-edge timing."""
import numpy as np, matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
fig, ax = plt.subplots(1, 3, figsize=(16, 4.4))
k = [0, 0.1, 0.3, 1, 3]; x = [0.03, 0.1, 0.3, 1, 3]
w1 = [0.598, 0.458, 0.447, 0.783, 0.852]; e1 = [0.006, 0.010, 0.011, 0.010, 0.013]
w2 = [-0.028, -0.163, -0.168, 0.341, 0.263]; e2 = [0.006, 0.012, 0.015, 0.015, 0.032]
tr = [1.50, 1.56, 1.92, 5.09, 19.97]
ax[0].errorbar(x, w1, e1, fmt='-o', color='#2a6fdb', capsize=3, label='single food peak')
ax[0].errorbar(x, w2, e2, fmt='-s', color='#c0392b', capsize=3, label='with decoy (trap)')
ax[0].set_xscale('log'); ax[0].set_xticks(x); ax[0].set_xticklabels(['0', '0.1', '0.3', '1', '3']); ax[0].axhline(0, color='k', lw=.5)
for xi, t in zip(x, tr): ax[0].text(xi, -0.33, f'{t:.1f}', ha='center', fontsize=7, color='#555')
ax[0].text(0.03, -0.27, 'turns/min (single peak):', fontsize=7, color='#555')
ax[0].set_ylim(-0.38, 1.0); ax[0].set_xlabel('sensor noise k (only source of randomness)'); ax[0].set_ylabel('chemotaxis index (12 starts, ±SE)')
ax[0].set_title('Below the turn threshold noise is blur;\nabove it, noise becomes search', fontsize=9); ax[0].legend(fontsize=7, loc='upper left'); ax[0].grid(alpha=.3)
names = ['A1 odour', 'A2 thermo\nmemory', 'A3 salt\nlearning', 'A4 habit-\nuation', 'A5 forag-\ning']
V = {'fixed bins (v1)': [1, 1, 1, 1, 1], 'grown, tuned only (25)': [0, 1, 0, 1, 1], 'grown + intensity (25b)': [1, 1, 0, 1, 0], 'grown + intensity,\ngain-matched (25c)': [1, 1, 1, 1, 0]}
im = np.array(list(V.values()))
ax[1].imshow(im, cmap='RdYlGn', vmin=-0.3, vmax=1.3, aspect='auto')
ax[1].set_xticks(range(5)); ax[1].set_xticklabels(names, fontsize=7); ax[1].set_yticks(range(4)); ax[1].set_yticklabels(list(V.keys()), fontsize=7)
for i in range(4):
    for j in range(5): ax[1].text(j, i, 'pass' if im[i, j] else 'fail', ha='center', va='center', fontsize=7)
ax[1].set_title('Concepts grown by a novelty gate\n(needs two kinds of node: intensity + tuned)', fontsize=9)
L = 16; w = 2 * np.pi / L; t = np.arange(24)
rng = np.random.default_rng(9000)
def samp(l, n=200): return np.array([rng.choice(l) + rng.integers(-1, 2) for _ in range(n)])
ab = samp([3])
for lags, col, lab in (([4], '#2a6fdb', 'B→food 4±1'), ([2, 8], '#c0392b', 'B→food 2 or 8')):
    bf = samp(lags); rl = np.convolve(np.bincount(ab, minlength=L)[:L] / 200, np.bincount(bf, minlength=L)[:L] / 200)[:24]
    z = np.mean(np.exp(1j * w * ab)) * np.mean(np.exp(1j * w * bf))
    ax[2].bar(t + (0.2 if len(lags) > 1 else -0.2), rl, width=0.4, color=col, alpha=.5, label=f'{lab}: real timing vector (16 numbers)')
    ax[2].axvline((np.angle(z) / w) % L, color=col, lw=2, ls='--', label=f'one complex number: t = {(np.angle(z)/w)%L:.1f}, |z| = {abs(z):.2f}')
ax[2].set_xlabel('steps after odour A (A→food never experienced)'); ax[2].set_ylabel('predicted probability of food')
ax[2].set_title('Complex edge = compact timing code:\nA→B (3±1) composed with B→food', fontsize=9); ax[2].legend(fontsize=6.5); ax[2].grid(alpha=.3)
fig.suptitle('Worm sprint 2', fontsize=11); plt.tight_layout(); plt.savefig('results/worm_sprint2.png', dpi=140)
