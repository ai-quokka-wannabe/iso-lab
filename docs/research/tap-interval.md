# The Tap Interval, from the Literature

The maintainer ruled, in the session of 2026-11-05, "Check literature first" on whether a 2 s interval between taps is
right for the benchmark's tap-habituation assay (STATE.md item 12). The 2 s came from the lab's toy-worm work: a
train of taps every 2 s separated habituation's two fitted parameters on a near noise-free target
([H13](../hypotheses/README.md#h13-mechanism-matched-modules-trained-as-a-whole-then-ground-down); the journal of
[2026-10-22](../journal/2026-10-22.md), § H13: A Short-Interval Train, Profiled and Fitted), and that journal said in
so many words that the interval's realism was unchecked. This note is that check: which interstimulus intervals (ISIs)
the tap-habituation papers used, what each reports for the level and rate of habituation and for spontaneous recovery,
and a proposal for the maintainer to rule on. It is literature work only; the benchmark's definition is not changed
here. The pace note's habituation entries ([pace.md § Touch](pace.md#touch)) hold the fitted time constants the lab has
and has not found; this note does not repeat them.

## How the Papers Were Read, and How Sure the Lab Is

Read on 2026-11-08 (UTC). Every paper's record and abstract came from Europe PMC's REST search, by DOI or by author and
title. Full texts came from NCBI's BioC service for papers in PubMed Central's open-access subset (Swierczek et al.
2011, Giles et al. 2015, Liu et al. 2018, McDiarmid et al. 2018 and 2022, Chalissery et al. 2023, Kokan et al. 2025) and from
Frontiers' own pages for Bozorgmehr et al. 2013; Ardiel et al. 2017 was read from PubMed Central's web page. Europe
PMC's full-text service answered HTTP 500 to every request that night, so three papers it holds in full were read as
abstracts only (Rankin and Wicks 2000, Cai et al. 2009, Ardiel et al. 2018), and the publishers' own pages for them and
for the Learning & Memory reviews refused the request (HTTP 403, or a browser challenge), as did bioRxiv (HTTP 429)
for Ardiel et al. 2018's preprint. The Rankin laboratory's primary ISI papers of 1992 to 1997 are in subscription
journals and were read as abstracts only. For a paper whose full text Europe PMC indexes but did not serve, the lab
asked its search index whether a phrase occurs in the text (`PMCID:... AND BODY:"..."`); such a result says that the
phrase is in the paper and nothing of its context, and the table marks it *indexed phrase* with low confidence.

How sure: **high** for quoted sentences, which are the papers' own; **medium** for a protocol taken from an abstract,
which can omit a condition; **low** for an indexed phrase and for anything read off a figure. Where a figure holds the
number and the lab did not read it, the cell says *in figures, not read*. No number below is estimated from memory.

## The Table

Taps are plate taps (a solenoid or a hand against the plate's side) unless a row says otherwise. "Level" is the
habituated response, "rate" how fast it is reached, "recovery" the return after the train stops.

| Source | ISIs | Protocol | Habituation level and rate | Spontaneous recovery | Read as; how sure |
|--------|------|----------|----------------------------|----------------------|-------------------|
| [Rankin, Beck and Chiba 1990](https://doi.org/10.1016/0166-4328%2890%2990074-o) | 10 s or 60 s, by [Bozorgmehr et al. 2013](https://doi.org/10.3389/fphys.2013.00088)'s summary: distance "decremented after repeating the stimulus 40 times at interstimulus intervals (ISIs) of 10 s or 60 s" | Reversal amplitude to "a mechanical vibratory stimulus" (abstract) | Decrement, not quantified in the abstract | "This response decrement recovered to the baseline after a few minutes" (Bozorgmehr et al.) | Abstract, and a secondary summary; medium for the ISIs, low for "a few minutes" |
| [Rankin and Broster 1992](https://doi.org/10.1037/0735-7044.106.2.239) | "short interstimulus intervals (ISIs)" against "longer ISIs"; the values are not in the abstract | Four experiments on "the rate of habituation, the degree of habituation, and the rate of recovery" | "habituation was more pronounced and faster ... with short interstimulus intervals (ISIs) than with longer ISIs" | "recovery from habituation was more rapid, with short interstimulus intervals (ISIs) than with longer ISIs. Rate of recovery differed in animals that had reached asymptotic response levels when compared with animals still in the descending portion of the habituation curve. Once animals reached asymptotic response levels, rate of recovery appeared to be determined by ISI and not by additional stimuli" | Abstract only (closed); high for the direction, the ISIs themselves unread |
| [Beck and Rankin 1993](https://doi.org/10.1016/0376-6357%2893%2990088-9) | 10 s and 60 s | Worms at 4, 7 and 12 days post-hatching | "Worms at all ages expressed habituation and dishabituation at a 10 s interstimulus interval (ISI)"; "older worms showed greater habituation than did 4-day-old worms to stimuli delivered at a 60 s ISI" | "At days 4 and 7, worms had recovered from habituation by 30 min after training" | Abstract only; high for the bound, medium for the protocol |
| [Broster and Rankin 1994](https://doi.org/10.1037/0735-7044.108.6.1019) | Fixed or variable with a mean of 10 s; fixed or variable 60 s; shifts between 10 s and 60 s during training | The effect of changing the ISI on habituation and recovery | "A 60-s ISI affected habituation at a 10-s ISI, but a 10-s ISI did not influence habituation at a 60-s ISI" | "When habituation stimuli were delivered at variable ISIs with an average of 10 s, recovery was slower than when habituation stimuli were delivered at fixed 10-s intervals. There were no differences in recovery following either fixed or variable stimulation at a 60-s ISI" | Abstract only; high for the statements |
| [Wicks and Rankin 1996](https://doi.org/10.1007/bf00216131) (J Comp Physiol A) | 10 s or 60 s | Intact and laser-ablated worms; each subcomponent of the response | "reversals are more frequent early in a habituation series and at longer inter stimulus intervals"; each component "habituates and does so with kinetics that are consistent with the decrement observed in the intact animal" | Not in the abstract | Abstract only; high for the statements |
| [Wicks and Rankin 1996](https://doi.org/10.1037/0735-7044.110.4.840) (Behav Neurosci) | 10 s (CON10) or 60 s (CON60); PLM-ablated worms at 10 s (PLM10) | Does recovery follow the ISI or the habituation kinetics? | "the PLM10 animals trained at a 10-s ISI habituated like CON60 worms" | "the recovery from habituation of the PLM10 animals very closely resembled the recovery of the CON10 worms. Thus recovery kinetics are dictated by consequences of a given ISI, which do not impact upon habituation rate and asymptote" | Abstract only; high |
| [Beck and Rankin 1997](https://doi.org/10.3758/bf03209851) | "long ISIs", not "short ISIs" (the title) | Long-term habituation by distributed training | The title: "Long-term habituation is produced by distributed training at long ISIs and not by massed training or short ISIs" | Not read | Title and bibliographic record only (OpenAlex, Crossref; no Europe PMC record); the body unread |
| [Rankin 2000](https://doi.org/10.1002/%28sici%291098-2302%28200005%2936%3A4%3C261%3A%3Aaid-dev1%3E3.0.co%3B2-7) (Dev Psychobiol) | 10 s or 60 s | Six developmental stages; head and tail circuits ablated apart | "All stages of development showed normal habituation and spontaneous recovery at both interstimulus intervals" | As left | Abstract only; high |
| [Rankin 2000](https://doi.org/10.1037/0735-7044.114.3.496) (Behav Neurosci) | 10 s and 60 s | Context conditioning with a chemosensory cue | Retention "at both 10- and 60-s interstimulus intervals was significantly greater" with the cue present at training and testing | Retention, not spontaneous recovery | Abstract only; high |
| [Rankin and Wicks 2000](https://doi.org/10.1523/jneurosci.20-11-04337.2000) (PMC6772661) | "all interstimulus intervals tested" (abstract); the index places "2-sec ISI", "10-sec ISI" and "60-sec ISI" in the text and neither "5-sec ISI" nor "30-sec ISI" | Wild type against *eat-4* | *eat-4* "habituate more rapidly and recover more slowly than wild-type worms at all interstimulus intervals tested" | As left; wild-type values in figures, not read | Abstract, and indexed phrases; high for the quotation, low for the set of ISIs |
| [Rose and Rankin 2001](https://doi.org/10.1101/lm.37801) (review) | - | - | "We have highlighted the importance of interstimulus interval (ISI) in both short- and long-term habituation and suggested that different cellular mechanisms might underlie habituation at different ISIs" | - | Abstract only; the full text, free on the publisher's site by OpenAlex, refused this machine |
| [Rose, Kaun and Rankin 2002](https://doi.org/10.1101/lm.46802) (PMC182588) | Long against short, values not in the abstract | Group training for long-term memory | "distributed training at long interstimulus intervals (ISIs) produced long-term retention, short ISIs did not"; "The inter-block interval must be >40 min" | Long-term retention, not spontaneous recovery | Abstract only; high |
| [Kindt et al. 2007](https://doi.org/10.1016/j.neuron.2007.07.023) | Not in the abstract | Habituation on and off food | "In the absence of food, animals habituate more rapidly than in the presence of food" | Not in the abstract | Abstract only (closed); the ISI unread |
| [Cai et al. 2009](https://doi.org/10.1038/emboj.2009.112) (PMC2693150) | "2, 5, 10, or 60 s ISI", by Bozorgmehr et al. 2013's summary; the index places "60-s ISI", "2 s" and "10 s" in the text | Wild type against an inactive MPS-1 kinase; "habituation scored as a response magnitude with probability and reversal distance combined" (Bozorgmehr et al.) | Mutants "do not habituate to repetitive mechanical stimulation such as tapping on the side of the Petri dish" (abstract); wild-type values in figures, not read | Not in the abstract | Abstract, a secondary summary and indexed phrases; medium for the set of ISIs |
| [Swierczek et al. 2011](https://doi.org/10.1038/nmeth.1625) (PMC3128206) | 10 s | 100 s of baseline, then 20 or 30 taps; on food after at least 6 h since transfer, tracked 10 min first; a reversal scored when the worm "moved backwards within one second of the impact"; "averaging 136 animals per tap" | "initially nearly all worms responded with large reversals, but with repeated taps the probability and size of responses decreased"; the habituated response is taps 28 to 30; values in figures, not read | Not measured | Full text; high for the protocol |
| [Timbers et al. 2013](https://doi.org/10.1016/j.neurobiolaging.2012.03.016) | 10 s and 60 s | Taps over a range of intensities, worms from the onset of reproduction to the end of egg laying | "response probability habituation increases (at both 10- and 60-second interstimulus intervals)" with age | Not in the abstract | Abstract only (closed) |
| [Sugi et al. 2014](https://doi.org/10.1073/pnas.1414867111) (PMC4260609) | Not in the abstract | An optical system that "automatically trains large populations" | Not in the abstract | Not in the abstract | Abstract only; the full text refused this machine |
| [Giles et al. 2015](https://doi.org/10.1534/g3.115.021410) (PMC4683646) | 10 s | 100 s, then 30 taps (45 in one control); a reversal within 2 s of a tap is a response; the level is the fitted curve's "value on the curve at the final stimulus" | Wild type falls "to approximately 0.4"; "the habituation rate (the half-life of the exponential curve) was longer at 15°" than at 23°; half-lives in figures only, the lab's reading in [pace.md § Touch](pace.md#touch) | Not measured | Full text; high for the protocol, low for the lab's figure reading |
| [Ardiel et al. 2017](https://doi.org/10.1038/s41539-017-0011-8) (PMC6161508) | 10 s (light, not tap) | "thirty 2 s light pulses delivered at 0.1 Hz" to the nociceptor ASH | Not a tap assay; listed because its protocol is the laboratory's | - | Full text; high |
| [Liu et al. 2018](https://doi.org/10.7554/eLife.36419) (PMC6054533) | 60 s | "a tap stimulus every 60 s for 30 min", 22 plates, "40,409 total animal-tap presentations" | "The inter-stimulus interval was chosen to minimize the effects of habituation" | Not measured | Full text; high |
| [McDiarmid et al. 2018](https://doi.org/10.1242/dmm.036517) (PMC6307914) | 10 s | "after a 100 s acclimatization period, 30 taps were administered at a 10 s interstimulus interval (ISI)" | "Average probability of eliciting a reversal response to 30 consecutive tap stimuli"; values in figures, not read | Not measured | Full text; high for the protocol |
| [Ardiel et al. 2018](https://doi.org/10.1098/rspb.2018.2084) (PMC6253365) | 10 s and 60 s (indexed phrases "10-s ISI", "60-s ISI"; "2-s ISI" absent) | Wild type against *cmk-1* and *ogt-1* | "animals habituate faster in response to more rapid stimulation"; the two genes "inhibit habituation at short ISIs and promote it at long ISIs" | Not in the abstract | Abstract and indexed phrases; high for the quotations, low for the set of ISIs |
| [McDiarmid et al. 2022](https://doi.org/10.1242/dmm.049359) (PMC9092656) | 10 s | 5 min to acclimate and 5 min of baseline, then "30 mechanosensory stimuli ... at a 10 s inter-stimulus interval"; "After the 30th stimulus, a 5 min rest period occurred, which was followed by the administration of a final stimulus to assess short-term memory retention of habituation (spontaneous recovery ...)" | Values in figures, not read | A single probe tap 5 min after the train; its value in figures, not read | Full text; high for the protocol |
| [Chalissery et al. 2023](https://doi.org/10.17912/micropub.biology.000760) (PMC10105302) | 10 s | "30 tap stimuli were administered with a 10-second ISI after a 600-second acclimatization period" | Values in figures, not read | Not measured | Full text; high for the protocol |
| [Kokan et al. 2025](https://doi.org/10.1371/journal.pone.0315069) (PMC12157217) | 10 s | 100 s, then 30 taps and 10 s more; "Habituated level" is "the mean response to the last 3 taps (taps 28-30)" | For wild type without ethanol, the fall in reversal probability from the initial to the final level was 0.44 ("0.47 separated the initial and final levels for the 400 mM group vs 0.44 for the 0 mM group"); at the first tap, worms without alcohol "were about three times more likely to reverse than to move forward" | Not measured | Full text; high |

The numbers the question most wants - the habituated level, the rate and the recovery time at each ISI, side by side -
are in the figures of Rankin and Broster 1992, Wicks and Rankin 1996, Rankin and Wicks 2000 and Ardiel et al. 2018,
none of which the lab could read; no paper read states them in its text. What the text does state is the direction of
every ISI effect, consistently across the laboratory's papers from 1990 on.

## What the Literature Shows

- **The field's interval is 10 s, and its train is 30 taps.** Every high-throughput tap paper read in full (Swierczek
  et al. 2011; Giles et al. 2015; McDiarmid et al. 2018 and 2022; Chalissery et al. 2023; Kokan et al. 2025) delivers 30 taps 10 s apart
  (20 in one of Swierczek et al.'s experiments) after an acclimatisation of 100 s to 10 min, scores a response, where
  the window is stated, as a reversal within 1 s (Swierczek et al.) or 2 s (Giles et al.) of the tap, and takes the
  habituated level from the last taps (28 to 30) or the fitted asymptote. Sure: high.
- **The comparison interval is 60 s.** From 1990 on, the Rankin laboratory's parametric papers pair 10 s with 60 s
  (Beck and Rankin 1993; Broster and Rankin 1994; both of Wicks and Rankin 1996; Rankin 2000, twice; Timbers et al.
  2013; Ardiel et al. 2018). Liu et al. 2018 chose 60 s "to minimize the effects of habituation". Sure: high.
- **Shorter intervals give faster and deeper habituation; longer ones give slower and shallower.** Rankin and Broster
  1992's abstract and Ardiel et al. 2018's say it; Broster and Rankin 1994 add that the processes do not mix
  symmetrically (a 60 s ISI "affected habituation at a 10-s ISI, but a 10-s ISI did not influence habituation at a
  60-s ISI"). Sure: high for the direction; the sizes unread.
- **Recovery is faster after short intervals, and it follows the interval, not the depth reached.** Rankin and Broster
  1992 ("more rapid, with short interstimulus intervals"); Wicks and Rankin 1996 (recovery "dictated by consequences of
  a given ISI, which do not impact upon habituation rate and asymptote"). The bounds in text: back "to the baseline
  after a few minutes" (Bozorgmehr et al.'s summary of Rankin et al. 1990), "recovered from habituation by 30 min after
  training" at 10 s and 60 s in young adults (Beck and Rankin 1993); the modern protocol probes once, 5 min after the
  train (McDiarmid et al. 2022). Sure: high for the direction and the 30 min bound; the curves unread. For the lab's
  modules this is the property the toy foil of E1 does not show (its recovery runs at one rate whatever the interval,
  [E1-E5.md § E1](../hypotheses/experiments/E1-E5.md)).
- **A 2 s interval has been used, rarely, and never as the field's standard.** It is in Rankin and Wicks 2000's text
  (an indexed phrase; low) and, by Bozorgmehr et al.'s summary, in Cai et al. 2009's set of 2, 5, 10 and 60 s (medium).
  The lab found no paper that states a habituation level, rate or recovery at 2 s in its text, and read no figure at
  2 s. Two features of the standard protocol bear on it directly. First, a response is scored within 1 to 2 s of the
  tap, and a reversal's duration is itself one of the habituating measures (Kokan et al. 2025 score probability,
  duration and speed; the durations are in their figures, not read), so a 2 s interval leaves the scoring window of
  one tap running into the next tap, and the baseline that Kokan et al.'s classification reads "0.3s to 0.1s before a
  tap" falls inside the previous response whenever that response outlasts 1.7 s. The journal of 2026-10-22 noticed the
  same on the toy: "at a 2 s interval many taps fall during a reversal". Second, Broster and Rankin 1994 show that what a train at
  one interval does depends on the other intervals in it; a 2 s train's result has no published wild-type curve to be
  compared with, so a Program scored on it would be scored against nothing the worm has been measured doing.
- **ISIs under 2 s belong to receptor physiology, not to this assay.** The pace note's touch entries give the
  receptors' adaptation over "a few tens of milliseconds" and currents that adapt within a stimulus; those are not
  habituation and are below what a tick resolves ([pace.md § For the Lab](pace.md#for-the-lab)).

## The Proposal, for the Maintainer to Rule On

- **The benchmark's tap-habituation assay uses two intervals, 10 s and 60 s, each a train of 30 taps**, in separate
  lives. 10 s is the field's standard and the only interval at which the lab has read anything of a wild-type curve
  (Giles et al. 2015's asymptote of about 0.4 and the lab's rough half-life reading; Kokan et al. 2025's fall of 0.44
  in reversal probability). 60 s is the laboratory's comparison interval and the one at which the ISI effects are
  reported: a Program whose habituation is faster and deeper at 10 s than at 60 s, and whose recovery is faster after
  10 s than after 60 s, shows the two properties the literature states most firmly, and a Program with a single
  decrement and a single recovery rate (the E1 foil) should fail the second. The ISI dependence is a measure worth
  having because it is the one tap-habituation result the sources agree on in words, which the lab can test for its
  direction without the figures it lacks.
- **Recovery is probed, at the least, 5 min after the train** (McDiarmid et al. 2022's protocol), and, if a life
  affords it, again at 30 min, where Beck and Rankin 1993 found young adults recovered at both intervals. The direction
  (more recovered at 5 min after 10 s than after 60 s) is the testable claim; its size waits on figures not yet read.
- **Not 2 s as the benchmark's interval.** It has no published wild-type curve the lab has read, it overlaps the
  response it measures, and realism outranks the fitter's convenience (*Realism over speed*). The 2 s train keeps its
  place as a fitting device on the toy worm, where it was found, if H13 resumes; a Program is not scored on it.
- **What this does not settle.** The response measure (probability, distance, duration or speed; Kokan et al. 2025
  show ethanol moving probability and speed in opposite directions), the response window (1 s or 2 s), the acclimatisation before the first
  tap (100 s to 10 min in the sources) and how a tap reaches a Program on the Grid (touch is the sense this assay waits
  on, [D1-D6.md § D6](../hypotheses/experiments/D1-D6.md); [STIMULI.md](../benchmark/STIMULI.md) has what a Program
  senses today) are for the benchmark's definition, when it is frozen, and are not proposed here.

How sure, overall: that 10 s and 60 s are the literature's intervals, high; that habituation is faster and deeper and
recovery faster at 10 s than at 60 s, high for the direction, with no size read; that 2 s is unsupported as a
benchmark interval, high on the protocol grounds above, medium on the literature, since the two papers that used it
were not read beyond their abstracts and a summary.

## What Remains

- Rankin and Wicks 2000 (PMC6772661), Ardiel et al. 2018 (PMC6253365) and Cai et al. 2009 (PMC2693150) in full, when
  Europe PMC's full-text service answers again or the bioRxiv preprint of Ardiel et al. 2018
  ([doi:10.1101/115972](https://doi.org/10.1101/115972)) is served; they hold wild-type curves at 2 s (the first and
  third) and at 10 s against 60 s (the second), with recovery in the first.
- Rankin and Broster 1992 and Wicks and Rankin 1996 in full, for the recovery curves by interval and the set of ISIs
  of 1992: subscription journals, a library route for the maintainer.
- The reviews Rose and Rankin 2001 and Ardiel and Rankin 2010, free on the publisher's site by OpenAlex's record but
  refused to this machine; they would give the laboratory's own statement of the ISIs it has used.
