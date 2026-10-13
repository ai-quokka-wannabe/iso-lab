# Datasets

Every dataset the lab uses, with what someone needs to fetch exactly the same bytes. Raw downloads live in
`data/raw/`, which is never committed; a script fetches them and checks their checksums.

| Dataset | Source | Licence | Size | Checksum | Used for |
|---------|--------|---------|------|----------|----------|
| Open Worm Movement Database (candidate) | [Zenodo community](https://zenodo.org/communities/open-worm-movement-database/); Javer et al. 2018, [doi:10.1038/s41592-018-0112-1](https://doi.org/10.1038/s41592-018-0112-1) | To be checked per record | To be measured | - | Phase 0 reference recordings (not yet chosen) |

## Adding a Dataset

Add a row with every column filled: where it comes from (a stable link and the paper describing it), its
licence (and whether it may be redistributed alongside GPL-3.0 code - if not, it stays an external download),
its size, the checksum of what was downloaded, and what the lab uses it for. A dataset whose licence cannot be
found is not used.
