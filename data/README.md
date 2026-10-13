# Datasets

Every dataset the lab uses, with what someone needs to fetch exactly the same bytes. Raw downloads live in
`data/raw/`, which is never committed; a script fetches them and checks their checksums.

| Dataset | Source | Licence | Size | Checksum | Used for |
|---------|--------|---------|------|----------|----------|
| Open Worm Movement Database, N2 single-worm recordings (chosen for Phase 0) | [Zenodo community](https://zenodo.org/communities/open-worm-movement-database/); Javer et al. 2018, [doi:10.1038/s41592-018-0112-1](https://doi.org/10.1038/s41592-018-0112-1) | CC-BY-4.0, every one of the 1792 records; fetched by script, never committed | Features files 46.6 GB in all, median 19.3 MB a record | MD5 per file in the record's Zenodo metadata (checked for record 1011832) | Phase 0 reference recordings |

## The Phase 0 Survey

The survey of public wild-type *C. elegans* behaviour recordings, made on 2026-10-13 (item 1 of the first
night; [the journal](../docs/journal/2026-10-13.md) has the commands). Two candidates were examined; others
remain to be surveyed.

### Open Worm Movement Database: the N2 records

- **What it is**: single-worm tracking of N2 hermaphrodites from the Schafer lab, published on Zenodo with
  Javer et al. 2018. Each recording is one Zenodo record, titled
  `N2 Schafer Lab N2 (Bristol, UK) | <timestamp>`. The metadata of record 1011832, read in full, names the protocol as the method of
  [Yemini et al. 2013](https://doi.org/10.1038/nmeth.2560), the worm transferred to the arena 30 minutes before
  recording, on NGM low-peptone agar, and the tracking software as Tierpsy at commit `cbfc23eb`.
- **How many**: 1792 records answer the query `title:"N2 Schafer Lab N2"` in the community, recorded from
  2009 to 2016. In 1747 of them the files are named *on food*; the other 45 carry no such word, and their
  condition is not yet read.
- **Licence**: CC-BY-4.0 on all 1792. Attribution is owed to the authors and the record; the lab keeps the
  files an external download rather than committing them.
- **Files**: three per record, by the metadata of all 1792; the MD5 checksums and the contents below were
  read in record 1011832 alone.

    | File | What it holds | Size, all 1792 | Median |
    |------|---------------|----------------|--------|
    | `<name>_features.hdf5` | Skeletons and contours, 57 per-frame feature series, 730 per-worm summary features, motion events | 46.6 GB | 19.3 MB |
    | `<name>.wcon.zip` | The track in WCON, a JSON format for worm tracks: metadata, units, times and skeleton points | 45.4 GB | 18.3 MB |
    | `<name>.hdf5` | The record's largest file; its contents were not opened in this survey | 813.6 GB | 262.5 MB |

- **What one record looks like**, read from record [1011832](https://doi.org/10.5281/zenodo.1011832): 29256
  frames at 32.57 frames per second (about 15 minutes), a 49-point skeleton per frame, positions in
  micrometres, times in seconds. The HDF5 is written by PyTables; any HDF5 reader opens it.
- **Download**: `https://zenodo.org/api/records/<id>/files/<file name, URL-encoded>/content`, checked against
  the MD5 in `https://zenodo.org/api/records/<id>`. The connection to Zenodo drops now and then, so a fetch
  retries.

### CeMEE Multi-Worm Tracker data

- **What it is**: Multi-Worm Tracker output for the founders, recombinant inbred lines, outbred populations
  and mutation-accumulation lines of the *C. elegans* Multiparent Experimental Evolution panel, many worms to
  a plate for 15 to 20 minutes ([doi:10.5281/zenodo.4074963](https://doi.org/10.5281/zenodo.4074963); Noble et
  al. 2017, [doi:10.1534/genetics.117.300406](https://doi.org/10.1534/genetics.117.300406)). Only the
  record's metadata was read; no archive was opened.
- **Licence**: CC-BY-4.0, but the record asks: "Some of these data are unpublished at present, please contact
  us if you would like to use them". The lab does not use it until the maintainer decides.
- **Size**: four archives, 0.55 GB (founders), 1.86 GB (populations), 4.42 GB (mutation accumulation) and
  41.5 GB (recombinant inbred lines), each with an MD5.
- **Why not first**: the authors ask to be contacted before their data are used, and N2 is one founder among many
  tracked a plate at a time; the Open Worm Movement Database gives single N2 worms with skeletons and features
  already computed.

### Recommendation

The Open Worm Movement Database's N2 *on food* records, starting from the features files: one licence across
all of them, a checksum for every file, the skeleton and the published features in a file of about 20 MB, and
enough recordings (1747) to hold real worms out against real worms when the test is checked (Phase 0 step 5 of
[the plan](../docs/PLAN.md)). The first subset to fetch, and how it is drawn, is the feature-extraction task's
to fix before it computes anything; nothing above 2 GB has been downloaded.

## Adding a Dataset

Add a row with every column filled: where it comes from (a stable link and the paper describing it), its
licence (and whether it may be redistributed alongside GPL-3.0 code - if not, it stays an external download),
its size, the checksum of what was downloaded, and what the lab uses it for. A dataset whose licence cannot be
found is not used.
