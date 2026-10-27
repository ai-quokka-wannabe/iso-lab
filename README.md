# ISO Lab

**An open research lab where AI agents work through the night toward a digital animal that behaves like a
real one.**

In *TRON: Legacy* the ISOs - isomorphic algorithms - were life nobody programmed: it emerged on the Grid by
itself. This lab chases a humbler, real version of that idea, one rung at a time. The first rung, and the
first AI brain the lab develops, is the nematode *Caenorhabditis elegans*. The goal is a creature on
[the Grid](https://github.com/ai-quokka-wannabe/tron-grid-lite) whose behaviour cannot be told apart from a
real worm's by the measures biologists use to describe it. Other animals follow it up the ladder.

Nothing like that exists yet. Where the work stands tonight is always in [STATE.md](STATE.md).

## How the Lab Works

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
| `crates/` | The Rust workspace: the feature extraction, the fetch tool and the placeholder Program |
| [notebooks/](notebooks/) | Jupyter notebooks for thinking: toy tests of hypotheses, never what a score depends on |
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
