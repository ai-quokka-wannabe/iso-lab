# Datasets

Every dataset the lab uses, with what someone needs to fetch exactly the same bytes. Raw downloads live in
`data/raw/`, which is never committed; a script fetches them and checks their checksums.

| Dataset | Source | Licence | Size | Checksum | Used for |
|---------|--------|---------|------|----------|----------|
| Open Worm Movement Database, N2 single-worm recordings (chosen for Phase 0) | [Zenodo community](https://zenodo.org/communities/open-worm-movement-database/); Javer et al. 2018, [doi:10.1038/s41592-018-0112-1](https://doi.org/10.1038/s41592-018-0112-1) | CC-BY-4.0; fetched by script, never committed | See [The Phase 0 Survey](#the-phase-0-survey) | MD5 per file in each record's Zenodo metadata | Phase 0 reference recordings |

## The Phase 0 Survey

The survey of public wild-type *C. elegans* behaviour recordings, made on 2026-10-13 (item 1 of the first
night; [the journal](../docs/journal/2026-10-13.md) has the commands). Two candidates were examined; others
remain to be surveyed.

### Open Worm Movement Database: the N2 records

- **What it is**: single-worm tracking of N2 worms from the Schafer lab, published on Zenodo with Javer et al.
  2018. Each recording is one Zenodo record, titled `N2 Schafer Lab N2 (Bristol, UK) | <timestamp>`, and its
  description carries structured fields - sex, stage, food, habituation among them. Record 1011832 names the
  protocol as the method of [Yemini et al. 2013](https://doi.org/10.1038/nmeth.2560), NGM low-peptone agar,
  and the tracking software as Tierpsy at commit `cbfc23eb`.
- **How many**: 1792 records answer the query `title:"N2 Schafer Lab N2"`, recorded 2009 to 2016. Not all are
  wild-type adults on food: by their fields, 58 are male, 3 are L3 larvae, 28 have no food and 26 had no
  30-minute wait. 1677 are adult hermaphrodites on OP50 after a 30-minute wait.
- **An open question**: 31 of those 1677 have file names that do not begin with *N2* and a space. Some are only other
  spellings (`n2 on food`, `N2_on food`), but others are named after mutants (`dop-1`, `dop-2`, `dop-3`,
  `odr-3`), or oddly (`M2`, `Ne`). Either the file name or the metadata is wrong; until that is traced, all 31
  are left out.
- **Licence**: CC-BY-4.0 on all 1792. Attribution is owed to the authors and the record; the lab keeps the
  files an external download rather than committing them.
- **Files**: three per record, each with an MD5 checksum in the record's metadata (true of every file of the
  1792). Sizes in GB of 10^9 bytes:

    | File | What it holds | All 1792 | The 1677 | Median, the 1677 |
    |------|---------------|----------|----------|------------------|
    | `<name>_features.hdf5` | Skeletons and contours, a per-frame table of 57 columns (53 features, the motion mode and three bookkeeping columns), 730 per-worm summary features, motion events | 49.36 GB | 44.11 GB | 19.56 MB |
    | `<name>.wcon.zip` | The track in WCON, a JSON format for worm tracks: metadata, units, times and skeleton points | 48.37 GB | 43.07 GB | 18.55 MB |
    | `<name>.hdf5` | The record's largest file; its contents were not opened in this survey | 894.64 GB | 819.37 GB | 264.56 MB |

- **What one record looks like**, read from record [1011832](https://doi.org/10.5281/zenodo.1011832): 29256
  frames at 32.57 frames per second (about 15 minutes), positions in micrometres, times in seconds. Only
  15815 frames (54.1%) carry a skeleton of 49 points; the rest are NaN, and the benchmark has to say how it
  treats them. The HDF5 is written by PyTables; any HDF5 reader opens it.
- **Download**: `https://zenodo.org/api/records/<id>/files/<file name, URL-encoded>/content`, checked against
  the MD5 in `https://zenodo.org/api/records/<id>`. The connection to Zenodo drops now and then, so a fetch
  retries; and a listing must be sorted (`sort=oldest`), because unsorted pages repeat some records and miss
  others.

### CeMEE Multi-Worm Tracker data

- **What it is**: Multi-Worm Tracker output for the founders, recombinant inbred lines, outbred populations
  and mutation-accumulation lines of the *C. elegans* Multiparent Experimental Evolution panel, many worms to
  a plate for 15 to 20 minutes ([doi:10.5281/zenodo.4074963](https://doi.org/10.5281/zenodo.4074963); Noble et
  al. 2017, [doi:10.1534/genetics.117.300406](https://doi.org/10.1534/genetics.117.300406)). Only the
  record's metadata was read; no archive was opened.
- **Licence**: CC-BY-4.0, but the record asks: "Some of these data are unpublished at present, please contact
  us if you would like to use them so we can avoid any overlaps". The lab does not use it until the maintainer decides.
- **Size**: four archives, 0.55 GB (founders), 1.86 GB (populations), 4.42 GB (mutation accumulation) and
  41.5 GB (recombinant inbred lines), each with an MD5.
- **Why not first**: the authors ask to be contacted before their data are used, and N2 is one founder among many
  tracked a plate at a time; the Open Worm Movement Database gives single N2 worms with skeletons and features
  already computed.

### Recommendation

The Open Worm Movement Database's N2 records, chosen by their metadata - adult hermaphrodites on OP50 after a
30-minute wait, less the 31 whose file names are in question: 1646 records. One licence across them, a
checksum for every file, the skeletons and published features in files of about 20 MB, and enough recordings
to hold real worms out against real worms when the test is checked (Phase 0 step 5 of
[the plan](../docs/PLAN.md)). The first subset to fetch, and how it is drawn, is the feature-extraction task's
to fix before it computes anything; nothing above 2 GB has been downloaded.
