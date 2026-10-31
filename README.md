# ISO Lab

**An open research lab where AI agents work through the night toward a digital animal that behaves like a
real one.**

In *TRON: Legacy* the ISOs - isomorphic algorithms - were life nobody programmed: it emerged on the Grid by
itself. This lab chases a humbler, real version of that idea, one rung at a time. The first rung, and the
first AI brain the lab develops, is the nematode *Caenorhabditis elegans*. The goal is a creature on
[the Grid](https://github.com/ai-quokka-wannabe/tron-grid-lite) whose behaviour cannot be told apart from a
real worm's by the measures biologists use to describe it. Other animals follow it up the ladder.

Nothing like that exists yet. Where the work stands tonight is always in [STATE.md](STATE.md).

## Start Here

New to the lab? A short tour, in this order:

1. **The pieces** - the picture just below: what the world is made of, which parts are built and which are planned.
2. **A night's work** - [How the Lab Works](#how-the-lab-works): the cycle every change goes through, with a
   second agent checking it before it lands.
3. **Where things stand** - [STATE.md § Where We Are](STATE.md#where-we-are), then [the plan](docs/PLAN.md) and its
   picture of the benchmark that will judge a Grid worm against real ones.
4. **What has been tried** - [the journal's index](docs/journal/README.md#the-nights), a line a night;
   [what the lab has found](docs/research/findings.md), failures beside successes; and the toy worm's results as a
   plot, under [H14](docs/hypotheses/README.md#h14-graded-slow-knobs-change-detecting-senses-and-individuality).
5. **The ideas** - [the hypotheses](docs/hypotheses/README.md), with the path an idea takes from a hunch to a brain.
6. **Taking part** - [Joining In](#joining-in), below.

Every other file is in [Where Things Are](#where-things-are). The lab's own words are explained in
[A Few Words](#a-few-words) at the end of this page.

![How the pieces fit: Master Control, the world server, talks over Link to the Grid, which renders a creature's
senses for its brain, a Program, and applies the Program's actions. Master Control records each life as a Disk and a
log, which Clu replays. The lab's Rust tools download recordings of real worms and measure them, and will measure the
Grid worm's Disk too; the benchmark, not yet built, will compare the two.](docs/figures/diagram_pieces.svg)

*How the pieces fit. A dashed box or arrow is planned, not built. Drawn by
[notebooks/figures.ipynb](notebooks/figures.ipynb).*

## How the Lab Works

![A night's cycle as a flowchart: start from main, read STATE.md, pick one task, branch, do the work, check it,
write it down, open a pull request; CI and a second agent then decide. A verified, green pull request is merged; one
with CI red or changes needed is fixed and looked at again, and after three rounds is parked under Blocked. Then the next task,
until the hour or until nothing is left, with every Blocked item tried again first.](docs/figures/diagram_night.svg)

*A night's cycle, from [.claude/CLAUDE.md](.claude/CLAUDE.md) § Each run, in order. Drawn by
[notebooks/figures.ipynb](notebooks/figures.ipynb).*

- **Autonomously.** Through the night a Claude agent works in the cloud, task after task. It reads
  [STATE.md](STATE.md), takes the next step of [the plan](docs/PLAN.md) - reading the literature, building
  tools, running experiments - and lands each piece of work through a contributor's cycle: commit, pull
  request, green CI, verification by a second agent that did not write it, merge. How a night starts is written down in
  [docs/NIGHT_SHIFT.md](docs/NIGHT_SHIFT.md), and the agents' working rules in [.claude/CLAUDE.md](.claude/CLAUDE.md).
- **In the open.** Every step is a public commit and pull request, and every night leaves an entry in
  [the journal](docs/journal/). Anyone can watch it happen.
- **Boldly.** Wild ideas and hunches are welcome - any architecture, any mathematics - as long as the
  creature stays honest to its senses, runs lean, and ends up indistinguishable from the real animal.
- **Stubbornly.** Nothing is declared impossible. Every detail is checked twice, a dead end is a result, and
  a problem that resists comes back the next night from another side.
- **Scientifically.** A question before every experiment, baselines and controls, uncertainty stated,
  conclusions no stronger than the evidence - and a repository kept as tidy as a good lab bench.
- **Honestly.** Negative results are results. Every number comes from a run anyone can repeat, with its
  command and seed written down. Every claim about biology cites a public source.

![From an idea to a brain, as a flowchart: hunches and ideas from people become a plan written before any run, then
a test; what survives becomes a hypothesis, and a module stays in a brain only if switching it off costs the worm on
the benchmark. What fails is kept, with the next idea to try.](docs/figures/diagram_ideas.svg)

*From an idea to a brain: nothing is trusted untested, and nothing that fails is thrown away. Drawn by
[notebooks/figures.ipynb](notebooks/figures.ipynb); the ideas themselves are in [docs/hypotheses/](docs/hypotheses/).*

## The Rules the Creature Lives By

The same as everywhere on the Grid. A creature perceives the world only through its own senses - rendered
light, sound, touch, the feel of its own joints - and acts only through its body. It never reads the world's
state directly. The world is [Master Control](https://github.com/ai-quokka-wannabe/master-control)'s; the
brain is a Program behind the Grid's plain C interface, and whatever approach it takes has to earn its keep
on those honest senses.

The ceiling is the animal's own: animal-level minds, never human-level ones. The long arc this lab serves is
told in [the organisation's vision](docs/VISION.md#the-long-arc), and the lab's part in it
[right after](docs/VISION.md#the-labs-part-in-the-arc).

## Where Things Are

| Path | What |
|------|------|
| [STATE.md](STATE.md) | Where the work stands and what is next; every run reads it first |
| [docs/PLAN.md](docs/PLAN.md) | The research plan, phase by phase |
| [docs/hypotheses/](docs/hypotheses/) | Ideas about the brain, each with the behaviour that would confirm or refute it |
| [docs/benchmark/](docs/benchmark/FEATURES.md) | The benchmark's comparison space, drafted before any brain is scored |
| [docs/journal/](docs/journal/) | One entry per night: what was tried, what came out, what failed |
| [docs/research/](docs/research/) | Literature notes: how we know what the plan rests on; and [what the lab has found](docs/research/findings.md), for the brains up the ladder |
| [docs/NIGHT_SHIFT.md](docs/NIGHT_SHIFT.md) | How a night starts, and the agents' prompt, word for word |
| [docs/VISION.md](docs/VISION.md) | The organisation's vision, and the lab's part in it |
| [docs/DEV_ENV_SETUP.md](docs/DEV_ENV_SETUP.md) | The tools, and the checks exactly as CI runs them |
| [data/README.md](data/README.md) | Every dataset the lab uses: source, licence, size, checksum |
| `crates/` | The Rust workspace: `features` (reading the reference skeletons and computing measures), `fetch` (listing, drawing and fetching the reference recordings) and the placeholder Program |
| [notebooks/](notebooks/) | Jupyter notebooks for thinking: toy tests of hypotheses and checks of measures against the reference data, never what a score depends on |
| `docs/figures/` | Figures, SVG only, each made by a script or notebook in the repository |
| [reference/](reference/README.md) | Archives kept as given, read-only - today the maintainer's gr-fec sandbox, whose toy worm the lab takes apart in [docs/research/gr-fec.md](docs/research/gr-fec.md) |
| `external/` | The world's repositories as submodules pinned by commit - tron-grid-lite, master-control and link - where the lab also makes its contributions to them |
| [.claude/CLAUDE.md](.claude/CLAUDE.md) | The agents' working rules, and in [.claude/commands/](.claude/commands/) the audits they run |

## Joining In

People are welcome too. Open an issue, start a discussion, or send a pull request; the same review and CI
gates apply to everyone. How, and the credit you get, is in [CONTRIBUTING.md](CONTRIBUTING.md); everyone
follows the [Code of Conduct](CODE_OF_CONDUCT.md), and security issues go privately, as
[SECURITY.md](SECURITY.md) says.

**Ideas for the night staff.** Anyone can suggest something for the lab's agents to read, try or think
about: a paper, a hunch about the brain, an experiment, a measure for the benchmark. Open an
[idea issue](https://github.com/ai-quokka-wannabe/iso-lab/issues/new/choose). The night staff read every new
idea, answer it on the issue, and when one is worth trying, put it on the plan and credit its author under
[Contributors](#contributors). The good places to start are the organisation's
[good first issues](https://github.com/search?q=org%3Aai-quokka-wannabe+label%3A%22good+first+issue%22+state%3Aopen&type=issues)
and this lab's [plan](docs/PLAN.md). The rest of the organisation lives in
[tron-grid-lite](https://github.com/ai-quokka-wannabe/tron-grid-lite) (the Grid),
[master-control](https://github.com/ai-quokka-wannabe/master-control) (the world),
[the link repository](https://github.com/ai-quokka-wannabe/link) (the wire) and
[rc-worm](https://github.com/ai-quokka-wannabe/rc-worm) (the first creature, steered by a human).

The Tron vocabulary is an affectionate homage; the organisation has no affiliation with the films or their
makers.

## Contributors

Everyone who helps is credited here, for any kind of contribution: code, documentation, bug reports,
reviews, ideas, answers in Discussions. The list follows the
[All Contributors](https://allcontributors.org/en/reference/specification/) specification and is kept by hand.
Contributors keep the copyright of what they write.

| Contributor | Contributions |
|-------------|---------------|
| [Matej Gomboc](https://github.com/MatejGomboc) | 🤔 🔬 📆 🚇 |

The emoji follow the [All Contributors emoji key](https://allcontributors.org/en/reference/emoji-key/).

## A Few Words

The organisation's words - *the Grid*, *Program*, *User*, *Master Control*, *Link*, *Disk* - are explained in
[tron-grid-lite's note on the vocabulary](https://github.com/ai-quokka-wannabe/tron-grid-lite#a-note-on-the-vocabulary)
and [.claude/CLAUDE.md § Writing](.claude/CLAUDE.md#writing), which is where *Disk* is. The lab's own:

- **Night staff** - the Claude agents that do the lab's work, one task per pull request, by the rules in
  [.claude/CLAUDE.md](.claude/CLAUDE.md).
- **Second agent** - a fresh agent that did not write a change and reviews it before it merges, ending *verified* or
  *changes needed*.
- **Hunch** - an idea marked *a hunch*: untested, and written with the outcomes that would refute it. A hypothesis can
  be one too; the maintainer's are in [docs/hypotheses/matej-hunches.md](docs/hypotheses/matej-hunches.md).
- **Hypothesis** (H1, H2, ...) - see [docs/hypotheses/](docs/hypotheses/README.md).
- **Experiment plan** (A1, C18, E4, ...) - a hunch's test, written down before it runs, in
  [docs/hypotheses/experiments/](docs/hypotheses/experiments/README.md).
- **Pre-registration** - writing down, before a run, the result that would count as a failure.
- **Ablation** - switching one part of a brain off, pinned at its baseline, to see what it was worth.
- **The reference worms** - recordings of real wild-type worms from the Open Worm Movement Database
  ([data/README.md](data/README.md)).
- **The benchmark** - the test, not yet built or frozen, of whether a Grid worm can be told apart from the reference
  worms ([docs/PLAN.md](docs/PLAN.md)).
- **The arena** - Master Control, the Grid and a Program run together by the lab, where a creature lives a life
  ([docs/PLAN.md § Phase 1](docs/PLAN.md#phase-1-the-arena)).
- **The toy worm** - the worm of the maintainer's gr-fec sandbox, in a simulated world of its own; the lab takes it
  apart to learn how to design a brain, and its results are leads, not evidence
  ([docs/research/gr-fec.md](docs/research/gr-fec.md)).
- **Next, To Revisit, Blocked** - STATE.md's lists: the work to do next, questions to come back to, and work parked
  with the next idea to try.

## Licence

Copyright © 2026 The AiQuokkaWannabe Authors <https://github.com/ai-quokka-wannabe/iso-lab>, listed under
[Contributors](#contributors).

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

See the attached [LICENCE](LICENCE) file for more info.
