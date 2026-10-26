# Riemannian / GR Geometry of FEC & EDAC Signal Spaces — Research Brief

**Owner:** Matej
**Compiled:** 23 October 2026, from four Claude chats (Jan–Jul 2026)
**Purpose:** a complete starting dossier for a research project and eventual paper, to be worked on in Claude Code.
**Language:** British English throughout.

> **Core thesis (Matej's words, the anchor of the whole project):**
> *"A signal is a point in some very high-dimensional phase space, travelling across that space — and that space behaves like gravity."*

---

## 0. Read this first (instructions for Claude Code)

1. **The governing rule of this project is "can reality referee this?"** No claim goes into the paper unless it has been checked against ground truth, a Monte-Carlo simulation, or a proper null/control. A beautiful picture with no referee is a *picture*, not a *result*.
2. **Section 6 (Audit) overrides earlier sections.** Several headline numbers from the chats are, on inspection, artefacts or tautologies. They are reproduced faithfully in §2–§4 because they are the history, but §6 says which ones survive. Do **not** build on a §2–§4 result without reading its §6 entry.
3. **Phase 0 (§8) is mandatory before anything else:** reproduce, then try to break, every inherited number.
4. The human (Matej) steers and must understand every result before it is merged. Prefer small, inspectable scripts and clear plots over clever monoliths. Explain *why* a number came out, not just *what* it is.
5. Use fixed seeds, log every parameter, and keep a `results/LEDGER.md` that labels each result **Genuine / Faithful (reproduces known theory) / Suggestive / Refuted**.
6. Matej explicitly wants brutal honesty and "mirage or real?" flagging. Overclaiming is the main failure mode of this project's history; avoid it.

---

## 1. Source chats

| Chat | Date | What it contains |
|---|---|---|
| [Vector and tensor analysis operators](https://claude.ai/chat/61792f33-0564-4902-8f15-a18ce7883aa4) | Jan–Feb 2026 | **Original work.** Hamming(7,4) embedding, curvature tensors, −0.988 "Einstein" correlation, phase transition, Fisher-Rao vs Hamming, BP vs gradient flow. Script `deep_exploration.py` and plots were produced there. |
| [Einstein equations in Hamming space](https://claude.ai/chat/f9efc9ed-3adf-488f-856b-d521f8f78806) | Mar 2026 | Retrieval and clean written summary of the above; list of unpursued applications. |
| [General relativity in FEC signal hyperspaces](https://claude.ai/chat/ba026b87-2c6f-45eb-9a83-9ae6180a3af8) | Jul 2026 | **Big extension.** SEC/DED faithfulness, BER as Kramers/Hawking escape, Schwarzschild attempt, "wormhole" antipodal throats, directional-noise test, `Geometric_Signal_Spaces_Compendium.md`, optimiser-diagnostic instrument, transformer thought-space geometry with null controls. |
| [Biological modification of general relativity](https://claude.ai/chat/8327768b-1f3c-4538-93a9-46c0116afb47) | Jan 2026 | **Not part of this project** (logistic gravity). Listed only so nobody goes looking. |

Scripts written in the July chat (in its sandbox, **not available locally — must be rewritten**): `wormhole.py`, `throat_test.py`, `open_throat.py`, `optic.py`, `thought_geom.py`. Their logic is reconstructed in §5.

---

## 2. Foundations — the original construction (Chat 1)

### 2.1 Embedding

Hamming(7,4): 16 codewords in {0,1}⁷, embedded in continuous ℝ⁷ as Gaussian wells:

$$E(\mathbf{x}) = -\sum_{\mathbf{c}\in\mathcal{C}} e^{-\beta\|\mathbf{x}-\mathbf{c}\|^2}$$

- The Hessian $H_{ij}=\partial_i\partial_j E$ was used as a Riemannian metric (only valid where positive-definite — this restriction was never handled carefully).
- Christoffel symbols, Riemann, Ricci, scalar curvature and Einstein tensor were computed numerically.
- Later work (Chat 3) moved to **extended Hamming(8,4)** in **BPSK** ($\pm1$) as well as the {0,1} embedding. Keep track of which embedding a result used.

### 2.2 Stress-energy and the "Einstein equation"

$$T_{ij}(\mathbf{x}) = \sum_{\mathbf{c}} P(\mathbf{c}\mid\mathbf{x})\,\frac{(c_i-x_i)(c_j-x_j)}{\sigma^2}$$

with $P(\mathbf{c}\mid\mathbf{x})$ a Boltzmann/posterior weight. At midpoints between codewords, $\mathrm{corr}(G_{ij},T_{ij})$ reached **−0.988**, read as $G_{ij}=\kappa T_{ij}$.

Caveats recorded at the time: in 1D/2D the Einstein tensor vanishes identically, so the test only means anything in $n\ge3$; the trace relation $R=\kappa\,\mathrm{Tr}(T)$ was marked "⚠ partial"; universality of κ was never tested. See §6.1.

### 2.3 The claimed "proofs" (status table from Chat 1)

| Claim | Status then | Notes |
|---|---|---|
| AWGN ≡ Langevin dynamics, $T=\sigma^2$ | "Proven, analytical" | Langevin $\dot x=-\nabla U+\sqrt{2T}\xi$ gives $P\propto e^{-U/T}$; AWGN gives $P(r\mid s)\propto e^{-\|r-s\|^2/2\sigma^2}$. |
| Soft decoding = gradient descent | "Proven" | $\nabla E(\mathbf{x}) = (\mathbf{x}-\mathbb{E}[\mathbf{c}\mid\mathbf{x}])/\sigma^2$; analytic vs numeric gradient error $9.19\times10^{-12}$; energy decreases monotonically to MAP. |
| Phase transition at critical SNR | "Proven" | Susceptibility peak + order parameter; **critical SNR ≈ −3 dB** for Hamming(7,4). "Liquid" (diffuse posterior) vs "crystal" (posterior on one codeword). |
| Fisher-Rao bounds BER | "Proven" | Bhattacharyya: $P_e\le\exp(-d_{FR}^2/8)$ with $d_{FR}=\|c_1-c_2\|/\sigma$; verified on BPSK repetition pair at 0–8 dB. |
| Fisher-Rao predicts BER better than Hamming | "Found" | corr(BER, $d_{FR}$) = **−0.998** vs corr(BER, $d_H$) = **−0.878**. |
| BP ≈ gradient flow on Bethe free energy | "Found" | ~**95 %** correlation through iterations. |
| $G_{ij}=\kappa T_{ij}$ | "Found" | **−0.988** at midpoints. |

### 2.4 Physics ↔ coding ↔ geometry dictionary

| Physics | Coding / DSP | Geometry |
|---|---|---|
| Temperature $T$ | Noise $\sigma^2$ | Diffusion on manifold |
| Inverse temperature $\beta$ | SNR | Metric sharpness |
| Boltzmann $P(E)$ | Posterior $P(\mathbf{c}\mid\mathbf{r})$ | Exponential map |
| Free energy | Log-likelihood | Energy functional |
| Gradient flow | Soft decoding | Geodesic motion |
| Phase transition | Decoding threshold | Topology change |
| Susceptibility | BER sensitivity | Curvature |
| Black hole / well | Codeword | Attractor basin |
| Event horizon | Decision boundary | Saddle ridge |
| Hawking emission | Bit error | Thermal escape over saddle |
| Wormhole throat | Low-barrier link between far codewords | Low-action geodesic |

### 2.5 Applications listed but never pursued

- Curvature-aware LDPC design (place codewords in low-curvature regions).
- SNR-adaptive coding (Fisher-Rao metric depends on channel).
- Early termination: detect the "phase transition" to stop iterating.
- Geometric decoder scheduling: follow geodesics rather than fixed schedules.
- Unequal error protection: Fisher metric flags "important" bits.
- Quantum error correction (Fubini–Study metric is native there).

---

## 3. The July extension — FEC / EDAC results (Chat 3)

### 3.1 Faithfulness: geometric decoder reproduces SEC and SEC/DED

A pure gradient-descent-on-wells decoder (no syndrome table, no parity-check matrix):

- **Hamming(7,4) SEC:** 112/112 single-bit errors corrected; agreement with syndrome decoder on **all 128** received words.
- **Extended Hamming(8,4) SEC/DED:** 128/128 single errors corrected; **448/448** double errors flagged as ambiguous (signal lands on a saddle equidistant between two wells, e.g. 1.906 vs 1.906); **zero silent miscorrections**.
- **Mechanism:** single error → inside one basin → rolls in. Double error → on the saddle between two wells → stalls → "detected". *The saddle is the geometric mechanism of double-error detection.*

### 3.2 BER as thermal escape ("Hawking radiation")

- Wrong first guess (rejected): BER ∝ curvature at the boundary. Curvature is the *sharpness* of the watershed, not the *crossing probability*.
- Accepted: Kramers/Arrhenius escape over the saddle,

$$\text{BER}\sim\exp\!\Big(-\tfrac{\Delta E_{\text{saddle}}}{\sigma^2}\Big)\;\longleftrightarrow\;\text{BER}\approx Q\!\Big(\tfrac{d_{\min}}{2\sigma}\Big)\sim\exp\!\Big(-\tfrac{d_{\min}^2}{8\sigma^2}\Big)$$

  with $\Delta E_{\text{saddle}}$ playing the role of $d_{\min}^2$.
- Monte-Carlo AWGN: predicted and simulated waterfalls were **parallel, same slope** (same diversity exponent). Interpreted as "Hawking temperature derivation = union bound, two languages".
- **Proposed GR refinement (never tested):** in curved space use the geodesic/instanton action:

$$\text{BER}\sim\exp\!\Big(-\tfrac{S_{\text{geodesic}}[\text{well}\to\text{saddle}]}{\sigma^2}\Big)$$

  Test proposed: geodesic-action BER vs union bound vs simulated BER on one plot, especially at **low SNR** where the union bound is loose.

### 3.3 Schwarzschild attempt

- Single Gaussian well in ℝ⁷ as an isolated spherically symmetric source; vacuum = region where other codewords' posterior weight is negligible; horizon = decision boundary ("an uncorrectable error is a horizon you can't climb back out of").
- The Hessian metric is Riemannian, so a time direction must be chosen: (a) decoding-trajectory parameter as time (clean), or (b) Wick-rotate / promote β (SNR) to a Lorentzian time (bold, speculative).
- Expected outcome stated at the time: *not* exactly $1-r_s/r$; the falloff that does appear would characterise what kind of "matter" a Gaussian codeword is.
- **What was actually run:** Matej chose option (b). A (1+1) reduced metric $ds^2=-f\,d\tau^2+h\,dr^2$ with potential $\Phi(r)=A(1-e^{-\beta r^2})$, lapse $f=e^{-2\Phi}$ and $h=e^{2\Phi}$ (so $fh=1$), built in SymPy (`schwarzschild_code.py`).
- **Result reported:** a "regular, inside-out black hole": finite curvature everywhere (Kretschmann peaks at 16, no singularity); lapse is 1 at the codeword and falls to $e^{-2A}\approx0.135$ at infinity, the reverse of Schwarzschild; a capture threshold $E^2_{\text{crit}}=e^{-2A}$ read as the decoding boundary. "The trap isn't the codeword — it's the noise floor at infinity."
- **Audit note:** the lapse $f=e^{-2\Phi}$ and the condition $fh=1$ were *chosen*, not derived, so the regularity and the inside-out shape follow from the construction. Treat as **Suggestive**; it has no referee.

### 3.4 "Wormhole" hunt — antipodal throats in Hamming(8,4)

- Set-up: Hamming(8,4) codewords in {0,1}⁸, $\beta=4$; minimum barrier for all **120** codeword pairs via a Nudged Elastic Band (15 images, 400 iterations, spring $k=0.1$, step 0.01).
- Result: **8 "throats"**, exactly the 8 antipodal pairs (codeword + bitwise complement, $d_H=8$). Barrier ≈ **0.9946** vs nearest-neighbour ($d_H=4$) ≈ **0.9633**.
- Follow-up (`throat_test.py`): decoding along the straight line $c_0\to c_{15}$ stays in $c_0$'s basin to $t=0.50$, flips to $c_{15}$ at $t=0.55$ — **no intermediate basins**. A kick of magnitude **1.5** flips $c_0\to c_{15}$, and also **1.5** flips $c_0$ to a $d_H=4$ neighbour (kick grid 0.5, 1.0, …).
- Interpretation given: "real tunnel, vanishing-width door" — barrier as low as nearest neighbours, but the throat is 1-D in 8-D so isotropic noise never threads it.

### 3.5 "Opening the throat" — directional-noise test

- BPSK Hamming(8,4), ML (minimum-Euclidean) decoder, 200,000 transmissions, matched total noise power.

| Channel (matched power) | Antipodal confusions |
|---|---|
| Isotropic AWGN | **0** |
| Directional (≈ 92 % of power on the antipodal axis) | **9,577** (≈ half of all errors) |

- Claimed at the time as "the one genuine, falsifiable, simulation-confirmed new result". **See §6.2 — this claim does not survive scrutiny in its current form.**
- Engineering framing offered: correlated bit-flips from a failing driver, narrowband interference, ISI patterns, jamming.

### 3.6 Optimiser / neural-network thread (same chat)

- Matej's idea: geometry could give a new kind of backpropagation.
- Three attempts to make geometry **steer** optimisation failed; diagnosis: steering needs the **global geodesic**, i.e. Christoffel symbols along a path not yet walked, and in NN training the metric depends on the parameters being moved — a self-referential, Einstein-like coupling ("parameters tell the manifold how to curve, curvature tells parameters how to move").
- Natural gradient / Adam / Shampoo = greedy one-step, re-linearised approximations to geodesic following (Euler integration with constant re-linearisation).
- What **did** work: geometry as a **measurement instrument** (`optic.py`): at each step compute the true natural-gradient direction and report the angle and magnitude by which Adam/SGD/Shampoo deviate. "A speedometer doesn't have to drive the car."
- GPU discussion: batched Hessian-vector products and Lanczos make richer local curvature cheap, but memory capacity/bandwidth and third-derivative (geodesic-correction) cost kill true geodesic steering at scale.

### 3.7 Transformer "thought-space" thread (same chat)

- Analytic set-up written down:
  - Metric: pullback Fisher, $g(h)=J^\top G_{\text{out}}J$ along the residual-stream trajectory ($J_\ell$ the per-layer Jacobian — "the load-bearing equation").
  - Geodesics = paths of minimum change in output distribution ("minimum semantic disruption").
  - Feature-sourced stress-energy, $T_{ij}(h)\sim\sum_\phi w_\phi(h)(\phi_i-h_i)(\phi_j-h_j)/\sigma^2$.
  - Layer update = geodesic + learned forcing term.
- Toy experiment 1 (16-dim residual, 4 layers, two-concept task, 100 % accuracy): ambiguous input sits equidistant to both concept wells (38.647 vs 38.644; partly built-in symmetry); trajectory nearly flat (<5 % deviation); **≈14 % slow-down** at the ambiguous point; off-centre bend.
- Toy experiment 2 (`thought_geom.py`), **the only result with a proper control battery:**

| Condition | corr(metric, $T$) |
|---|---|
| Real concept features, at midpoints | **+0.931** |
| Random features | ≈ 0 |
| Shuffled metric coordinates | **−0.006** |
| Real features, random location | **+0.332** |

  Geodesic vs linear interpolation between concepts: confidence 0.999 vs 0.993 (small, right direction).
- Caveat: this tests corr(**metric**, $T$), not corr(**Einstein tensor**, $T$) — not the same claim as §2.2.

### 3.8 Other domains swept (context only — out of scope for the paper)

- **Primon gas / RH:** Fisher metric $g(\beta)=(\log\zeta)''=\mathrm{Var}(\log n)\to1/(\beta-1)^2$ at Hagedorn ($\beta\to1^+$), verified (ratio 0.998 → 0.99998). Curvature of $\log|\zeta|$ spikes at zeros — guaranteed by construction. No referee.
- **Quantum (Fubini–Study):** qubit distances $\pi/2$, $\pi/4$; Mandelstam–Tamm as a geodesic statement; Berry phase $=-\tfrac12\Omega$. Native geometry, fully validating, no surprise.
- **Domain classifier (worth keeping for the paper's framing):** the lens pays off where geometry is *imposed* (not native), there is a *crisp referee*, and the community does *not* already think geometrically. FEC fits all three.
- Other candidate spaces named: lattice cryptography (closest-vector problem = nearest-well decoding), fitness landscapes, SAT landscapes, BHV phylogenetic tree space.

---

## 4. Key formulae (collected)

$$E(\mathbf{x})=-\sum_{\mathbf{c}}e^{-\beta\|\mathbf{x}-\mathbf{c}\|^2},\qquad g_{ij}=\partial_i\partial_jE\ \ (\text{where }>0)$$

$$\Gamma^k_{ij}=\tfrac12g^{km}(\partial_ig_{jm}+\partial_jg_{im}-\partial_mg_{ij}),\qquad \ddot x^k+\Gamma^k_{ij}\dot x^i\dot x^j=0$$

$$G_{ij}=R_{ij}-\tfrac12Rg_{ij}\stackrel{?}{=}\kappa T_{ij},\qquad T_{ij}=\sum_{\mathbf{c}}P(\mathbf{c}\mid\mathbf{x})\frac{(c_i-x_i)(c_j-x_j)}{\sigma^2}$$

$$\nabla E(\mathbf{x})=\frac{\mathbf{x}-\mathbb{E}[\mathbf{c}\mid\mathbf{x}]}{\sigma^2}\quad(\text{log-sum-exp form})$$

$$P_e\le\exp(-d_{FR}^2/8),\quad d_{FR}=\|c_1-c_2\|/\sigma;\qquad \text{BER}\sim e^{-\Delta E/\sigma^2}\sim e^{-S_{\text{geo}}/\sigma^2}$$

---

## 5. Reconstructed code skeletons

These reproduce the logic of the July scripts. Rewrite cleanly; vectorise; add seeds and tests.

```python
import numpy as np
from itertools import product, combinations

# Extended Hamming(8,4)
H7 = np.array([[0,0,0,1,1,1,1],[0,1,1,0,0,1,1],[1,0,1,0,1,0,1]])
base = np.array([b for b in product([0,1], repeat=7) if np.all((H7 @ np.array(b)) % 2 == 0)])
C01 = np.hstack([base, (base.sum(1) % 2)[:, None]]).astype(float)  # {0,1}^8
Cb  = 1 - 2 * C01                                                   # BPSK ±1

def E(p, C, beta):      return -np.exp(-beta * ((p - C)**2).sum(1)).sum()
def gradE(p, C, beta):
    d = p - C; w = np.exp(-beta * (d**2).sum(1))
    return 2 * beta * (w[:, None] * d).sum(0)

# Gradient-descent "geometric decoder" (used for SEC/DED faithfulness)
def geo_decode(r, C, beta, steps=2000, dt=0.02):
    p = r.astype(float).copy()
    for _ in range(steps): p -= dt * gradE(p, C, beta)
    return int(np.argmin(((p - C)**2).sum(1)))

# NEB minimum-barrier search (wormhole.py): 15 images, 400 iters, k=0.1, dt=0.01
# Directional-noise test (open_throat.py): ML decode argmin ||Cb - rx||^2,
#   isotropic AWGN vs same total power with ~92% along (Cb[j]-Cb[i]) axis, 200k trials.
```

Original Chat-1 numerical check (keep as a unit test): Bhattacharyya bound vs exact $Q(d/2\sigma)$ vs simulation for BPSK pair $[-1,-1,-1]$ / $[1,1,1]$ at 0, 2, 4, 6, 8 dB, $\sigma^2=1/(2\,\text{SNR})$, 50k trials.

---

## 6. Audit — mirage or real? (READ BEFORE BUILDING)

Verified in the compiling session (23 Oct 2026) or derivable on paper. Ordered by severity.

### 6.1 The −0.988 "Einstein equation" — **unproven, likely construction artefact**
- $T_{ij}$ is (up to scale) the **posterior covariance** of the codeword around $\mathbf{x}$. The Hessian of a log-sum-exp energy is *made of* that same posterior covariance. So the metric, hence $G$, and $T$ share a construction; a high correlation is expected without any "Einstein-like law".
- The well-shape artefact test (Lorentzian, compact-support, anisotropic wells) was proposed twice and **never run** in code space.
- No null controls were ever run for the FEC version (the transformer version in §3.7 had them, but tested a different quantity).
- Using the Hessian as a metric is only valid where it's positive-definite; near saddles (exactly where the midpoints are) it is **not**. How this was handled is undocumented.
- **Status: Suggestive at best. Phase 0 must settle it.**

### 6.2 The "wormhole"/directional-noise result — **as stated, not a new result**
Verified numerically:
- Extended Hamming(8,4) has pairwise distances **only 4 (112 pairs) and 8 (8 pairs)**. "The hunt found exactly the 8 antipodal pairs" is therefore **guaranteed**: they are the *only* pairs with $d_H>4$.
- At $\beta=4$ in the {0,1} embedding, wells barely overlap: well depth $=-1.0$, midpoint barrier $0.9633$ (nearest) vs $0.9946$ (antipodal), vs **1.0** if wells did not interact at all. **Every barrier saturates at ≈ the well depth**, so "antipodal barrier ≈ nearest-neighbour barrier" says nothing about the code; it's the β choice.
- Concentrating noise along the axis $c_j-c_i$ makes **any** pair $(i,j)$ confusable — not specifically antipodal ones. And an ML decoder that assumes white noise is **mismatched** to coloured noise; the correct decoder whitens (Mahalanobis distance), and the relevant distance spectrum is the whitened one. This is standard coloured-noise detection theory.
- The kick-size equivalence (1.5 vs 1.5) comes from a coarse 0.5-step grid; the Euclidean midpoints differ (1.0 vs √2 in {0,1}).
- **What might survive:** a *properly posed* question — "for a realistic structured impairment (burst, ISI, narrowband interferer), does a geometric quantity predict *which* codeword pairs become vulnerable better than the standard (whitened) distance spectrum?" That is open, refereeable and possibly interesting. The current result is not it.
- **Status: Refuted as a discovery; reusable as a motivating example only.**

### 6.3 Fisher-Rao "beats" Hamming (−0.998 vs −0.878) — **suspicious**
- For equal-variance Gaussians $d_{FR}=d_E/\sigma$, and for BPSK $d_E^2=4d_H$, so $d_{FR}$ is a monotone function of $d_H$ at fixed σ. The difference in correlation must come from comparing across different SNRs/codes or soft vs hard metrics. Needs re-deriving exactly what was correlated with what.
- The Bhattacharyya bound itself is textbook-correct and is just the union-bound exponent. **Status: Faithful (known theory), novelty claim unproven.**

### 6.4 BER as Kramers/Hawking escape — **Faithful, not new**
- Same exponent as the union bound by construction. Fine as a framing section. The *geodesic-action correction at low SNR* (§3.2) is the only part that could be new, and it is **untested**.

### 6.5 SEC/DED faithfulness — **Faithful, solid**
- A sufficiently sharp energy-well gradient decoder converges to the nearest codeword, so agreement with syndrome decoding is expected. Good as a sanity check and a nice pedagogical figure ("the saddle *is* double-error detection"). Not new.

### 6.6 Phase transition at ≈ −3 dB, BP ≈ gradient flow (95 %) — **Faithful, prior art exists**
- Statistical mechanics of decoding (Sourlas 1989; Nishimori; Montanari, Mézard) and BP as Bethe free-energy minimisation (Yedidia–Freeman–Weiss) already establish these. A finite code (16 codewords) has no true thermodynamic phase transition; the "transition" is a smooth crossover — the paper must say so.

### 6.7 Transformer +0.931 with nulls — **Genuine at toy scale, different claim**
- Best-controlled result in the record. But it correlates the *metric* with feature-sourced $T$, not $G$ with $T$; and features were built into a toy task. Out of scope for an FEC paper; keep as a possible follow-up.

### 6.8 Prior art
Moved and expanded into **§10 (Literature review, 23 Oct 2026)**. Short version: almost every *foundation* result in §2–§3 is already known in some form. Novelty has to come from the hypotheses in §7.

---

## 7. What a paper could realistically claim

Ranked by (plausibility × novelty), **revised after the literature review in §10**. The honest headline today is: **there is no confirmed new result yet.** There *is* a coherent framework, a set of faithful reproductions, and several sharp, testable hypotheses — and the literature shows exactly where the unclaimed ground is.

1. **Curvature of the Bethe free energy as an online confidence / early-termination signal** (recommended lead).
   - *Why this and not the toy Hessian:* the principled curvature object already exists. Watanabe & Fukumizu showed the Hessian of the Bethe free energy is tied to the graph's edge zeta function and to stability of BP fixed points; Heskes showed stable BP fixed points are local minima of the Bethe free energy; Ikeda–Tanaka–Amari tied BP's decoding error to the e-curvature of a submanifold.
   - *The gap:* all of these use curvature for **analysis or a one-shot correction**, not as a **per-frame, per-iteration signal** for stopping, detecting undecodable frames, or flagging unreliable outputs. No paper found does that.
   - *Hypothesis:* a cheap estimate of the Bethe-Hessian's smallest eigenvalue (or the Ikeda–Tanaka–Amari e-curvature term), tracked along the decoding trajectory, detects non-converging frames earlier and/or predicts undetected errors better than the existing rules.
   - *Referee and baselines:* the standard criteria catalogued by Amador et al. — syndrome check, variable-node reliability (Kienle & Wehn), convergence of mean magnitude, satisfied-check counting (Shin et al.), checksum summation, soft-word cycle detection, plus a genie bound. Metrics: average iterations, FER loss, undetected-error rate, compute cost. Codes: DVB-S2 short frames and 5G NR base graphs.
2. **Kramers–Langer prefactor for instanton error-floor estimates.**
   - The "BER as escape over the saddle" idea is, in LDPC language, the **instanton** method (Stepanov, Chertkov, Chilappagari, Vasic): the most probable noise configuration that makes the decoder fail. Their main output is the leading exponent (an "effective distance", e.g. 12.45 for the Tanner (155,64,20) code under BP).
   - The genuinely geometric next step is the **prefactor**: in Kramers–Langer theory the escape rate's prefactor is set by the **curvature (Hessian eigenvalues) at the well and at the saddle**. Hypothesis: a curvature-corrected instanton estimate predicts error-floor FER better than exponent-plus-multiplicity alone.
   - *Must check first* whether the Chertkov group already computed fluctuation prefactors (not established in this review).
3. **Instantons under structured noise** (the honest successor to the "wormhole", §6.2).
   - Under noise with covariance Σ, the most probable failure is the one minimising the Mahalanobis norm, so the vulnerable trapping sets change. Hypothesis: whitened/Mahalanobis instanton search predicts which trapping sets dominate under bursts, ISI or narrowband interference better than AWGN-trained analysis. Baselines: matched (whitening) decoders; the virtual-check approach of Iyer, Sannamoth & Emerson for correlated errors.
4. **Discrete Ricci curvature of the Tanner graph as a cheap structural predictor.**
   - Ollivier curvature is already used in coding theory for **bounds** on code size (Iceland & Samorodnitsky), and in graph ML curvature explains **bottlenecks / over-squashing** in message passing (Topping et al.). BP is message passing on the Tanner graph.
   - *Gap:* no work found that uses Ollivier/Forman/balanced-Forman curvature to locate trapping sets or predict error floors. Hypothesis: strongly negative-curvature edges co-locate with dominant trapping sets. Cheap to test on codes with known trapping-set enumerations.
5. **Is there an "Einstein-like" law at all?** Downgraded to an appendix / negative-result section. Run the controls, report honestly either way.
6. **Pedagogical/framework contribution:** the unified dictionary, SEC/DED-as-saddle picture, the "imposed geometry + crisp referee" classifier. Good as framing sections, not as a main claim. Must cite the prior art in §10 for every dictionary entry.

Likely venues (later): IEEE Communications Letters / Trans. Inf. Theory (if 1–3 works); Entropy or Information Geometry (Springer) for a framework paper; ISIT/ITW workshop paper as a first step.

---

## 8. Work plan for Claude Code

### Phase 0 — Reproduce and break (do first; no new ideas until done)
- [ ] `src/codes.py`: Hamming(7,4), extended Hamming(8,4), embeddings {0,1} and BPSK, distance spectra.
- [ ] `src/landscape.py`: energy, gradient, Hessian (analytic, vectorised); positive-definiteness map.
- [ ] `src/curvature.py`: Christoffel, Riemann, Ricci, scalar, Einstein — via JAX/autodiff, not finite differences; unit-test on a sphere and on flat space.
- [ ] Reproduce −0.988; then run the **artefact battery**: Gaussian vs Lorentzian vs compact-support vs anisotropic wells; β sweep; random codes with same distance spectrum; shuffled $T$; random locations; define and report what "midpoint" sampling does.
- [ ] Reproduce SEC 112/112, 128/128 and SEC/DED 128 + 448, zero miscorrections.
- [ ] Reproduce Fisher-Rao vs Hamming correlations and pin down exactly what was compared.
- [ ] Re-run the directional-noise test, add: every-pair-axis version, matched (whitening) ML decoder, β sweep for NEB barriers.
- [ ] Write `results/LEDGER.md` with a verdict for each.

### Phase 1 — Literature (first pass done 23 Oct 2026, see §10)
- [ ] Read the full texts of the §10 "must read" items; confirm every claim in §10 against the papers themselves (§10 was compiled from abstracts and summaries).
- [ ] Specific open checks: (a) did the Chertkov group compute instanton prefactors / fluctuation determinants? (b) has anyone used the Bethe-Hessian or IG curvature as a stopping criterion? (c) has anyone used Ollivier/Forman curvature on Tanner graphs for trapping sets? A short forward-citation search on Ikeda–Tanaka–Amari 2004, Watanabe–Fukumizu and Chilappagari et al. should settle all three.
- [ ] Write `docs/prior_art.md`: for each entry in the §2.4 dictionary, the paper that already established it.

### Phase 2 — Lead hypothesis (default: §7 item 1, Bethe-Hessian stopping signal)
- [ ] Real LDPC codes (use an established library such as `aff3ct`, `sionna`, or `pyldpc` — check licences), AWGN + fading + burst channels.
- [ ] Implement the baseline stopping rules from Amador et al. (§10.5) first, then the curvature signal.
- [ ] Pre-register metrics and thresholds before running; report compute cost of the curvature estimate honestly (Lanczos on Hessian-vector products, not a full eigendecomposition).
- [ ] Cheap side-experiment for §7 item 4: Ollivier/balanced-Forman curvature on the Tanner (155,64,20) graph vs its known (5,3) trapping sets.

### Phase 3 — Write
- [ ] Paper skeleton in LaTeX: Introduction; Background (information geometry, stat-mech of codes); Framework (dictionary, embeddings); Faithfulness checks; Main result; Negative results / limitations (include §6 honestly); Conclusion.

### Suggested repository layout

```
gr-fec/
  CLAUDE.md            # short: rules from §0, pointers into this brief
  docs/brief.md        # this file
  docs/prior_art.md
  src/                 # codes, landscape, curvature, decoders, channels, neb
  experiments/         # one script per experiment, each writes to results/
  results/LEDGER.md    # Genuine / Faithful / Suggestive / Refuted per claim
  paper/
  tests/
```

Tooling: Python 3.11+, NumPy, SciPy, JAX (autodiff for curvature), Matplotlib; pytest; fixed seeds; every experiment script prints its full parameter set.

---

## 9. Ideas parked (not for the first paper)

- Wick-rotating β into a Lorentzian time; genuine light-cone structure in code space.
- Time-varying channel (β(τ)) as a "cosmology of decoding".
- Anti-codewords (negative wells) as forbidden symbols / naked singularities.
- Three-codeword saddle networks vs Voronoi tessellation.
- Quantum error correction via Fubini–Study.
- Lattice cryptography (CVP as nearest-well decoding) as the next "imposed geometry + referee" domain.
- Optimiser diagnostic instrument (angle/magnitude deviation from natural gradient).
- Transformer thought-space geometry at real scale (pullback-Fisher metric on a small open model; garden-path sentences as saddles).

---

## 10. Literature review (first pass, 23 October 2026)

Compiled from web searches, abstracts and paper summaries. **Verify every claim against the full paper before citing.** Grouped by which part of our framework it pre-empts or enables.

### 10.1 Verdict in one paragraph
The *foundations* of this project are largely known. Decoding as energy minimisation (Wadayama's GDBF and projected-gradient decoders), BP as Bethe free-energy minimisation (Yedidia–Freeman–Weiss, Heskes), the information geometry of turbo/LDPC decoding (Richardson, Ikeda–Tanaka–Amari), phase transitions in decoding (Kabashima, Saad, Vicente) and "error as escape over a barrier" (the instanton method of Chertkov and co-workers) are all established. What does **not** appear to exist: curvature used as an **online decoder signal**, **curvature (Kramers–Langer) prefactors** layered onto instantons, **Mahalanobis instantons for structured noise**, and **discrete Ricci curvature of Tanner graphs** for trapping sets. Those are the targets (§7).

### 10.2 Information geometry of iterative decoding — closest prior art
- **T. Richardson, "The geometry of turbo-decoding dynamics", IEEE Trans. Inf. Theory 46(1):9–23, Jan 2000.** Turbo decoding as a dynamical system on densities over the hypercube; product densities as a subspace; fixed points always exist (Brouwer); stability when eigenvalues of a product of Jacobians lie in the unit disc; approximate gap between turbo and ML log-likelihoods; suggests low-SNR breakdown is non-convergence rather than absence of fixed points. [PDF](https://www.josephboutros.org/ldpc_vs_turbo/turbo_Richardson_IT2000.pdf) · [IEEE](https://ieeexplore.ieee.org/document/817505)
- **S. Ikeda, T. Tanaka, S. Amari, "Information geometry of turbo and low-density parity-check codes", IEEE Trans. Inf. Theory 50(6):1097–1114, June 2004, doi:10.1109/TIT.2004.828072.** Unifies turbo, LDPC, loopy BP and the Bethe approximation with dual (e/m) affine connections. Decoding error comes from the mismatch between an m-flat equimarginal submanifold and its e-flat hull; the leading error correction is tied to the **e-curvature**; they give a post-processing correction; the principal error term vanishes without 4-cycles; equilibria coincide with Bethe free-energy critical points. Limitations they note: second-order, per-instance, global convergence left open. **Any curvature claim we make must be positioned against this paper.** [Teikyo record](https://pure.teikyo.jp/en/publications/information-geometry-of-turbo-and-low-density-parity-check-codes/) · [Tech-report draft](https://www.ism.ac.jp/~shiro/papers/techreport/TR2003-2.pdf) · [NeurIPS 2001 version](https://www.ism.ac.jp/~shiro/papers/books/ikeda_nips2001.pdf)
- **H. Ben-Azza et al., "Information geometry for decoding", 2025, doi:10.37256/cm.6120255392.** Revisits Ikeda et al.; m-projections onto e-flat submanifolds for BSC and Gaussian channels; alternating-projection convergence results on the hypercube; tested on Gallager codes. Shows the line is still active. [Link](https://ojs.wiserpub.com/index.php/CM/article/view/5392)

### 10.3 Bethe free energy, its Hessian, and BP stability
- **T. Heskes, "Stable fixed points of loopy belief propagation are local minima of the Bethe free energy", NeurIPS 2002.** [Link](https://proceedings.neurips.cc/paper/2002/hash/d2a27e83d429f0dcae6b937cf440aeb1-Abstract.html)
- **Y. Watanabe, K. Fukumizu, "Graph zeta function in the Bethe free energy and loopy belief propagation" (arXiv:1002.3307).** The determinant of the Bethe free-energy Hessian equals the reciprocal of the edge zeta function (up to a positive factor); positive-definiteness conditions; non-convexity with ≥ 2 cycles; stability of LBP fixed points via zeta-function eigenvalues. **This is the principled "curvature" object for §7 item 1.** [Link](https://ar5iv.arxiv.org/html/1002.3307)
- Yedidia, Freeman, Weiss — Bethe free energy and BP (verify exact reference).

### 10.4 Decoding as gradient descent on an energy (pre-empts "soft decoding = gradient descent")
- **T. Wadayama et al., "Gradient descent bit flipping algorithms for decoding LDPC codes", ISITA 2008.** Energy $E(x)=\alpha\sum_n x_ny_n+\sum_m\prod_{n\in H(m)}x_n$ (correlation + bipolar syndromes), maximised by bit flipping. [Academia copy](https://www.academia.edu/79882691/Gradient_descent_bit_flipping_algorithms_for_decoding_LDPC_codes)
- **V. Savin, "Gradient descent bit-flipping decoding with momentum", arXiv:2204.02359, 2022.** Adds a momentum term to escape loops — note the physical analogy (inertia) already used. [Link](https://arxiv.org/pdf/2204.02359)
- **T. Wadayama, S. Takabe, "Deep learning-aided trainable projected gradient decoding for LDPC codes", arXiv:1901.04630.** Minimises LLR-linear term plus a ReLU penalty outside the parity polytope; trained step sizes. Related: proximal decoding ([arXiv:2102.05256](https://arxiv.org/pdf/2102.05256), [arXiv:2409.07278](https://arxiv.org/html/2409.07278v1)). [Link](https://arxiv.org/abs/1901.04630)
- *Implication:* our Gaussian-well landscape is one more energy in a well-populated family. Cite this lineage in the framework section; don't claim the gradient-descent view as new.

### 10.5 Early-termination / stopping criteria (baselines for §7 item 1)
- **E. Amador, R. Knopp, V. Rezard, R. Pacalet, "Hybrid iteration control on LDPC decoders" (EURECOM).** Catalogues: syndrome check; soft-word cycle detection (Glikiotis & Paliouras); variable-node reliability (Kienle & Wehn); convergence of mean magnitude (Li, You & Li); satisfied-parity-check counting (Shin et al.); checksum summation (Cui, Chen & Wang); erased-message count under self-corrected min-sum; hybrid rule; genie bound. [PDF](https://www.eurecom.fr/publication/3166/download/cm-publi-3166.pdf)
- Further: [EURECOM follow-up](https://www.eurecom.fr/publication/3417/download/cm-publi-3417.pdf); a DVB-S2 decoding-control patent ([US 8010869](https://image-ppubs.uspto.gov/dirsearch-public/print/downloadPdf/8010869)) — relevant to Matej's satellite context.

### 10.6 Instantons — the established form of "BER as escape over the saddle"
- **M. G. Stepanov, M. Chertkov, "Instanton analysis of LDPC codes in the error-floor regime", arXiv:cs/0601070, 2006.** Numerical "instanton-amoeba" scheme; Tanner (155,64,20) and Margulis (672,336,16) codes; validated against Monte Carlo FER. [Link](https://arxiv.org/abs/cs/0601070)
- **S. K. Chilappagari, M. Chertkov, M. G. Stepanov, B. Vasić, "Instanton-based techniques for analysis and reduction of error floors of LDPC codes" (arXiv:0903.1624).** Instanton = most probable noise configuration causing decoder failure (minimum-L2 noise on AWGN; fewest flips on BSC); linked to pseudo-codewords; FER approximated by summing dominant instantons with multiplicities; Tanner code: (5,3) trapping sets dominate, BP effective distance 12.45; redesigned code removes them. Key refs inside: Richardson "Error floors of LDPC codes" (Allerton 2003); Stepanov, Chernyak, Chertkov, Vasić, PRL 2005; Feldman–Wainwright–Karger LP decoding; Vontobel–Koetter graph covers. [Link](https://arxiv.org/pdf/0903.1624)
- *Implication:* our "Hawking/Kramers" BER and the "geodesic action" refinement are, in substance, instanton analysis. The new piece we can add is curvature — the Kramers–Langer prefactor (§7 item 2) — and structured-noise instantons (§7 item 3).

### 10.7 Statistical physics of LDPC codes (pre-empts the "phase transition" finding)
- **R. Vicente, D. Saad, Y. Kabashima et al., "Statistical physics of irregular LDPC codes" (cond-mat/9908358)** and **"The statistical physics of regular LDPC error-correcting codes" (cond-mat/0003121).** Replica analysis of LDPC thresholds. [Irregular](https://arxiv.org/abs/cond-mat/9908358) · [Regular](https://arxiv.org/abs/cond-mat/0003121) · [Review: "LDPC codes: a statistical physics perspective"](https://research.aston.ac.uk/en/publications/low-density-parity-check-codes-a-statistical-physics-perspective/)
- Also: Sourlas (1989); Nishimori's book; Mézard & Montanari, *Information, Physics, and Computation* (verify).

### 10.8 Discrete curvature
- **E. Iceland, A. Samorodnitsky, "On coset leader graphs of structured linear codes" (arXiv:1802.01184).** Ollivier Ricci curvature of the coset-leader (Cayley) graph + discrete Bonnet–Myers bound code size for locally correctable/testable codes. Proof that discrete curvature already has a foothold in coding theory — for **bounds**, not decoding. [Link](https://arxiv.org/abs/1802.01184)
- **J. Topping et al., "Understanding over-squashing and bottlenecks on graphs via curvature", ICLR 2022 (arXiv:2111.14522).** Negatively curved edges cause bottlenecks in message passing; balanced Forman curvature; curvature-guided rewiring. BP is message passing, so this is the template for §7 item 4. [Link](https://arxiv.org/pdf/2111.14522)
- An extended search found **no** paper applying graph Ricci curvature to Tanner graphs to locate trapping sets (absence of evidence, not proof — confirm in Phase 1).

### 10.9 Riemannian metrics from energies (method support)
- **L. Bethune et al., "Follow the energy, find the path: Riemannian metrics from energy-based models" (arXiv:2505.18230).** Builds **conformal** metrics $(\alpha E+\beta)I$ or $(\alpha e^{-E}+\beta)^{-1}I$ from an EBM; geodesics by minimising kinetic energy with a learned interpolant. Notably avoids the Hessian-as-metric route, which sidesteps our positive-definiteness problem (§6.1). Useful if we keep any continuous signal-space geometry. [Link](https://arxiv.org/html/2505.18230v3)

### 10.10 Correlated / structured noise
- **P. S. Iyer, D. Sannamoth, J. Emerson, "Enhancing belief propagation decoders for correlated errors using cycle error characterization" (APS March Meeting 2026).** Virtual checks favouring correlated error patterns in quantum LDPC BP; up to ~3× logical improvement. Baseline for §7 item 3. [Link](https://meetings-archive.aps.org/smt/2026/mar-c09/5/)
- Classical coloured-noise ML detection (whitening, Mahalanobis distance) — any detection-theory text, e.g. Van Trees.

### 10.11 Tangential
- **S. Kanno, Y. Shimada, "Spectral codes: a geometric formalism for quantum error correction" (arXiv:2601.19765, 2026).** Codes as low-energy spectral projections of Dirac operators; spectral gap controls leakage. Interesting framing for a later QEC extension; not curvature-based. [Link](https://arxiv.org/html/2601.19765v1)
- Constant-curvature curve tube codes for analog error correction ([arXiv:2205.12332](https://arxiv.org/pdf/2205.12332)) — curvature used in *code construction* for analog codes; skim for terminology clashes.

### 10.12 Must-read list for Claude Code (in order)
1. Ikeda, Tanaka, Amari 2004 (IEEE TIT 50(6)).
2. Watanabe & Fukumizu (arXiv:1002.3307) + Heskes 2002.
3. Chilappagari, Chertkov, Stepanov, Vasić (arXiv:0903.1624).
4. Amador et al., Hybrid iteration control (stopping-rule baselines).
5. Topping et al. 2022 (curvature and message-passing bottlenecks).
6. Richardson 2000; Wadayama GDBF 2008; Vicente/Saad/Kabashima.

---

## 11. Planned but never done (abandoned threads)

Offers or plans made in the chats that were never carried out, usually because the conversation jumped to something more exciting. Ordered by usefulness for the paper.

### 11.1 FEC/EDAC — directly relevant
| # | Thread | Where | What stopped it |
|---|---|---|---|
| A1 | **Artefact test of −0.988**: re-run $G$ vs $T$ with Lorentzian, compact-support and anisotropic wells | Chat 3, proposed twice (once as the "honest steer" to do first) | Matej picked the Schwarzschild idea instead; never revisited |
| A2 | **Geodesic-action BER vs union bound vs simulated BER** on one plot, esp. at low SNR | Chat 3 | Conversation jumped to Hawking/Thorne. Now maps onto instanton + Kramers–Langer prefactor (§7 item 2) |
| A3 | **Local curvature along the BP trajectory at LDPC scale** (early termination / scheduling) | Chat 3 | Never started. Now the lead hypothesis (§7 item 1) |
| A4 | **Fisher-Rao as a design objective**: build a Hamming-optimised and a Fisher-Rao-optimised code, compare BER on a non-AWGN channel | Chats 2 and 3 | Never built. Note §6.3: on equal-variance Gaussian channels FR distance is monotone in Hamming distance, so this only makes sense on fading/coloured channels |
| A5 | **Geodesic decoding as a real decoder**: integrate the geodesic equation and compare to min-sum / sum-product | Chat 3 | Never built. Literature (§10.4) suggests it would be a variant of gradient/proximal decoding |
| A6 | **Check DVB-S2 LDPC for exploitable "throats"** under its real channel impairments | Chat 3, closing remarks | Never started. Recast as structured-noise instantons (§7 item 3) |
| A7 | Original application list: curvature-aware LDPC design; SNR-adaptive coding; early termination via phase-transition detection; geometric decoder scheduling; unequal error protection from the Fisher metric; QEC via Fubini–Study | Chats 1–2 | None pursued |
| A8 | From Chat 1's "what would make this publishable": **connection to channel capacity**; a **geodesic optimality theorem** (are geodesics provably the optimal decoding paths?) | Chat 1 | Marked 🔲 unexplored |

### 11.2 Toy-playground experiments (Chat 3, "the toy has knobs")
- β sweep: watch the saddle ridge sharpen/soften as a change in causal structure. **Cheap and useful** — it also exposes the barrier-saturation problem in §6.2.
- Three codewords in a triangle: does the saddle network match a Voronoi tessellation or does curvature distort it?
- Negative well depth ("anti-codeword", forbidden symbol): naked-singularity-like region?
- Time-varying channel, β(τ): a non-stationary "cosmology of decoding".
- A code with a known bad pair: does the geometry find the weakness unprompted?
- Riemannian (non-Wick-rotated) single-well radial analysis: the "clean" Schwarzschild option (a) was skipped in favour of (b).

### 11.3 Neural-network threads (out of scope for the FEC paper)
- **First-order Hessian-vector-product geodesic step**, measured against plain gradient descent and Adam with the diagnostic gauge — offered twice, never built (conversation moved to GPU economics).
- **Real small open model + garden-path sentences**: looking for the saddle in a real reasoning trajectory. An install of a real model stack was attempted; the experiments that actually ran used small trained toy transformers.
- **Fully grokked model** to decide whether thought-space geometry appears once the algebra crystallises, or was task-specific all along — explicitly "left on the table".
- **"The forced-geodesic equations of thought-space"** as a written analytic section — offered for the compendium.
- Feature-sourced curvature test using the **Einstein tensor** rather than the metric (the run that was done correlated the metric with $T$; see §6.7).

---

## 12. Sprint 1 results — Tanner-graph curvature vs CCSDS (23 Oct 2026, 19:59–21:59 UTC)

**Lead attacked:** §7 item 4 (discrete Ricci curvature of the Tanner graph as a predictor of where BP fails), chosen as the most unusual lead; focused on the **CCSDS TC LDPC (128,64)** (cubesat uplink code) at Matej's request. Code, data and full ledger: `gr-fec.zip` → `results/LEDGER.md`.

### Verdict: mirage in the waterfall region; untested in the error floor
1. **One-step Ollivier curvature is blind (proved and verified).** For girth ≥ 6 it depends only on the two endpoint degrees: −0.5 on every edge of a (3,6) code; on CCSDS (128,64) exactly −0.5417 (degree 3–8 edges) and −0.675 (degree 5–8). Any curvature signal must be multi-scale.
2. **Per-bit prediction (random (3,6) codes, n=200):** multi-scale and resistance curvature predict which bits fail (ρ ≈ 0.3), but **no better than 6-cycle counts**; leave-one-code-out 0.32 vs 0.31. Little to nothing beyond cycles.
3. **CCSDS per-block test:** by cyclic symmetry the 128 bits form 8 equivalent classes. At 3 and 4 dB failures are large (≈13 wrong bits) and spread uniformly; no block differs from any other (ANOVA p 0.65–0.98).
4. **Design test (the prize):** 51 random 4-cycle-free lifts of the CCSDS protograph, FER at 4.0 dB.
   - Exploratory batch of 6 codes: mean 3-step curvature ranked FER at ρ = −0.89, looking spectacular.
   - **Pre-registered** (20:44 UTC) and tested on an independent batch of 23 codes: **ρ = +0.17, p = 0.78 → refuted.**
   - Cycle counts fail too (pooled |ρ| ≤ 0.1 for 6/8-cycles).
   - Reason: in the waterfall the lifts are nearly interchangeable (FER spread ×1.34, only 30–50 % of it real).
   - The real CCSDS code sits at the 35th percentile of random lifts at 4 dB.
5. **Lesson for the project:** the 6-point exploratory "discovery" is the same trap as the July "wormhole". Pre-registration plus an independent batch caught it within 30 minutes. Make this standard practice (§0 rule 1).

6. **5 dB re-simulation (exploratory):** the 23 confirmation codes at 5 dB (FER ≈ 6 × 10⁻⁵, ≈ 50 failures each) are too few to resolve differences per code, and failures are still large (median 12 bits): **5 dB is still waterfall for this short code.** But code rankings at 4 and 5 dB agree (ρ = +0.40, p = 0.03): real, persistent code differences exist, and curvature doesn't track them (ρ = −0.07).

### What this does NOT rule out
Structure-dependent differences between codes live in the **error floor** (trapping sets, FER ≪ 10⁻⁵), not the waterfall. The floor region was not reached (the 5 dB CCSDS run was stopped unfinished). A fair test of curvature-for-design therefore needs one of:
- **Importance sampling** or instanton-based FER estimation (§10.6) to reach FER 10⁻⁶–10⁻⁹ in reasonable time.
- **BSC + Gallager-B / min-sum** decoders, where trapping sets dominate at much higher error rates.
- **Codes with known trapping-set enumerations** (Tanner (155,64,20) and its (5,3) sets) as ground truth, testing whether *set-level* (not node-level) curvature flags them.

The baseline to beat in any such test is the cycle/ACE/trapping-set machinery, not just raw cycle counts.

### Recommended next step
§7 item 1 (Bethe-Hessian curvature as an online stopping signal) has a cheap, crisp referee (iterations saved, undetected errors) and doesn't need the floor region. It is now the better next sprint. Item 4 should only be revisited with an importance-sampling FER estimator in place.

---

## 13. Sprint 2 results (24 Oct 2026, 04:24–05:24 UTC)

Two questions, both on firm ground now. Full numbers: `results/LEDGER.md` rows L9–L15; pre-registrations `results/PREREG2–4.md`.

### 13.1 First confirmed positive result: a curvature-linked early-warning signal for the CCSDS decoder
**Idea:** BP is an iterated map on messages. The local stretching factor of that map (spectral radius of its Jacobian, estimated by finite-difference Jacobian–vector products and power iteration) is the stability quantity that Heskes and Watanabe–Fukumizu tie to the curvature of the Bethe free energy (§10.3).
**Test:** CCSDS TC LDPC (128,64), sum-product BP, 50 iterations, AWGN at 3.0 and 3.5 dB. Among frames still unconverged at iteration 10, predict "will never converge". Baseline: syndrome weight, its trend, mean |LLR| (CMM-like), its trend (VNR-like), bit flips, message-change norm and ratio, **plus the full per-iteration history** of syndrome, |LLR| and message change.
**Discipline:** exploratory data → pre-registration → models frozen → fresh frames.

| Pre-registration | What | Outcome |
|---|---|---|
| PREREG2 | Warm-started probe every iteration (≈ 3× compute) | ✅ AUC +0.039 (3.0 dB, CI [+0.028, +0.049]), +0.035 (3.5 dB, CI [+0.018, +0.054]); best single predictor |
| PREREG3 | Cheap: 5 probes once at iteration 10 | ❌ +0.016: real (CI excludes 0) but below pre-set bar 0.019 |
| PREREG4 | Cheap: 8 probes once at iteration 10, active frames only | ✅ +0.032 (CI [+0.023, +0.042]) and +0.037 (CI [+0.025, +0.049]) on brand-new frames; overhead 8–12 % of decoder work |

**What it means:** the geometry of the decoder's own update map carries information about failure that the standard stopping statistics — even their whole trajectory — do not. This is the first result in the project that survived a pre-registered, out-of-sample test.
**What it does not (yet) mean:** turned into a hard stopping rule at a 1 % FER-loss budget, iteration savings are small and noisy. An AUC gain of ~0.035 is modest.
**Dynamics finding (corrects an earlier guess):** 90 % of failing frames are still flipping bits at iteration 50 — wandering near marginal stability (stretching ≈ 1.08) — not sitting in a stable trap. Slow-but-successful frames stretch more strongly (≈ 1.3).

**Next steps for this lead (in order):**
1. Use it where a modest AUC gain is valuable: **hand predicted failures early to a second-stage decoder** (ordered-statistics decoding is realistic at n = 128), or request retransmission early (HARQ / uplink command re-send). Measure FER and latency, not only iterations.
2. Try a better estimator: Lanczos/Arnoldi on the BP Jacobian (complex eigenvalues = oscillation), or the Bethe-Hessian determinant via the edge-zeta formula (§10.3).
3. Replicate on a second code (AR4JA or CCSDS C2) and on min-sum, the decoder used in hardware.
4. Literature check — **first pass done 24 Oct; adjacent prior art exists, the exact use not yet found:**
   - Kocarev, Lehmann, Maggio, Scanavino, Tasev, Vardy, "Nonlinear dynamics of iterative decoding systems: analysis and applications", IEEE Trans. Inf. Theory 52(4):1366–1384, 2006 — iterative decoders as dynamical systems (fixed points, oscillation, bifurcation, chaos); controls transient chaos in turbo decoding and proposes a **turbo stopping criterion** (abstract suggests it is entropy-based; check full text). [Link](https://researchportal.ip-paris.fr/en/publications/nonlinear-dynamics-of-iterative-decoding-systems-analysis-and-app/)
   - Zheng, Lau, Tse, Wong, "Study of bifurcation behavior of LDPC decoders", Int. J. Bifurcation and Chaos 16(11):3435–3449, 2006 — **derives the LDPC decoder Jacobian and its eigenvalues** (fold, flip, Neimark–Sacker bifurcations) vs SNR. [Link](https://research.polyu.edu.hk/en/publications/study-of-bifurcation-behavior-of-ldpc-decoders/)
   - "Evidence of chaos in the belief propagation for LDPC codes" (HAL hal-00736202) — likely Lyapunov exponents of BP; page was bot-blocked, read it. [Link](https://hal.archives-ouvertes.fr/hal-00736202)
   - Machine-learning early termination for turbo/LDPC decoders (Samsung, IEEE WCNC 2021) and an ML decoder-performance-prediction patent (US 11088712) — features not visible from abstracts; must check they don't already use Jacobian/Lyapunov features. [WCNC](https://research.samsung.com/research-papers/Machine-Learning-Based-Early-Termination-for-Turbo-and-LDPC-Decoders) · [Patent](https://patents.google.com/patent/US11088712)
   - **Positioning:** our quantity is a per-frame finite-time local Lyapunov (stretching) estimate. The Jacobian of LDPC decoders is studied in the dynamics literature as an *ensemble/SNR-level* object; we found no work using it **per frame as a failure predictor beyond standard stopping statistics, with a cheap checkpoint probe and a pre-registered out-of-sample test**. That is the candidate contribution — conditional on the three full texts above.

### 13.2 The −0.988 "Einstein equation" is a symmetry artefact (oldest open question, settled)
Exact autodiff Riemann/Ricci/Einstein tensors (JAX, float64; validated on S² and S³) for E(x) = −Σ f(|x−c|²) on Hamming(7,4), with Matej's requested well shapes:
Gaussian, Lorentzian, exponential, compact-support bump, δ′-like dipole, δ″-like (Laplacian of Gaussian, "Mexican hat"), δ⁗-like (bi-Laplacian of Gaussian), and ever-narrower Gaussians (β = 16, 64) as the delta-function limit.

- At midpoints between neighbouring codewords, T is exactly the pair-axis tensor aaᵀ (corr 1.000 for β ≥ 4), and **corr(G, T) equals corr(G, aaᵀ)** for every shape (β = 4: Gaussian −0.942 vs −0.943; Lorentzian −0.981 vs −0.981; δ⁗-like −0.999 vs −0.999). The "Einstein equation" is the reflection symmetry of the midpoint.
- The sign of "κ" flips with β and shape (Gaussian: +0.62, −0.48, −0.94, +1.00 at β = 1, 2, 4, 64). A law can't do that.
- Taking T from a different midpoint kills the correlation (≈ 0) — consistent with symmetry, not with matter sourcing curvature.
- **Delta wells:** a true delta (or any derivative of one) has zero Hessian everywhere except at the codeword, so the metric is identically zero and curvature is undefined. The narrow-Gaussian limit still only reproduces the symmetry artefact. The δ′ dipole breaks the symmetry: corr = 0, metric near-singular.
- Figure: `results/e06_symmetry_artefact.png`.

**Implication for the paper:** drop "Einstein equation for codes" as a claim. Keep it, if at all, as a cautionary example in a methods/limitations section. The geometric content worth publishing is in the *decoder dynamics* (13.1), not in a static code-space metric.

---

## 14. Sprint 3 results — thermodynamics meets the decoder (24 Oct 2026, 05:01–05:25 UTC)

**Hunch tested (Matej):** bring statistical physics in. **FEC translation:** the decoder's "temperature" is the noise level it *assumes*, i.e. how strongly it scales the channel LLRs (α = true σ² / assumed σ²). α = 1 is the **Nishimori line** (correct SNR estimate); α > 1 = overconfident ("cold", also what un-normalised min-sum does); α < 1 = timid ("hot").

| Pre-registration | Prediction | Outcome |
|---|---|---|
| PREREG5 (3.0 dB) | Colder decoder → more chaotic failures; hotter → frozen failures; FER best near α = 1 | ✅ all three: Spearman +0.91 (p = 0.0002); frozen share 90 % → 0 %; oscillating 3 % → 100 %; FER minimum at α = 1.0 |
| PREREG6 (2.5, 3.5 dB) | Chaos onset (FTLE = 0) lies at the Nishimori line, α ∈ [0.8, 1.25] | ❌ α_c = 1.01, 0.87 (3.0 dB, exploratory), 0.76: drifts hot with SNR. Phase trends and Nishimori near-optimality replicate (✅ S7, S8) |

**What is now established on CCSDS (128,64):** belief propagation has a **two-phase failure structure** controlled by LLR scaling — a *frozen* phase (wrong fixed points, negative Lyapunov exponent) when the decoder underestimates SNR and a *chaotic* phase (perpetual oscillation, positive Lyapunov exponent) when it overestimates — with the error-rate optimum at the matched (Nishimori) scaling and a broad, flat bottom (±10 % costs ≲ 5 % FER).
**What failed:** chaos onset is not pinned to the Nishimori line; it moves with SNR. So "tune LLR scaling until failing frames sit at zero chaos" is *not* a correct self-calibration rule as stated.
**Engineering reading (diagnostic, still useful):** the *type* of failure tells you which way your SNR estimate is off — if failures are frozen, the decoder is too timid; if they are 100 % oscillating at high FTLE, it is likely overconfident. A calibrated thermostat would need the α_c(SNR) relation (or a different statistic) — an open, testable question.

**Prior art to position against:** Gounai & Ohtsuki, "Decoding algorithms based on oscillation for LDPC codes", IEICE Trans. Fundamentals E88-A(8):2216–2226, 2005 (oscillation causes errors; oscillation-aware damping and OSD) [link](https://keio.elsevierpure.com/en/publications/decoding-algorithms-based-on-oscillation-for-low-density-parity-c/); Cordova et al., "On LLR mismatch in BP decoding of overcomplete QLDPC codes", arXiv:2603.04991, 2026 (FER flat over wide LLR mismatch) [link](https://arxiv.org/abs/2603.04991v1); finite-temperature / Nishimori decoding in the statistical physics of codes (Kabashima, Saad, Sourlas; e.g. [cond-mat/9904342](https://arxiv.org/pdf/cond-mat/9904342)); NASA combining-ratio estimation for LDPC (Mahmoud & Hi, 2012) estimates scaling from channel samples, not decoder behaviour [link](https://ntrs.nasa.gov/citations/20120009160).

### 14.1 The physics ↔ FEC map (only what has been measured)
| Physics | FEC meaning | Status |
|---|---|---|
| Temperature | Noise variance σ² = LLR scaling | Measured |
| Nishimori line | Correct SNR estimate / LLR scaling | Near-optimal (L17) |
| Too cold / too hot | SNR over- / under-estimated | Measured |
| Frozen phase | BP stuck at a wrong non-codeword fixed point | L16 |
| Chaotic phase | BP oscillation (bits flip forever) | L16 |
| Lyapunov exponent (FTLE) | Per-frame amplification of message perturbations | Predicts failure (L9, L11) |
| Entropy | Total soft-output uncertainty (bits/frame) | L19 |
| Mass | Prior probability of a codeword | Framing |
| Field spread | Channel noise (likelihood width) | Framing |
| Well | Likelihood basin | Faithful |
| Horizon | Decision boundary | Faithful |
| Phase transition | Waterfall / threshold | Known |
| Light cone | Computation tree; girth = when a bit's information meets itself | Known framing |
| Einstein equation | — (symmetry artefact) | Refuted (L14) |
| Fluid / Fokker–Planck / JKO | Ensemble of received words draining into codeword basins | Not yet tested |

---

## 15. Sprint 4 — wild hunches (24 Oct 2026, 06:09 UTC onwards)

Brief: "what is so crazy that no one thought of it?" Each idea was mapped to FEC, screened, and only promising ones were pre-registered (PREREG7–9). Ledger rows L20–L29.

### 15.1 Scoreboard
| Idea (physics source) | FEC translation | Verdict |
|---|---|---|
| Temperature diversity (stat. mech.) | Decode with a hot and a cold decoder | ❌ = more iterations; failures 91–97 % shared |
| Chaos points at the error (Lyapunov vector) | Use the dominant stretching direction to pick bits to guess | ❌ chance level (AUC 0.49) |
| Tempering steel (anneal / quench / cycles) | Per-iteration LLR-scaling schedules | ❌ loses to plain damping; cycles hurt |
| Generalised Born rule / Wick rotation | Complex-temperature BP (evidence with phases) | ❌ any phase strictly hurts; decoding selects real positive weights |
| **Wolfram multiway / causal invariance; Sorkin causal sets** | **Random check-update order, redrawn every iteration** | ✅ **confirmed** (p ≤ 1e-11 vs fixed order at 3.0/3.5 dB); 23–55 % fewer frame errors than flooding at no extra cost |
| Feedback / servo loops (thermostat) | Per-frame control of decoder temperature | Heating oscillating frames ❌ always hurts; **anti-freeze servo ✅ confirmed but small** (−1.6 to −4.1 % FER, almost no losses) |
| Langevin thermal kicks | Noise injected into undecided frames | ❌ |
| Time crystal (average over the oscillation) | Decide failed frames from cycle-averaged beliefs | ❌ (exploratory "success" was my bug — caught by PREREG9) |
| Improved process (all lessons) | Random-order BP + oscillation-guided guess-and-redecode | ✅ 2.7× / 4.5× / 10× lower FER than flooding (3.0/3.5/4.0 dB) — **but OSD fallback is ~10× better again (known)** |

### 15.2 The one physics insight that survived everything
**Oscillation is search, freezing is death.** Every attempt to calm an oscillating frame (heating the channel, softening messages, damping inside layered decoding, large noise) lost frames that would have converged. Everything that *added exploration without destroying information* helped: random update order (big), re-melting frozen frames (small), a second random-order attempt (~9 % of failures). In statistical-physics language: at the matched temperature the decoder's failures are a glassy *search* problem, not an instability to suppress.

### 15.3 Practical bottom line for CCSDS (128,64) (relevant to satellite work)
- Belief-propagation-family decoders sit **10–100× above near-ML** frame error rates for this short code. The known remedy is **BP followed by ordered-statistics (MRB/OSD) decoding** on failures — Baldi, Maturo, Paolini, Chiaraluce, "On the use of ordered statistics decoders for low-density parity-check codes in space telecommand links", EURASIP J. Wireless Commun. Netw. 2016, doi:10.1186/s13638-016-0769-z.
- **What we can add to that known hybrid:** random-order layered BP as the first stage reduces how often the expensive OSD must run by **24 / 42 / 55 %** (3.0 / 3.5 / 4.0 dB) versus flooding, at equal BP cost — less onboard power/latency. Worth a proper study (fixed-point arithmetic, min-sum, hardware cost of random order).
- Novelty of random per-iteration ordering is **unverified**: dynamic/informed schedules (Casado et al., RBP), shuffled BP (Zhang & Fossorier) exist; a search found no paper redrawing a random order every iteration, but this needs a careful check.

### 15.4 Matej's quantum-gravity list, mapped honestly to FEC
| Programme | Best FEC mapping | Status |
|---|---|---|
| Wolfram physics (multiway systems, causal invariance) | BP is **not** causally invariant: update order changes the outcome. Flooding (the order-independent decoder) is the *worst*; random order is best | **Tested ✅** (L24) |
| Sorkin: causal sets | Random ("sprinkled") ordering of update events instead of a fixed clock | Same experiment as above — **tested ✅** |
| Sorkin: generalised measure / higher-order interference; generalised Born rule | (a) Born exponent ↔ LLR scaling (temperature): optimum at the standard rule (L17). (b) Complex amplitudes in BP: **tested ❌** (L23). (c) Interference hierarchy ↔ loop-series corrections to BP (Chertkov–Chernyak loop calculus, known) | Partly tested |
| Turok (Boyle–Finn–Turok CPT-symmetric universe) | "Mirror universe" = decode the CPT image (negated LLRs). CCSDS (128,64) contains the all-ones word (every check has even weight), so the mirror decode is exactly symmetric — **provably no new information** | Closed by symmetry |
| Loop quantum gravity (spin networks / spin foams = sums over histories of graphs) | BP is exact on the universal cover of the Tanner graph; finite **graph covers** give pseudo-codewords (Vontobel–Koetter) — the "spin foam" of decoding | Known framing, untested here |
| String / M-theory dualities | **MacWilliams identity** (code ↔ dual code weight enumerators) is a Fourier/T-duality; dual-domain BP (Forney normal graphs) and BEC duality (EXIT area theorem) | Known framing |
| AdS/CFT (holography) | Spacetime as a quantum error-correcting code (Almheiri–Dong–Harlow 2015) — the reverse direction of this project | Context |

### 15.5 Next experiments (ranked)
1. **Random-order BP + OSD hybrid on CCSDS (128,64) and (256,128)/(512,256)**: OSD-call rate, total complexity, FER/latency; min-sum and fixed-point versions — the satellite-relevant study.
2. Check novelty of per-iteration random scheduling carefully (Casado et al. 2010 RBP; Zhang & Fossorier shuffled BP; stochastic schedules).
3. Error-floor region (importance sampling) for the hybrid.
4. Optional theory: why does random order help? Hypothesis — it breaks the periodic orbits (limit cycles) that fixed schedules lock failing frames into; test via period detection of failing-frame trajectories under fixed vs random order.

### 15.6 Sprint 4, second half (06:45–07:05 UTC) — generality and the hybrid
- **Why random order helps is still unknown.** The limit-cycle explanation is refuted: true periodic orbits are rare under every schedule (L30).
- **On CCSDS (128,64) it's robust:** random-per-iteration order beats all 20 fixed orders tried; randomising just the 4 hardware layers keeps most of the gain (L31).
- **It does not generalise to AR4JA (8192,4096):** every randomised variant lost to the designed natural order (L32). Treat L24 as code-specific until tested on more short codes.
- **Early hand-off to OSD** saves 30–40 % of BP work at small FER cost (L33). The geometric failure predictor adds ~nothing there, because OSD is strong enough that hand-off selection hardly matters (L34).

### 15.7 Physics narrative that the data now supports
- **BP = classical dynamics on the code landscape:** it rolls downhill and, when it fails, either freezes (hot) or wanders chaotically (cold); the matched temperature is optimal (L16–L17).
- **OSD = tunnelling:** it jumps straight through barriers by flipping a few bits at once in the most reliable basis. That's why it beats every "keep rolling more cleverly" idea by ~10×. The instanton picture of §10.6 is the bridge.
- **Practical lesson for short space codes:** spend ingenuity on *when and how to tunnel* (OSD order, hand-off point, OSD call rate), not on making the rolling smarter.

### 15.8 What to tell a satellite-comms team (CCSDS TC (128,64))
1. Use **BP + OSD-2 fallback** (Baldi et al. 2016): ~10–100× lower FER than BP alone at 3–4 dB.
2. **Hand off at ~20 iterations**, not 50: ~30 % less BP work, FER essentially unchanged (L33).
3. Consider **random (or per-layer random) check order** in the BP stage for this code: 24–55 % fewer OSD calls (L29), same BP cost — verify in fixed-point/min-sum first, and check novelty.
4. If the receiver's SNR estimate is uncertain, the **failure type** diagnoses the error direction: frozen failures = LLRs too timid, all-oscillating = too confident (L16).
5. **Order the OSD basis by a blend of channel and BP reliabilities** (|LLR| + 0.25·|BP posterior|), but pick the winning candidate by the channel likelihood: residual FER −37 % (3.0 dB) and −43 % (3.5 dB) at the same OSD cost (L36, pre-registered). Closely related published ideas exist (complementary-LLR OSD, IEEE 2023; learned DIA ordering, Li & Yu) — treat as a tuning tip, not a novelty.
6. Cheap OSD-0/1 benefit from BP-posterior ordering; OSD-2 needs the channel likelihood as its metric (L35).

**Best measured CCSDS (128,64) chain (FER at 3.0 / 3.5 dB):** random-order BP (20 it) → blended-reliability OSD-2 = **0.23 % / 0.032 %**, vs flooding BP (50 it) 6.1 % / 1.6 % — about **26× / 50× fewer frame errors**.

---

## 16. Sprint 5 (24 Oct 2026, 07:18–08:15 UTC) — safety, the ML limit, and why things failed

### 16.1 The hidden cost, and the gravity gate (most practically important result)
- BP failures are **detected** (unsatisfied parity checks → retransmit). OSD always outputs a codeword, so its mistakes are **undetected** (L37): UER 2.0e-3 / 3.4e-4 at 3.0 / 3.5 dB. For telecommand, an executed wrong command is worse than a resend. (CCSDS TC transfer frames can carry a CRC-16 that catches most of these; the gate still avoids wasted CRC failures.)
- **Gravity gate (PREREG11 ✓):** accept OSD's codeword only if its well is clearly deeper than the runner-up's, relative to how far it had to tunnel (r = gap/action ≥ 0.5). Fresh frames: **0 of 178 and 0 of 63 wrong answers accepted**, retransmissions **5.6× and 10× lower** than BP alone. This is Forney's 1968 erasure-decoding principle in landscape language; the ratio form and its use in BP+OSD for CCSDS TC appear new to us but need a literature check.

### 16.2 Reaching the ML limit
- ML FER bracketed by OSD-3 certificates (L39). **Multiverse OSD (PREREG12 ✓):** run cheap OSD-2 from several orderings — the blend and BP's posterior at **iteration 5** — and keep the likeliest. Two universes cut residual FER 2.7–3.3×; six reach **0.003 % at 3.5 dB, inside the ML bracket**, at ~29 % of OSD-3's cost. With the gate: 0 wrong accepted, retransmissions 0.088× BP-only at 3.5 dB.
- Why it works (F10): OSD-2 can only fix ≤ 2 errors in its "most reliable basis". The early-iteration universe has a different basis: 18 of the 27 frames unreachable by the blend are reachable there.

### 16.3 A failed prediction turned into a law (PREREG13 ✓)
Chaos onset is **not** at the Nishimori line (PREREG6 ✗); it sits at a fixed *absolute* LLR amplitude: α_c × Eb/N0 ≈ 1.74. Predicted α_c(4.0 dB) = 0.693, measured 0.667. Still post-hoc in origin and drifting slightly (1.80 → 1.68); a third SNR outside the fitting range passed.

### 16.4 Failure dossier — every dead idea, demystified
| Dead idea | Why it failed (tested, not assumed) |
|---|---|
| "Einstein equation" G ≈ κT (−0.988) | Midpoint reflection symmetry: T ≡ pair-axis tensor; corr(G,T) = corr(G, aaᵀ) for every well shape; κ flips sign with β (L14) |
| Antipodal "wormholes" | Guaranteed by the {4, 8} distance spectrum; barriers saturated at β = 4; aimed noise opens *any* pair; mismatched decoder (§6.2) |
| Fisher–Rao "beats" Hamming | For BPSK/AWGN d_FR ∝ √d_H — same ranking (§6.3) |
| One-step Ollivier curvature | Depends only on endpoint degrees in girth ≥ 6 graphs (L1) |
| Curvature predicting failing bits / ranking codes | Waterfall failures are global avalanches (≈ 14 wrong bits over 7 of 8 blocks); whether a frame fails is decided by the channel error count; code variants differ ≤ 34 % (F1, L6) |
| Temperature diversity | Channel decides: FER 0.1 % (≤ 8 channel errors) → 62 % (≥ 18); decoder temperature cannot change it — 98 % of failures shared (F2) |
| Lyapunov vector locates errors | It lives on degree-5 hub nodes (74 % vs 50 %), the code's amplifiers; wrong-bit rate at chance (F7) |
| Tempering cycles / jitter | Each temperature change re-weights channel vs accumulated messages, quenching frames between frozen and chaotic phases; FER worsens monotonically with cycle speed and amplitude (F12) |
| Complex ("Wick-rotated") temperature | Imaginary parts never decay (channel keeps injecting phase); 4–16 % of messages near tanh poles; frames stall (F5) |
| Heating oscillating frames / damping in layered BP / Langevin kicks | Oscillation is productive search: 39 % of frames undecided at it. 10 converge later; suppressing it kills them (F3) |
| Time-crystal averaging | Oscillation is among *wrong* states: cycle-average 13 vs final 14 wrong bits (F4); exploratory success was an RNG-reuse bug |
| Chaos onset at the Nishimori line | Onset tracks absolute LLR amplitude instead (L43) |
| Multipoles (redundant checks) in-graph | 0 → 6514 four-cycles (F6) |
| Coulomb repulsion | Residual errors are basis-reach failures (≥ 3 errors in the MRB); repulsion doesn't change the basis (F10) |
| Lyapunov predictor inside BP+OSD | OSD rescues 96 % of handed-over frames; selection barely matters (L34) |
| Random order on AR4JA | It converges slower (26.4 vs 23.1 it.); a 50-iteration cap truncates it; gap vanishes at 100–150 it. Random order trades speed for exploration (L45) |
| Limit-cycle explanation of random order | Periodic orbits rare (L30) — superseded by the speed-vs-exploration account |

**Unifying lesson:** in the waterfall, *the channel decides which frames are hard*; decoders differ only in (a) how fast they converge and (b) how far they can search. Physics-flavoured tricks helped exactly when they added *reach* (random order on a search-limited code; multiverse OSD; tunnelling) and hurt when they removed information (phases, noise, heating) or speed (random order on an iteration-limited code).

### 16.5 Recommended CCSDS TC (128,64) receiver (all steps confirmed on fresh frames)
Random-order layered BP (≤ 20 it.) → on failure, multiverse OSD-2 (blend + posterior at it. 5) → **gravity gate** (accept only r ≥ 0.5, else request retransmission).
Retransmissions at 3.0 / 3.5 dB: 0.90 % / 0.12 % — **6.7× / 13× fewer than standard flooding BP** (6.07 % / 1.63 %) and 6× / 11× fewer than random-order BP alone — with no observed increase in undetected errors (Monte-Carlo-limited to ~1e-5). Next: fixed-point min-sum version; importance sampling to verify UER at 1e-7…1e-9; literature check on Forney-style gating in BP+OSD.

### 16.6 Hardware reality check (08:11–08:18 UTC)
- **Random update order does not survive hardware-style decoding** (normalised min-sum, 6-bit LLRs and messages): rescued ≈ lost at every normalisation tried (L46). Drop it from the hardware recommendation.
- **The OSD chain and the gravity gate do survive (PREREG14 ✓)**: with a min-sum 6-bit front end (fixed order, 20 iterations), MV2 OSD-2 plus the gate accepted **0 wrong answers** at 3.0 and 3.5 dB on fresh frames, while retransmissions fell to **0.85 % and 0.145 %** (7.6× and 11.5× fewer than the min-sum decoder alone) (L47).

### 16.7 Final recommended CCSDS TC (128,64) receiver (supersedes 16.5)
**6-bit LLRs → normalised min-sum layered BP (×0.75, ≤ 20 iterations, any fixed order) → on failure: multiverse OSD-2 (blended ordering + BP posterior at iteration 5; candidates judged by the channel metric) → gravity gate (accept only if gap/action ≥ 0.5, else request retransmission).**
Measured on fresh frames: retransmissions ÷7.6 (3.0 dB) and ÷11.5 (3.5 dB) vs the min-sum decoder alone, with no accepted wrong answers in 96 000 + 192 000 frames. Open items before any flight use: importance-sampling estimate of UER down to 1e-9; OSD-2 fixed-point/latency budget (2 × 2081 re-encodings per failed frame, ~1–2 % of frames); literature check on Forney-style gating and multi-ordering OSD for CCSDS TC; repeat on the (256,128) and (512,256) CCSDS TC codes.

---

## 17. Literature check of the candidate contributions (24 Oct 2026)

Checked from abstracts, publisher pages and secondary sources; full texts still to be read where marked.

| Our result | Closest prior art found | Verdict |
|---|---|---|
| **Gravity gate** (accept OSD output only if gap/action ≥ 0.5) | **Gap** = Forney's likelihood-ratio erasure rule (best vs runner-up codeword): G. D. Forney, "Exponential error bounds for erasure, list, and decision feedback schemes", IEEE Trans. Inf. Theory, March 1968 (vol. IT-14 no. 2 pp. 206–220 — volume/pages to verify on IEEE Xplore, arnumber 1054129). **Action** = D_x, the sum of \|LLR\| where the output differs from the received hard decision, used as a gate for accepting BP outputs by Zhang, Yue, Li & Vucetic, "Efficient near maximum-likelihood reliability-based decoding for short LDPC codes" (arXiv:2306.00443, 2023). | **Both ingredients known.** Their ratio, and its use to gate OSD outputs in BP+OSD for space telecommand, was not found — a modest, engineering-level contribution at most. |
| Undetected-error concern for LDPC+OSD in telecommand | Baldi, Maturo, Ricciutelli, Chiaraluce, "On the error detection capability of combined LDPC and CRC codes for space telecommand transmissions" (arXiv:1605.08936; IEEE ISCC 2016): estimates UER with a CRC on short LDPC decoder outputs and concludes the requirements are met. | **Known problem, known mitigation (CRC).** Our gate is complementary: it turns would-be CRC failures into early erasures. Read the full text for their UER targets and whether OSD was included. |
| **Multiverse OSD** (OSD-2 from several orderings incl. BP posterior at an early iteration) | Rosseel, Mannoni, Savin & Fijalkow, "Sets of complementary LLRs to improve OSD post-processing of BP decoding", ISTC 2023: multiple OSD runs on selected LLR sets including **per-iteration a posteriori LLRs** and learned accumulated LLRs; Rosseel et al., "Decoding short LDPC codes via BP-RNN diversity and reliability-based post-processing" (arXiv:2206.12150, 2022). | **Known.** Our two-set choice (blend + iteration-5 posterior) for CCSDS TC (128,64) is an instance, not a new method. |
| Blended channel + posterior ordering | Accumulated / weighted per-iteration LLRs for OSD (Rosseel et al. 2023, single-neuron weighted sum; prior work cited by Zhang et al. 2023); learned DIA ordering (Li & Yu, arXiv:2307.06575, 2404.14165). | **Known in spirit.** |
| Per-frame Jacobian/Lyapunov failure predictor (PREREG2–4) | Tasev, Kocarev, Lehmann & Maggio, "Nonlinear phenomena in the turbo decoding algorithm", NOLTA 2002: largest Lyapunov exponent **per frame** vs SNR for turbo decoding, chaos and chaotic transients — but no stopping/failure rule. Kocarev et al., IEEE TIT 52(4), 2006: turbo stopping criterion (basis not confirmed; average-entropy criteria exist in the turbo literature). Zheng, Lau, Tse & Wong, IJBC 2006: LDPC decoder Jacobian eigenvalues and bifurcations. | **Use as an LDPC failure predictor beyond standard statistics not found.** Practical value weak (L34). Read Kocarev 2006 full text before claiming. |
| **Frozen-vs-chaotic failure phases under LLR scaling, and chaos-onset law α_c·Eb/N0 ≈ 1.74** | Schlegel & Zhang, "On the dynamics of the error floor behavior in (regular) LDPC codes" (arXiv:0902.1786): error-floor failures settle at wrong fixed points (different regime). LLR-saturation analyses (arXiv:1403.3678, 1412.8090). Oscillation-aware decoding (Gounai & Ohtsuki 2005). Patent US 8286048 scales LLRs from SNR/MCS, not from decoder behaviour. | **Not found — the most original physics-flavoured finding.** Needs a careful search of the statistical-physics-of-decoding literature (finite-temperature decoding, Nishimori-line studies) and replication on another code before claiming. |
| Random check order every iteration (sum-product) | Dynamic/informed scheduling (Casado et al.), shuffled BP (Zhang & Fossorier), dynamic layered scheduling (Jia et al. 2025). Per-iteration random order not found. | Unclaimed but **null in hardware-style decoding (L46)** — not worth pursuing. |

**TC frame CRC (fact check of my earlier claim):** TC Transfer Frames (CCSDS 232.0-B-4) can end with a 16-bit Frame Error Control Field using CRC-16-CCITT (generator x¹⁶ + x¹² + x⁵ + 1, register preset 0xFFFF), per an implementation citing §4.1.4.2. Its presence is a configuration choice in independent implementations (ocaml-tc "with or without the FECF"; Logtalk `HasFECF`; NASA F´ comment "optional 2 octets FECF", though its deframer always checks it). I could not open the standard itself (the ECSS mirror was down): **confirm in CCSDS 232.0-B-4 whether the FECF is mandatory or a managed parameter.**

**Consequence for the receiver recommendation:** if the FECF is present, the CRC already reduces undetected errors by ≈ 2⁻¹⁶ (Baldi et al. 2016 show LDPC + CRC meets TC requirements). The gravity gate then mainly converts CRC-failing OSD outputs into earlier erasures and adds margin; its value is largest on links without a FECF.

### 17.1 Revised publication outlook
- **Engineering note (most solid):** a measured, pre-registered CCSDS TC (128,64) receiver study — min-sum front end, multi-set OSD (known method) and a Forney/D_x-style gate with explicit undetected-error accounting — positioned against Baldi et al. 2016 and Rosseel et al. 2023.
- **Physics-of-decoding paper (most original, needs more work):** frozen vs chaotic failure phases controlled by LLR scaling, the chaos-onset amplitude law, and "oscillation is search" — replicate on 2–3 more codes and min-sum, then search the stat-phys literature thoroughly.

## 18. Sprint 6 — pushing the original findings (24 Oct 2026, from 08:33 UTC)

Goal: take the findings §17 marked as *not found in the literature* — the frozen/chaotic failure phases, the chaos-onset law and "oscillation is search" — and try to break them on new codes and on the hardware decoder. Four pre-registered tests (PREREG15–18), plus one post-hoc observation.

### 18.1 What happened
| Test | Prediction | Outcome |
|---|---|---|
| PREREG15 — onset law on (3,6) and (4,8) random codes, n = 200 | α_c = K/(Eb/N0), with K either constant or drifting linearly in dB | **Constant K fails on (3,6); drifting K passes on both** (0.661 measured vs 0.664 predicted; 0.709 on (4,8)). K is a property of the code: 1.68–1.80 CCSDS, 1.32–1.56 (3,6), 1.59–1.73 (4,8). |
| PREREG16 — min-sum has a temperature too | Normalisation β plays the role of α | **Pass.** β 0.4 → 1.0 takes failures from 82 % frozen to 100 % oscillating; FTLE −0.30 → +0.57. |
| PREREG17 — are failures stuck or slow? | Failing frames stop making progress | **Weak pass.** CCSDS failures are stuck (median progress ratio 1.000); AR4JA failures are partly still moving (11 % progressing; 25 % succeed by it. 100). |
| PREREG18 — edge-of-chaos optimum for min-sum | FER-optimal β lies above β_c, with positive FTLE | **Pass at 2.5 and 3.5 dB.** β_c ≈ 0.565 at both SNRs; β* = 0.8; FTLE at β* ≈ +0.34. |

### 18.2 The picture that emerges (figure `results/phase_collapse.png`)
1. **One phase diagram, many decoders.** Divide the temperature knob (α for sum-product, β for min-sum) by its own chaos onset and 13 sweeps — 3 codes, 2 decoders, 2.0–4.0 dB — fall almost onto one curve below the onset (FTLE spread SD ≤ 0.04), with the frozen→oscillating crossover at exactly 1. *FEC meaning:* every BP failure is either "frozen" (a confident wrong fixed point, like an error-floor trapping set) or "chaotic" (endless search), and one number per decoder says which regime you are in.
2. **Best decoding is slightly chaotic.** In 11 of 13 sweeps the lowest FER is on the chaotic side, at 1.06–1.42 × onset (the other two are at 0.96 and 0.98, within grid spacing). *FEC meaning:* a decoder should be tuned so that its failures *just* start to oscillate — it then keeps searching instead of locking in. This is a stat-phys reason for the familiar empirical normalisation factor ≈ 0.75–0.8 in min-sum.
3. **Min-sum and sum-product differ in one useful way.** Sum-product's onset moves with SNR (α_c ∝ 1/SNR, with a slowly drifting K); min-sum's onset is essentially fixed (β_c ≈ 0.565 at 2.5 and 3.5 dB). *Engineering meaning:* a fixed β ≈ 0.8 stays near-optimal across SNR, which is why fixed-normalisation hardware works; a sum-product decoder would need SNR-aware scaling.
4. **Short vs long codes fail differently.** CCSDS (128,64) failures are stuck — more iterations won't help, a post-processor (OSD) will. AR4JA (8192,4096) failures are partly slow — more iterations or a faster schedule will help. This also explains why random check order helped one and hurt the other (L45).

### 18.3 Honest status
- **Strengthened:** the phase picture (now 3 codes × 2 decoders) and the edge-of-chaos optimum.
- **Refined:** the onset law is not universal — K depends on the code and drifts with SNR. It is a *calibration* law, not a constant of nature.
- **Weakened / caveats:** all codes are short or mid-length with flooding/min-sum decoders; the collapse is post hoc; the β grid (0.1 steps) limits how precisely β* is located; random codes are n = 200 only. Not yet tested: an irregular standard code with sum-product at long length, or quantised decoders' onset.
- **Literature still to search before claiming:** "edge of chaos" and optimal inference (Nishimori temperature, finite-temperature decoding — Ruján 1993, Sourlas 1994, Nishimori 2001), the dynamics of iterative decoding (Richardson 2000 geometry of turbo decoding; Agrawal & Vardy 2001 "The turbo decoding algorithm and its phase trajectories"; Tasev/Kocarev 2002–2006), and the min-sum normalisation-factor literature (Chen & Fossorier 2002) — to check whether "optimum just above chaos onset" has been stated.

### 18.4 Revised paper outlook
§17.1 stands, with the physics paper now stronger: *"Frozen and chaotic failure phases of belief-propagation decoding: a temperature picture of LLR scaling and min-sum normalisation"*, with the collapse figure, the edge-of-chaos optimum and the short/long failure dichotomy as the three results. Next work: replicate on one more standard code (e.g. the CCSDS (256,128) TC code or a 5G NR short code), add one long-code sum-product sweep, then do the literature search above.

## 19. AI branch — "physics of thought" (24 Oct 2026)
Matej's ideas on AI (concept hypergraph as the abstract layer, hallucination = creativity, hyperwalls, complex-weighted
Feynman-like edges, quantumness and creativity) with mappings, verdicts, hypotheses H-AI1–H-AI8, prior art and the
experiments live in **`docs/ai_branch.md`**. Results so far: the gravity gate does *not* transfer to a sparse
associative memory (L53), because hallucinations there are mostly *novelty* (detected by fit) rather than
*confusion* (detected by gap); dense codes like CCSDS only have confusions (L54). The LLM gate (E31) was running and
the generation-phase test (E32) was ready when paused.
- **Worm agent (24 Oct, ~10:25–11:00 UTC):** a worm-level hypergraph agent on a simulated agar plate (`src/worm.py`)
  passes seven pre-registered C. elegans assays in v1, including thermotaxis memory and salt-avoidance learning with no
  assay-specific code (L57); the edge-of-chaos hunch does not hold at worm level (L58). Details in `docs/ai_branch.md` §6.4.
- **Worm sprint 2 (24 Oct, from 10:37 UTC):** noise helps only above the decision threshold (L61); concept nodes grown by
  a novelty gate pass 4/5 assays once there are two kinds of node, intensity and tuned (L62); complex edges are a compact
  Fourier code for timing — efficiency, not new capability (L63). Details in `docs/ai_branch.md` §6.6.
- **Worm v3 re-run (24 Oct, from 10:52 UTC):** with inner senses and hyperedges added, inner senses must act as gates,
  not predictors (L64–L65; v3b passes 4/5 with the sharpest starvation effect); sensor noise is only needed when the
  internal representation is rugged (L66); hunger gating learned but too weak to detect in a ceiling-limited test (L67).
- **Embodied worm (24 Oct, from 11:05 UTC):** strict World → Body (metabolism, sensory neurons) → Brain split; the brain
  sees only senses. v4b, with temporal integration, passes all five assays without cheats (L68–L69); hunger gating via
  hyperedges is suggestive but under threshold (L70). Details in `docs/ai_branch.md` §6.8.
- **LLM gate (finished ~11:20 UTC):** the gravity gate detects a small LLM's wrong factual answers best of the scores tried (AUROC 0.881 vs 0.858 sample agreement, 0.839 mean log-prob; L71).
- **A worm's life (24 Oct, from 12:02 UTC):** 48 h in a world with depleting food. With instinct inside the learner, learning
  made worms worse (they learned the food zone was "disappointing", L72). With instinct as a separate fixed pathway the
  damage disappears, but a well-fed worm in a rich world has nothing to learn (L73). Next: a harder world.
- **Harsh world (from 12:27 UTC):** learning finally beats instinct (energy 0.404 vs 0.277), but only edges out a random walker (0.369): the learner cannot override a fixed instinct and explores too little (L74).
- **Worm v6 (from 12:50 UTC):** learned trust in instinct (instinct × learned value, a hyperedge) lifts harsh-world energy
  to 0.61 (v5 0.40, random 0.37) with no harm in the rich world; curiosity as a novelty drive did not help (L75).
- **Paused for the day at ~13:15 UTC, 24 Oct 2026.** Start next time with `docs/ai_branch.md` §0 and §8.
- **Day-2 hour (24 Oct, 13:58–14:40 UTC):** v6 trust replicates and passes all lab exams (L76–L77); at Matej's request the
  brain's numbers were evolved instead of hand-picked — evolution rediscovered learned trust and beat the hand design in both
  worlds (L78). Paused again ~14:40 UTC; start with `docs/ai_branch.md` §0 and §8.
- **25 Oct (10:26–~13:00 UTC): organic brain v7** — architecture moved into the genome, worlds randomised. Fitness rose but an
  unbalanced world schedule let evolution memorise "go warm" and switch learning off; in never-seen worlds the learning-heavy
  e45 brains generalised best (L79–L82). Evolution and project-wide statistics added (`a50`, `a51`).
- **25 Oct (12:21–~13:55 UTC): organic v7b** — three balanced lives per genome; cold band improved (0.44 → 0.58) and the best
  warm-band worm emerged, but evolution switched learning off again; designed learning + evolved numbers (e45) still
  generalises best (L83–L84). Paused; next: fitness = minimum across lives, longer lives, learning that pays within a life.
- **25 Oct evening (20:41–23:15 UTC):** organic v7c with worst-of-three-lives fitness — best organic generalist (worst case
  0.69), learning stays on, learned trust in smell positive in 3/3 runs with learning (L85). Imagination as random walks on the
  concept graph: no reliable benefit after replication; it needs directed turns and a heading sense (L86).
- **25 Oct late evening (~22:55–23:40 UTC):** a body that can steer — steering towards the neighbouring concept with higher
  learned value (value-guided weathervaning) is a robust win in never-seen worlds (+0.17, +0.17, +0.24; L87–L88); multi-step
  imagination still weaker because noisy concept labels poison its transition model. Paused here.
- **~23:55 UTC:** stable concept labels did not rescue multi-step imagination (L89); greedy value-steering replicated a fourth
  time. Night run ends here; start next session with `docs/ai_branch.md` §0, §6.15–6.19 and §8.
- **~00:05 UTC 26 Oct:** relational-action imagination also fails (L90); six imagination variants never beat one-step greedy
  value-steering (replicated 5×). Night run ended; everything written down.
- **26 Oct (06:44–09:35 UTC):** principles synthesis (`docs/principles.md`); organic v8 with steering genes (worst case 0.55, L91);
  compiled brain 2.3× faster (L92); optimiser race — self-adaptive mutation + successive halving not better than the GA (L93).
- **26 Oct ~09:50 UTC:** handoff written (`docs/HANDOFF.md`, `CLAUDE.md`, `requirements.txt`) so a new Claude Code session can resume without the chat history.
