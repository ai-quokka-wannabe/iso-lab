# The Worm's Pace

The rule *The animal's pace* asks that a Program's reaction latencies, the time constants of its internal states
and the rhythm of its decisions match the worm's, each from a cited source. This note gathers what the lab has read
so far. It is a first pass: some latencies are given in the sources only as figures, which have not been read, and
those are listed as gaps rather than guessed.

The papers were read on 2026-10-14 in full on PubMed Central, through NCBI's BioC service and Europe PMC, unless a
line says otherwise. How sure the lab is: high for the quoted sentences and numbers, which are the papers' own;
lower for the paragraphs headed *For the lab*, which are the lab's reasoning. Only numbers stated in a paper's text
are used; none is read off a graph.

## Reaction Latencies

### Touch

[Liu et al. 2018](https://doi.org/10.7554/eLife.36419) (PMC6054533) recorded the responses of many freely moving
worms to plate taps and to optogenetic stimulation of the six gentle-touch neurons.

- For one analysis, comparing responses in two contexts against a mock stimulus, they counted the first transition
  "within 2 s after stimulus onset"; it is a window chosen for that comparison, not their general definition of a
  response.
- After a tap or a 1-second stimulation of the touch neurons, "[b]oth light and tap also evoked an increase in 'Turn'
  behavior that peaked 5 s post-stimulus".
- From their filters for reversals they suggest "the animal may integrate the sensory signal over approximately 3 s in
  determining to reverse"; this is their interpretation, worded as a suggestion.
- On the receptors themselves, summarising earlier work, they write that the currents evoked by a force "adapt with a
  timescale of a few tens of milliseconds".
- **At the receptor**: in the nose's OLL neurons, exposed by micro-dissection of the nose tip, a touch-evoked receptor
  current "had a very short latency (1.5 ± 0.3 ms, 12 μm displacement, n = 4)", and at 15 μm "the MRC latency
  approached 1 ms (0.8 ± 0.2 ms, n = 6)" ([Fan et al. 2021](https://doi.org/10.1007/s12264-021-00629-4), PMC8099987).
  This is a receptor's latency in a dissected preparation, not a behaviour's, and one of several of its kind: in the
  same kind of recording, nose touch evoked currents in the ASJ neurons "with a very short latency (3.35 ± 0.26 ms
  with a 15 μm displacement of the touch probe towards the worm's nose tip)" ([Zou et al.
  2025](https://doi.org/10.1038/s41467-025-56938-z), PMC11830769), and with a 15 μm displacement "the latency was 4.3±
  0.9 ms (n = 6) in the AMsh cell and 4.1± 1.1 ms (n = 6) in the ASH neuron" ([Ding et al.
  2015](https://doi.org/10.1371/journal.pone.0117114), PMC4325002). Touch reaches the nose's receptor neurons, and the
  glia beside them, within a few milliseconds.
- **Not found**: the latency from a touch to the start of a reversal, as a number in a paper's text. A scan of the
  first 15 open-access results of each of five Europe PMC queries on touch, tap and latency (45 papers; the queries
  are in the journal) found none; Liu et al. show such latencies in figures, which were not read.
- **Habituation lengthens the latency, for a nociceptor.** Ardiel et al. stimulated the polymodal nociceptor ASH with
  light, repeatedly: "the ∼1 sec increase in response latency by the end of 30 stimuli" ([Ardiel et al.
  2016](https://doi.org/10.1101/lm.041830.116), PMC5026203). Their 2017 paper (under *Gaps, to Be Read*) restates that
  result, "longer response latency", and its own controls show it in figures only. These are latencies to stimulating
  a neuron, not to touch.
- **Habituation to taps, at second-scale intervals.** A review from the Rankin laboratory, summarising Rankin et
  al. 1990, writes that reversal distance "decremented after repeating the stimulus 40 times at interstimulus
  intervals (ISIs) of 10 s or 60 s. This response decrement recovered to the baseline after a few minutes"
  ([Bozorgmehr et al. 2013](https://doi.org/10.3389/fphys.2013.00088), PMC3750945). Longer memories follow training:
  after spaced training, memory was tested "24 h later", and after massed training "intermediate term memory lasts 16 but
  not 24 h". These are secondary statements and protocol timescales, not fitted time constants.

### Noxious Heat

[Mohammadi et al. 2013](https://doi.org/10.1186/1471-2202-14-66) (PMC3703451) heated a spot on the worm with a laser
pulse of 133 ms.

- "the animal's reaction time does in fact vary inversely with stimulus amplitude", and "the reaction time is
  proportional to the logarithm of the stimuli strength".
- At the head the typical response is "a reversal, followed by an omega turn, then a recommencement of forward
  motion".
- **Not found yet**: the reaction times in seconds, which are in their Figure 2c and not in the text. The figure was
  not read (PubMed Central's page asks for a CAPTCHA, which the lab does not get round, and Europe PMC did not serve
  the image).

[Wittenburg and Baumeister 1999](https://doi.org/10.1073/pnas.96.18.10477) (PMC17914), only the abstract read: "[t]he
escape reflex can be observed in 98% of the adult animals". The full text could not be fetched as text; it is to be
read for a latency.

### Light

[Edwards et al. 2008](https://doi.org/10.1371/journal.pbio.0060198) (PMC2494560) shone blue-violet and ultraviolet
light on worms.

- "In blue-violet light, wild-type worms showed a rapid response initiation (1–2 s) and a slow response decay after
  turning off the light (∼2 min)"; after a minute of light, "their locomotion slowly decayed to the basal rate over
  the next 140 s".
- Mutants lacking a neuropeptide-release protein (unc-31) or the G protein Gαq responded after "∼10-s delay", against
  "1–2 s for wild type".
- "light exposure induces body contraction within 330 ms", the interval between their images, so 330 ms is an upper
  bound set by the sampling, not a measured latency.

### Temperature in the Physiological Range

Thermotaxis, the response to temperatures the worm prefers, is a different behaviour from heat avoidance (Wittenburg
and Baumeister 1999); its reaction latency is still to be read. One study of rapid cooling scores a reversal as a
response if it starts "within 5 s of stimulation initiation" ([Lin et al. 2024](https://doi.org/10.1038/s41467-023-44638-5),
PMC10770330): a scoring window, like Liu et al.'s, not a measured latency. Two slower timescales are stated in text:

- **Hours, for a remembered temperature without food.** Worms placed on a thermal gradient without food for one hour
  "accumulated at the temperature of previous cultivation"; "when left longer on the thermal gradient, animals started
  dispersing in several hours and dispersed almost evenly within 24 hours regardless of their previous cultivation
  temperature" ([Aoki et al. 2022](https://doi.org/10.1371/journal.pgen.1010219), PMC9176836). The authors tie the
  dispersal to starvation. The time course is in their figures, not read; hours to a day is an order of magnitude, not
  a time constant.
- **Minutes, in the thermosensory neuron.** In patch-clamp recordings from AFD in glued animals, the neuron exposed
  through a slit in the cuticle, AFD's activation threshold depends on the temperature at which the neuron is held
  during the recording. The paper states the hold for a mutant: "In contrast with wild-type AFD neurons, the
  application of a holding temperature of 17°C for at least 15 min to pde-2 mutant AFD neurons failed to shift T * to
  the new temperature" ([Wang et al. 2013](https://doi.org/10.1085/jgp.201310959), PMC3787776). That wild-type
  neurons shift after a like hold is the lab's reading of this contrast and of their Figure 6, not read; the wild-type
  hold time is not stated. "At least 15 min" is the protocol's minimum hold: a shift seen after it says nothing of
  how fast the threshold moves, and it is not a time constant.

### Smell

[Sanders et al. 2013](https://doi.org/10.1186/1471-2202-14-156) (PMC3878553) held a hair dipped in 30% octanol before
the worm's nose and "observed reversals characteristic of the avoidance response ... with latencies of 4 sec in both
genetic backgrounds, both on and off food (data not shown)". A number stated in text, but for data the paper does not
show, so it is weak. The Grid has no chemical sense yet in any case.

## Rhythms

- **The body wave.** [Stephens et al. 2008](https://doi.org/10.1371/journal.pcbi.1000028) (PMC2276863) evolved their
  model of the worm's posture "for long times (93.75 s≈47 cycles)", about 2.0 s per cycle (93.75 / 47 = 1.99). That is
  their model's oscillation, fitted to worms tracked off food, not a direct measurement, and it is used here only as
  an order of magnitude.
- **The crawling frequency of the reference recordings** is measured by Yemini et al. 2013 as "Absolute Midbody
  Frequency" ([the comparison space](../benchmark/FEATURES.md)); its value for wild type is in the feature database,
  to be computed by the feature tool (STATE.md *Next*), not taken from a paper.

## Time Constants of Internal States

| State | Time constant | Source, read in full |
|---|---|---|
| Dwelling, on food | Durations fit one exponential, τ = 482 s | [Flavell et al. 2013](https://doi.org/10.1016/j.cell.2013.08.001) |
| Roaming, on food | Durations fit two exponentials, τ = 75 s and 520 s; "85% of roaming states have a mean lifetime of 75 sec" | Flavell et al. 2013 |
| Settling after transfer to a new plate | Speed and crawling frequency decay with τ = 19 minutes, midbody amplitude with τ = 48 minutes; the fit is good for speed (R² = 0.96) and weaker for frequency (0.71) and amplitude (0.59) | [Yemini et al. 2013](https://doi.org/10.1038/nmeth.2560) |
| After-effect of light | Locomotion returns to its basal rate over about 140 s, a duration, not a fitted τ | Edwards et al. 2008 |
| Feeding, on meeting food | Pumping "rose slowly, on the time scale of 100 s of seconds", in worms food-deprived for 1-2 h and then offered food in a microfluidic chip that holds the worm at its midsection by a vacuum clamp, "leaving the head ... and tail free to move" | [Katzen et al. 2023](https://doi.org/10.7554/eLife.69779) (PMC10231927) |
| Lethargus, the quiescence between larval stages | "typically last 1-2 h", as Faerberg et al. 2024 cite Singh and Sulston 1978; the primary source is not read | [Faerberg et al. 2024](https://doi.org/10.1242/dev.202961) (PMC11266794) |

## For the Lab

- **The Grid's tick is 31.25 ms** ([the comparison space](../benchmark/FEATURES.md#the-two-bodies) gives its source).
  The behavioural timings above, 330 ms and longer, span ten ticks or more and are resolvable; the receptors'
  adaptation over "a few tens of milliseconds" is a timescale, not a latency, and lasts about one tick, below what a
  tick can resolve. A Program's reflexes must not be faster than the worm's merely because the tick allows it.
- **What the benchmark measures.** The latencies above come from stimuli the reference recordings do not contain: they
  are of spontaneous behaviour. So the benchmark can hold a Program to the time constants of its states and its rhythm
  from the recordings themselves, but to its reaction latencies only through stimulus assays, which wait on stimuli
  the experimenter can deliver on the Grid ([the C plans](../hypotheses/experiments/C1-C12.md)).
- **The habituation window** of the comparison space rests on Yemini et al.'s τ = 19 and 48 minutes: the reference
  recordings start 30 minutes after transfer, when, if the decay starts at transfer, speed has made about 1 -
  e^(-30/19) = 0.79 of its decline. Which window of a Grid life matches is an open question of the comparison space
  ([FEATURES.md § Open Questions Before Freezing](../benchmark/FEATURES.md#open-questions-before-freezing)), answered
  when the benchmark is frozen.

## Gaps, to Be Read

- A touch-to-reversal latency: none is stated in the text of 45 open papers searched; the next step is a figure read
  with a stated method, or the reference recordings themselves, if they include taps. Liu et al. 2018 cite Ardiel et
  al. 2017, "Habituation as an adaptive shift in response strategy mediated by neuropeptides"
  ([doi:10.1038/s41539-017-0011-8](https://doi.org/10.1038/s41539-017-0011-8), PMC6161508), for "reversal latency";
  read in full, it measures latency to light-activation of ASH, with values in figures only.
- The heat-avoidance reaction times of Mohammadi et al. 2013, Figure 2c, and Wittenburg and Baumeister 1999 in full.
- A thermotaxis reaction latency (the slower timescales above are orders of magnitude only), and the primary source
  for lethargus (Singh and Sulston 1978). [Roman et al. 2023](https://doi.org/10.1073/pnas.2215191120) (PMC10068832)
  fit a model of thermal preference after starvation with "four time scales", the avoidance slower than the
  habituation; the fitted values are in their supplementary tables, not read.
- Fitted time constants for tap habituation and its recovery: the primary papers (Rankin et al. 1990 and later) were
  not open in full text; only a review's summary is read.
- The timing of sleep-like quiescence outside lethargus.
