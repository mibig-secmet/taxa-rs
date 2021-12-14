# MIBiG taxonomy handling

NCBI taxdump handling for MIBiG

This package is designed to help the MIBiG database to handle NCBI taxid lookups using
[NCBI taxdump data](https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/new_taxdump/).

The package contains both a Rust library as well as a command line tool `mibig-taxa` to
manage a local JSON-based cache of interesting taxa, allowing bulk database imports to speed up
compared to parsing directly from the taxdump files.


## Installation

```
cargo install mibig-taxa
```

## Usage

In order to use it, grab the [latest taxdump collection](https://ftp.ncbi.nlm.nih.gov/pub/taxonomy/new_taxdump/)
and extract it. You'll also need a directory of MIBiG BGC entry json files.

Then, you can run the command line tool like

```
mibig-taxa init --cache mibig_cache.json --datadir path/to/mibig/jsons/ --mergeddump path/to/merged.dmp --taxdump path/to/rankedlineage.dmp
```
to initialise the cache and
```
mibig-taxa list --cache mibig_cache.json
```
to list the contents of the provided cache.

For using the library, see the API documentation.

## License

Licensed under the Apache License, Version 2.0
([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as Apache-2.0, without any additional terms or conditions.
