---
category:       Debugging
description:    DWARF debug info parsing.
---

# gimli

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.15.0](#0.15.0) | low | medium | positive |

# 0.15.0

Looks nice and solid.  Well documented, well tested, code coverage, safe + sound.
Large crate - so I haven't taken the time to thoroughly audit everything - but I have at least skimmed over all code.
I've written some of my own partial DWARF parser in C++ in the past... parts look appropriately familiar.

Detail
------

| File                                      | Rating | Notes |
| ----------------------------------------- | ------:| ----- |
| benches/bench.rs                          | +1    | fs
| examples/dwarfdump.rs                     | +1    | fs
| fixtures/self/*                           |       | unreviewed binary data (used in test fixtures, probably OK)
| fixtures/self/README.md                   | +1    | |
| releases/friends.sh                       | +1    | |
| releases/release-announcement-template.md | +1    | |
| src/abbrev.rs                             | +1    | |
| src/aranges.rs                            | +1    | |
| src/cfi.rs                                | +1    | |
| src/constants.rs                          | +1    | |
| src/endianity.rs                          | +1    | |
| src/leb128.rs                             | +1    | |
| src/lib.rs                                | +1    | |
| src/line.rs                               | +1    | |
| src/loc.rs                                | +1    | |
| src/lookup.rs                             | +1    | |
| src/op.rs                                 | +1    | |
| src/parser.rs                             | +1    | |
| src/pubnames.rs                           | +1    | |
| src/pubtypes.rs                           | +1    | |
| src/ranges.rs                             | +1    | |
| src/reader.rs                             | +1    | |
| src/str.rs                                | +1    | |
| src/test_util.rs                          | +1    | |
| src/unit.rs                               | +1    | |
| tests/parse_self.rs                       | +1    | |
| .cargo-ok                                 | +1    | |
| .gitignore                                | +1    | |
| .travis.yml                               | 0     | I haven't reviewed travis-cargo |
| Cargo.toml                                | +1    | |
| Cargo.toml.orig                           | +1    | |
| CONTRIBUTING.md                           | +1    | |
| coverage                                  | +1    | |
| format                                    | +1    | |
| LICENSE-APACHE                            | +1    | |
| LICENSE-MIT                               | +1    | |
| README.md                                 | +1    | |
| rustfmt.toml                              | +1    | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | Single `match unsafe { memmap::Mmap::map(&file) }` |
| fs        | +1    | Safe, only used in benches/examples/tests
| io        | +1    | Some io::{stderr, Error, Write} - all sane
| docs      | +1    | Lots of 'em!
| tests     | +1    | Lots of 'em!

TIL
---
travis-cargo
