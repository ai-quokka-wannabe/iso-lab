# The Worm's Pace

The rule *The animal's pace* asks that a Program's reaction latencies, the time constants of its internal states
and the rhythm of its decisions match the worm's, each from a cited source. This note gathers what the lab has read
so far. Some latencies are given in the sources only as figures; those not yet read are listed as gaps rather than
guessed.

The papers were read on 2026-10-14, or on 2026-11-03 where a line names that pass, in full on PubMed Central, through
NCBI's BioC service and Europe PMC, unless a line says otherwise. How sure the lab is: high for the quoted sentences and numbers, which are the papers' own;
lower for the paragraphs headed *For the lab*, which are the lab's reasoning. Numbers are taken from a paper's text,
except where a line says they are read off a figure: those carry the uncertainty of the reading, and the method is in
the journal.

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
  are in the journal) found none, and neither did five more queries on 2026-11-03 (journal of 2026-11-03, § The Pace
  Note's Gaps, a Fourth Pass), nor the full texts of the force-clamped touch assay of [Mazzochette et al.
  2018](https://doi.org/10.1039/c8ib00045j) (PMC6168290), the high-speed films of touch assays of [Nekimken et al.
  2017](https://doi.org/10.1371/journal.pone.0178080) (PMC5438190) and the optogenetic tracker of [Leifer et al.
  2011](https://doi.org/10.1038/nmeth.1554) (PMC3032981).
- **The nearest, from activating the touch neurons with light** (2026-11-03 pass): [Porto et al.
  2019](https://doi.org/10.1038/s41598-019-41349-0) (PMC6435754) drove channelrhodopsin in the touch receptor
  neurons (promoter *mec-4*, in a *lite-1* background) of freely crawling worms with a pseudorandom light sequence
  switching at 2 Hz, filmed at 13 Hz ("acquisition rate is 13 Hz", so frames about 77 ms apart), and computed by
  reverse correlation the filter linking the anterior neurons' activation to the worm's acceleration: "the delay to
  the peak (0.2 s) and the decay timescale of the filter (0.4 s)". For reversals and pauses they write only that
  "this increased likelihood happens within the first second after a stimulus". This is a filter's peak, not the
  start of a reversal, and it includes the light-gated channel's own kinetics and comes from light, not touch; but it is
  frame-level timing, stated in text, and the lab's best number so far for how soon the touch neurons move the body.
  How sure: high that the paper says it; moderate for its use as a touch latency.
- **Read from Liu et al.'s Figure 2—figure supplement 2**, by pixel (method in the journal of 2026-10-14): the
  population's rate of classified transitions into their "Fast Reverse" state peaks about 0.25 s after a plate tap,
  and about 0.6 s after the onset of a 1 s light pulse. The light figure is not a touch latency: it comes from a
  transgenic strain expressing the light-gated channel Chrimson in the touch neurons (AML67), whose tap sensitivity
  the paper reports as reduced, and it includes the channel's own activation. Both are times of a peak, not of a
  response's start, and they rest on a behaviour classifier whose timing is coarse: frames came "at 14 frames per
  second", a transition is stamped at the first frame classified in the new state, and the wavelet spectrogram behind
  the classification "introduces an inherent uncertainty in the precise timing of a behavior transition", which in
  their words "causes the animal occasionally to appear to respond (but not actually respond) to a stimulus prior to
  its delivery", over up to 2.7 s in the worst case; the paper gives no figure for Fast Reverse alone. So the figure
  shows classified reversals clustering within about a second of a tap, but it does not resolve when a reversal
  starts; it is not a latency to hold a Program to.
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
- **Habituation, from primary sources** (2026-11-03 pass). The Rankin laboratory's primary papers of 1990 to 2001
  are not open; their abstracts, read through Europe PMC, give one bound: at 4 and 7 days old, "worms had recovered
  from habituation by 30 min after training" ([Beck and Rankin 1993](https://doi.org/10.1016/0376-6357%2893%2990088-9),
  abstract only), and recovery "was more rapid, with short interstimulus intervals (ISIs) than with longer ISIs"
  ([Rankin and Broster 1992](https://doi.org/10.1037/0735-7044.106.2.239), abstract only). A later open paper fits
  exponentials to tap habituation, 30 taps at 10 s intervals, and calls "the habituation rate (the half-life of the
  exponential curve)" a measure, but reports it in figures only ([Giles et al.
  2015](https://doi.org/10.1534/g3.115.021410), PMC4683646). The one fitted value stated in text comes from
  optogenetic activation of the touch receptor neurons, not taps: Porto et al. 2019 (above) needed an exponential decay
  of the response magnitude across their continuous stimulus sequence and found that "a decay rate of 50 s provided
  the best predictions", chosen from a range of values tested (their Figure S14, not read), without an uncertainty.
  Their methods write the decay as exp(-λt) with "λ is 50 s", so the 50 s is a time constant, whatever the word
  *rate* says. They add that it "is consistent with previous findings from investigations of habituation to
  stimulation of TRNs, with both tapping and optogenetic stimuli", citing [Timbers et al.
  2013](https://doi.org/10.1016/j.neurobiolaging.2012.03.016) (not open to this session), the lead for a tap value.
  So far: a recovery bound and a model's time constant, said by its authors to agree with tap habituation, not yet
  a fitted time constant of tap habituation read by the lab.
- **Tap habituation, read off a figure** (2026-11-03, fifth pass). Giles et al. 2015 do not print their half-lives
  as numbers in any figure either: their figures plot response probability against tap number with "best-fit
  exponential curves" and mark only the asymptote. By eye, from the wild-type curve of their Figure 3B (30 taps,
  10 s apart; reversal "within 2 sec" of a tap counted as a response; n = 3 to 6 plates of 50 to 100 worms), the fitted
  curve falls from about 0.9 at the first tap to an asymptote of about 0.3, and is halfway down after roughly 5 to 10
  taps, so roughly 50 to 100 s at that interval. This is the lab's reading of a drawn curve, not the authors' fitted
  value, and its uncertainty is the whole range given; how sure: low. It is the same order as Porto et al.'s 50 s.
  Timbers et al. 2013, the source Porto et al. cite, is still not open: Europe PMC lists it as subscription only, with
  no author manuscript, and its abstract gives no time constant, only that with age "response probability habituation
  increases (at both 10- and 60-second interstimulus intervals)".

### Noxious Heat

[Mohammadi et al. 2013](https://doi.org/10.1186/1471-2202-14-66) (PMC3703451) heated a spot on the worm with a laser
pulse of 133 ms.

- "the animal's reaction time does in fact vary inversely with stimulus amplitude", and "the reaction time is
  proportional to the logarithm of the stimuli strength".
- At the head the typical response is "a reversal, followed by an omega turn, then a recommencement of forward
  motion".
- **Read from their Figure 2c**: the reaction times are not in the text, only in the figure, "[t]he reaction time of
  the worm in response to a head-applied laser stimulus as a function of laser power". Its image came from Europe
  PMC's supplementary-files service (`https://www.ebi.ac.uk/europepmc/webservices/rest/<PMCID>/supplementaryFiles`, a
  zip of the paper's figure files; PubMed Central's page asks for a CAPTCHA, which the lab does not get round). Read
  by pixel, with the method in the journal of 2026-10-14, the four mean reaction times are about 0.33, 0.29, 0.27 and
  0.25 s, falling as the laser current rises from about 30 to about 150 mA (read from the figure's logarithmic axis as
  about 31, 64, 102 and 154 mA; the paper names 60, 100 and 150 mA in its Figure 1d caption and 30 and 150 mA in its
  discussion, without tying them to this panel). Each mean is read to within about 0.005 s. The panel draws two error
  marks, grey boxes labelled "SEM" and whiskers labelled "VAR", and the text defines neither VAR nor the panel's
  number of worms; the marks do not behave as a standard deviation beside its standard error (for the strongest
  stimulus the whisker, about ±4 ms, lies inside the box, about ±7 ms), so they are not read here as the spread among
  worms. For the two weakest stimuli the whiskers are about ±0.027 s and the boxes about ±0.025 and ±0.016 s. The text
  does not say from which moment the reaction time is counted, nor how the reaction is detected; the videos were
  recorded "at 30 fps", so the times rest on frames 33 ms apart. How sure: moderate for the means as plotted, low for
  what exactly they measure and for their spread.
- **What the reaction times count, from the first author's thesis** (2026-11-03 pass). [Mohammadi
  2014](http://hdl.handle.net/1807/43669), the PhD thesis behind the paper, public in the University of Toronto's
  repository and read in the text the repository extracted from its PDF, describes the method: each frame is reduced
  to 41 points along the body; "[i]t was assumed that the worm was going forward at the moment the thermal stimulus is
  applied"; a reversal's start is the frame at which the centroid's path, "sampled at every 2 frames", turns by more
  than 120 degrees; a pause is a centroid speed below 0.4 pixels per frame, with the body at least three quarters extended, for three
  consecutive frames; and every flag was then checked by eye in a viewer and corrected where wrong. On timing it states, of the 133 ms pulse and
  the heat's decay to 1/e after about 150 ms, a "total time estimate for the thermal pulse" of about 300 ms, and that
  "the average reaction time for a worm to withdraw from the noxious pulse is ~250ms", so that the worm moves "at its
  withdrawal (accelerated) speed for ~50ms" of the pulse. That arithmetic, 300 - 250 = 50 ms, counts the reaction time
  from the laser's onset; neither text says so in words, so this is the lab's reading, and it reads the end point
  as the first frame flagged in a new state, which the thesis does not tie to Figure 2c either. How sure: moderate for
  the onset, lower for the end point. The thesis's ~250 ms agrees with the strongest stimulus's 0.25 s read off the
  figure. A later paper from the same laboratory, with the same kind of pulse (0.1 s) filmed at 60 Hz, considered
  removing the first "about 10 frames (0.16 s) after the laser stimulation since worms do not respond to the
  stimulus so quickly"; removing them "had a negligible effect on the model performance", and they were kept
  ([Leung et al. 2016](https://doi.org/10.1371/journal.pcbi.1005262), PMC5189946). The authors' assumption that
  nothing happens within 0.16 s is not a measurement, but it is consistent with the rest.

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
PMC10770330): a scoring window, like Liu et al.'s, not a measured latency. The papers that time the behaviour's
response to defined temperature steps are identified - [Ryu and Samuel 2002](https://doi.org/10.1523/jneurosci.22-13-05727.2002)
(PMC6758190), [Zariwala et al. 2003](https://doi.org/10.1523/jneurosci.23-10-04369.2003) (PMC6741103), [Clark et al.
2007](https://doi.org/10.1523/jneurosci.1032-07.2007) (PMC6672141) and [Luo et al. 2014](https://doi.org/10.1073/pnas.1315205111)
(PMC3932917) - but none was open to this session: they are free to read on PubMed Central, behind its CAPTCHA, but
outside the open-access subset that Europe PMC's and NCBI's text services serve, and the publishers' pages refused
the request (2026-11-03 pass; only the first two abstracts read, which state no latency).

- **At the receptor** (2026-11-03 pass): in patch-clamp recordings from AFD in a slit-worm preparation, small rapid
  temperature steps (0.26 ± 0.06 °C) changed the thermoreceptor current with "an average latency of 96 ± 5 ms (n =
  3)" in three recordings, and 132 ms in a fourth made with a different pipette saline; "[t]he latency was the time
  difference between the half maximum points of both traces", the temperature's and the current's ([Ramot et al.
  2008](https://doi.org/10.1038/nn.2157), PMC2587641, the author manuscript, read through NCBI's BioC service). Like
  the touch receptors' milliseconds, a receptor's latency in a dissected animal, not a behaviour's; it says
  thermosensation starts about a tenth of a second after the change, some three ticks of the Grid.

Three slower timescales are on record:

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
- **Fitted time scales, of a model of thermal preference.** [Roman et al.
  2023](https://doi.org/10.1073/pnas.2215191120) (PMC10068832) confined single worms in millimetre-sized droplets
  without food, where they swim, and "monitor[ed] their swimming behavior" for up to 4 hours with a gradient across
  each droplet; some worms had been starved beforehand, for different lengths of time. They fitted a model with "four
  time scales"; in their words, "the avoidance time scale is slower than the habituation one". The fitted values are
  in their supplementary Tables S3 and S4 (read in the supplementary PDF): the avoidance time scale τa is 1.39 for
  worms reared at 15°C and 0.75 for worms reared at 25°C, with upper-bound uncertainties of 0.58 and 0.39; the
  habituation time scale is a fitted fraction of it, τh/τa = 0.28 and 0.41, with upper-bound uncertainties of 0.14 and
  0.29. The tables give no unit; the supplement says "t represents the time in hours since the beginning of assaying
  in the droplet", and the main text has the preference "changing the sign in about 3 h for cold worms and 2 h for
  warm worms", so the lab reads the time scales as hours: about 83 and 45 minutes for avoidance, and 23 and 18 minutes
  for habituation. The habituation times are weak: combining the two upper-bound uncertainties in quadrature gives
  about 65% and 90%, so the cold-worm value is known only to within a factor of about two and the warm-worm value is
  barely bounded from below (the tables' lower-bound uncertainties are a few per cent, so the real uncertainty lies between).
  These are a model's parameters, fitted to swimming worms in droplets, not time constants measured directly in
  crawling ones; the time scales at the rearing temperature in the same tables are left out here.

### Smell

[Sanders et al. 2013](https://doi.org/10.1186/1471-2202-14-156) (PMC3878553) held a hair dipped in 30% octanol before
the worm's nose and "observed reversals characteristic of the avoidance response ... with latencies of 4 sec in both
genetic backgrounds, both on and off food (data not shown)". A number stated in text, but for data the paper does not
show, so it is weak. The Grid has carried a chemical sense since Program ABI 11, but its only odours are those of
food (STATE.md *Needs From Other Repositories*), so it has no repellent to time an avoidance against yet.

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
| Lethargus, the quiescence between larval stages | Measured (2026-11-03 pass), in wild-type cohorts each worm observed "at regular 1 h intervals" (for "5–60 s" by the methods, about a minute by the results) and scored lethargic when "neither pharyngeal pumps nor sinusoidal locomotion were observed": the L4 lethargus lasted 2.2 ± 0.4 h (mean ± sd, N = 16), 2.0 ± 0.0 h (N = 17) and 2.0 ± 0.0 h (N = 12) in three N2 cohorts and 2.2 ± 0.6 h (N = 20) in a wild-type GR1395 cohort, the L3 1.6 ± 0.5 h (N = 17) and 1.4 ± 0.5 h (N = 13), the L2 1.3 ± 0.5 h (N = 18) (Supplementary file 1). The worms were singled on NGM-RNAi plates, so these wild-type rows may be control-RNAi conditions; the lab did not check which. Hourly sampling resolves the durations only to the hour; the methods say strains were grown at 25 °C unless otherwise specified, and the lab did not find another temperature stated for these assays. Within L4 lethargus, the posture-based measurements of Biron's and Raizen's laboratories show "alternating bouts of quiescence and motion of durations of 1–100 sec", with "most quiescent bout durations" shorter than 20 s. The primary source usually cited, Singh and Sulston 1978, is still not open, and it is cited two ways: "1-2 h" by Faerberg et al. 2024, a "2–3 hr period" by [Nagy et al. 2013](https://doi.org/10.7554/eLife.00782) (PMC3699835) and "the 2–3 hour period of L4leth" by Nagy, Raizen and Biron 2014 | [Patel et al. 2022](https://doi.org/10.7554/eLife.80010) (PMC9377799); [Nagy, Raizen and Biron 2014](https://doi.org/10.1016/j.ymeth.2014.03.009) (PMC4112141, a methods paper, the author manuscript); [Faerberg et al. 2024](https://doi.org/10.1242/dev.202961) (PMC11266794) |
| Satiety quiescence, in adults fasted 12 h then refed | Fifth pass, 2026-11-03. A hidden Markov model of each worm's speed and turning, filmed at 1 frame a second for an hour after 3 h of refeeding on good food, gives, for 14 wild-type worms, a mean probability of staying quiescent from one step to the next of 0.98942 (σ 0.00031, the σ not defined in the file); with one step taken as one frame (the lab's assumption, not stated beside the numbers), a quiescent state lasts on average 1 / (1 - 0.98942) ≈ 95 s, and dwelling ≈ 75 s, in line with the paper's "typical lifetimes observed were between 1 and 2 min" for all states. The authors warn that watching wakes the worms, so "it has not been possible to watch satiety-induced quiescence for more than about a minute"; these are bouts under observation, not undisturbed ones. Read off a bar chart: once a worm was found quiescent, the bout's duration was timed for 10 worms and averaged; wild type, 12 h fasted and refed 3 or 6 h, about 48, 51 and 41 s in three panels (Figure 1A, B, E; read by eye to about ± 2 s; the error bars are not defined in the caption) | [Gallagher et al. 2013](https://doi.org/10.1371/journal.pone.0059865) (PMC3610905), Dataset S1 (`wt_fasted`, columns Q -> Q and D -> D) and the methods; [Hyun et al. 2016](https://doi.org/10.1038/srep24841) (PMC4838937), Figure 1 and the methods |
| Sleep-like quiescence after stress, in adults | Duration: after a 30 min heat shock at 35 °C, "both pharyngeal pumping and locomotion remain suppressed for up to an hour after exposure". Latency: after cold stress (30 min at −15 °C), "a peak of ALA-dependent locomotor quiescence at 20 minutes" - the time of a peak, not a bout's length. For contrast, not quiescence: after "a brief (<1 min) heat exposure" animals "recover normal activity within minutes", recovery from the acute response, which the paper cites to its Table S1, not read. The time courses themselves are in figures not read | [Hill et al. 2014](https://doi.org/10.1016/j.cub.2014.08.040) (PMC4254280, the author manuscript), the passages quoted read, not the whole paper |

## For the Lab

- **The Grid's tick is 31.25 ms** ([the comparison space](../benchmark/FEATURES.md#the-two-bodies) gives its source).
  The behavioural timings above, 200 ms and longer (Porto et al.'s filter peak, 0.2 s, is the shortest), span six
  ticks or more and are resolvable; the receptors' latencies, from about 1 ms for touch to about 100 ms for
  temperature, are under one to about three ticks, and their adaptation over "a few tens of milliseconds" is a
  timescale, not a latency, about one tick long, below what a tick can resolve. A Program's reflexes must not be faster than the worm's merely because the tick allows it.
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

What is still open after the passes of 2026-11-03 (journal of 2026-11-03, § The Pace Note's Gaps, a Fourth Pass, and
§ The Pace Note's Gaps, a Fifth Pass, have what was tried).

- A touch-to-reversal latency, from a mechanical touch, timed to the reversal's start: none is stated in the text of
  the open papers searched. The nearest are Liu et al.'s figure (reversals clustering within about a second) and Porto
  et al.'s filter (a peak 0.2 s after optogenetic activation of the touch neurons), both above. The reference
  recordings contain no stimuli. Liu et al. 2018 cite Ardiel et al. 2017, "Habituation as an adaptive
  shift in response strategy mediated by neuropeptides"
  ([doi:10.1038/s41539-017-0011-8](https://doi.org/10.1038/s41539-017-0011-8), PMC6161508), for "reversal latency";
  read in full, it measures latency to light-activation of ASH, with values in figures only. The next ways: Liu et
  al.'s or Porto et al.'s published data, if public, re-analysed for the reversal's onset; or a figure of a paper timing
  touch at a high frame rate.
- What Mohammadi et al. 2013's reaction times count, in words: the first author's thesis (above) fixes the detection
  method and implies, by its arithmetic, that they count from the laser's onset, but neither it nor the paper defines
  the end point, nor the figure's "VAR" marks. Their group's earlier paper, [Ghosh et al.
  2012](https://doi.org/10.1186/1741-7007-10-85) (PMC3520762), does not define a response latency either; its nearest
  measure is TMAX, the "mean time to reach MAXS" (the maximum speed after the laser pulse). The authors or their code
  would settle it. And Wittenburg and Baumeister 1999 in full, still not served as text (BioC: not in the open-access
  subset; the publisher refused the request).
- A thermotaxis reaction latency in behaviour: the receptor's latency is now read (Ramot et al. 2008, above); the
  behavioural papers are identified but not open to this session (above).
- Fitted time constants for tap habituation and its recovery: the primary papers (Rankin et al. 1990 and later) are
  not open in full text; their abstracts give a recovery bound only. Giles et al. 2015 print no half-life, even in
  their figures; the lab's reading of their wild-type curve by eye (roughly 50 to 100 s at 10 s intervals, above) is
  rough. Timbers et al. 2013, behind Porto et al.'s 50 s, is subscription only with no author manuscript in Europe PMC,
  and its abstract gives no time constant. Next: the Multi-Worm Tracker data of a tap habituation paper, if public,
  fitted by the lab.
- The primary source for lethargus as usually cited, Singh and Sulston 1978 (not open); measured durations are now in
  the table, from Patel et al. 2022.
- The timing of sleep-like quiescence outside lethargus: stress-induced quiescence has timings from Hill et al.
  2014's text (above), its figures' time courses still to read. Satiety quiescence is now narrowed (fifth pass, table
  above): bouts under observation last about 40 to 50 s by stopwatch (Hyun et al. 2016, read off a bar chart) and
  about 95 s on average by a hidden Markov model (Gallagher et al. 2013, Dataset S1, assuming one step per frame). Both
  are bouts of watched worms, which the authors say wake under observation; undisturbed bouts are unmeasured in what
  the lab has read. Its primary source, You et al. 2008
  ([doi:10.1016/j.cmet.2008.01.005](https://doi.org/10.1016/j.cmet.2008.01.005), PMC3786678), is still not served
  (Europe PMC: HTTP 500; NCBI's BioC: not in the open-access subset; the publisher's pages refused the request).
