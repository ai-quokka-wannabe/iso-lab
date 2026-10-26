"""Pass/fail and key numbers across worm versions."""
import numpy as np, matplotlib
matplotlib.use('Agg'); import matplotlib.pyplot as plt
cols = ['A1 odour\nchemotaxis', 'A2 thermo\nmemory', 'A3 salt\nlearning', 'A4 habit-\nuation', 'A5 forag-\ning', 'Mohri:\nstarved vs fed', 'needs sensor\nnoise?', 'hunger gating\n(hyper vs pairwise)']
rows = {
 'v0  fixed bins, single-assay calibration': (['+0.67', '+2.0 °C', '+0.77/+0.01', 'pass', '×1.4', '4.0 vs 2.2', '—', '—'], [1, 0, 1, 1, 0, 1, .5, .5]),
 'v1  + no absence nodes, 2-assay calibration': (['+0.61', '+4.3 °C', '+0.33/−0.08', 'pass', '×2.7', '3.5 vs 1.8', 'yes (rugged bins)', '—'], [1, 1, 1, 1, 1, 1, 1, .5]),
 'v2c grown nodes: intensity + tuned': (['+0.50', '+4.9 °C', '+0.32/−0.11', 'pass', '×1.5', '—', '—', '—'], [1, 1, 1, 1, 0, .5, .5, .5]),
 'v3  + inner senses as predictors, hyperedges': (['+0.62', '−1.1 °C', '+0.42/−0.18', 'pass', '×1.4', '3.3 vs 2.7', 'no (smooth code)', 'none'], [1, 0, 1, 1, 0, 1, 0, 0]),
 'v3b inner senses as gates': (['+0.63', '+3.1 °C', '+0.49/−0.16', 'pass', '×1.5', '3.3 vs 0.7', '(not re-run)', 'weak, at ceiling'], [1, 1, 1, 1, 0, 1, .5, 0]),
 'v4  EMBODIED: brain sees only senses': (['+0.57', '+2.85 °C', '+0.24/−0.12', 'pass', '×1.4', '3.4 vs 0.9', '—', '+0.12 vs −0.21'], [1, 0, 1, 1, 0, 1, .5, .25]),
 'v4b embodied + temporal integration': (['+0.74', '+4.07 °C', '+0.37/−0.13', 'pass', '×5.8', '3.4 vs 0.8', '—', '+0.07 vs −0.10'], [1, 1, 1, 1, 1, 1, .5, .25]),
}
fig, ax = plt.subplots(figsize=(15, 5.0))
M = np.array([v[1] for v in rows.values()])
ax.imshow(M, cmap='RdYlGn', vmin=-0.3, vmax=1.3, aspect='auto')
for i, (k, (txt, _)) in enumerate(rows.items()):
    for j, t in enumerate(txt): ax.text(j, i, t, ha='center', va='center', fontsize=7.5)
ax.set_xticks(range(len(cols))); ax.set_xticklabels(cols, fontsize=8); ax.xaxis.tick_top()
ax.set_yticks(range(len(rows))); ax.set_yticklabels(list(rows.keys()), fontsize=8)
ax.set_title('Worm versions on the same pre-registered tests (green = pass / supports; red = fail; yellow = not tested / suggestive)', fontsize=10, pad=40)
plt.tight_layout(); plt.savefig('results/worm_versions.png', dpi=140)
