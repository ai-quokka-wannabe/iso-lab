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

- They counted a behavioural transition as a response if it came "within 2 s after stimulus onset".
- After a tap or a 1-second stimulation of the touch neurons, "both light and tap also evoked an increase in 'Turn'
  behavior that peaked 5 s post-stimulus".
- From their filters for reversals they suggest "the animal may integrate the sensory signal over approximately 3 s in
  determining to reverse"; this is their interpretation, worded as a suggestion.
- On the receptors themselves, summarising earlier work, they write that the currents evoked by a force "adapt with a
  timescale of a few tens of milliseconds".
- **Not found yet**: the latency from a touch to the start of a reversal, as a number in a paper's text. Liu et al.
  show it in figures, which were not read.

### Noxious Heat

[Mohammadi et al. 2013](https://doi.org/10.1186/1471-2202-14-66) (PMC3703451) heated a spot on the worm with a laser
pulse of 133 ms.

- "The animal's reaction time does in fact vary inversely with stimulus amplitude", and "the reaction time is
  proportional to the logarithm of the stimuli strength".
- At the head the typical response is "a reversal, followed by an omega turn, then a recommencement of forward
  motion".
- **Not found yet**: the reaction times in seconds, which are in their Figure 2c and not in the text. The figure was
  not read (PubMed Central's page asks for a CAPTCHA, which the lab does not get round, and Europe PMC did not serve
  the image).

[Wittenburg and Baumeister 1999](https://doi.org/10.1073/pnas.96.18.10477) (PMC17914), only the abstract read: the
"escape reflex can be observed in 98% of the adult animals". The full text could not be fetched as text; it is to be
read for a latency.

### Light

[Edwards et al. 2008](https://doi.org/10.1371/journal.pbio.0060198) (PMC2494560) shone blue-violet and ultraviolet
light on worms.

- "In blue-violet light, wild-type worms showed a rapid response initiation (1–2 s) and a slow response decay after
  turning off the light (∼2 min)"; after a minute of light, "their locomotion slowly decayed to the basal rate over
  the next 140 s".
- Mutants lacking a neuropeptide-release protein (unc-31) or the G protein Gαq responded after "∼10-s delay", against
  "1–2 s for wild type".
- "Light exposure induces body contraction within 330 ms", the interval between their images, so 330 ms is an upper
  bound set by the sampling, not a measured latency.

### Temperature in the Physiological Range

Not covered yet. Thermotaxis, the response to temperatures the worm prefers, is a different behaviour from heat
avoidance (Wittenburg and Baumeister 1999); its time course is to be read from a thermotaxis source.

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
| Settling after transfer to a new plate | Speed and crawling frequency decay with τ = 19 minutes, midbody amplitude with τ = 48 minutes | [Yemini et al. 2013](https://doi.org/10.1038/nmeth.2560) |
| After-effect of light | Locomotion returns to its basal rate over about 140 s | Edwards et al. 2008 |

## For the Lab

- **The Grid's tick is 31.25 ms**, so every latency above, from tens of milliseconds at the receptors to seconds in
  behaviour, is resolvable; a Program's reflexes must not be faster than the worm's merely because the tick allows it.
- **What the benchmark measures.** The latencies above come from stimuli the reference recordings do not contain: they
  are of spontaneous behaviour. So the benchmark can hold a Program to the time constants of its states and its rhythm
  from the recordings themselves, but to its reaction latencies only through stimulus assays, which wait on stimuli
  the experimenter can deliver on the Grid ([the C plans](../hypotheses/experiments/C1-C12.md)).
- **The habituation window** of the comparison space rests on Yemini et al.'s τ = 19 and 48 minutes: the reference
  recordings start 30 minutes after transfer, when, if the decay starts at transfer, speed has made about 1 -
  e^(-30/19) = 0.79 of its decline. Which window of a Grid life matches is still to be fixed.

## Gaps, to Be Read

- A touch-to-reversal latency stated in a paper's text, or a figure read with a stated method.
- The heat-avoidance reaction times of Mohammadi et al. 2013, Figure 2c, and Wittenburg and Baumeister 1999 in full.
- A thermotaxis time course.
- Habituation to repeated taps, and its time constants (the Rankin laboratory's papers are the classic source; none
  was open in full text in this search).
- The timing of quiescence and sleep-like states, and of feeding.
