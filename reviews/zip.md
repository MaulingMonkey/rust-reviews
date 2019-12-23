---
category:       General Utility
description:    Zipping/unzipping .zip archives.
---

Preamble
========

General review concerns:
* Path traversal attacks - appears protected against
* Zipbombs - DOESN'T appear protected against, possible DoS vector.
* Special names like `CON` etc. - DOESN'T appear protected against.
* Filesystem permissions.

0.5.3
=====

Replaced libflate with flate2, minor touchups.  LGTM.

0.5.2
=====

Looks like a solid crate.  A few minor concerns:

* 755 permissions.  Necessary, but bandied about.
* Unsanitized path names are accessible, easy to misuse.
* Doesn't ban access to `CON` or similar.
* Lacks fuzz tests

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| benches/read_entry.rs                         | +1    | |
| examples/extract_lorem.rs                     | +1    | |
| examples/extract.rs                           | +1    | |
| examples/file_info.rs                         | +1    | |
| examples/stdin_info.rs                        | +1    | |
| examples/write_dir.rs                         | 0     | 755 permissions make me slightly nervous, but I think it's safe
| examples/write_sample.rs                      | 0     | 755 permissions make me slightly nervous, but I think it's safe
| script/doc-upload.cfg                         | +1    | |
| src/compression.rs                            | +1    | |
| src/cp437.rs                                  | +1    | |
| src/crc32.rs                                  | +1    | |
| src/lib.rs                                    | +1    | |
| src/read.rs                                   | +1    | |
| src/result.rs                                 | +1    | |
| src/spec.rs                                   | +1    | |
| [src/types.rs](src/types.rs)                  | 0     | Could be a little more defensive towards misue, but pretty solid.
| src/write.rs                                  | +1    | |
| tests/data/*.zip                              |       | Unreviewed... probably OK though
| tests/end_to_end.rs                           | +1    | |
| tests/invalid_date.rs                         | +1    | |
| tests/zip64_large.rs                          | +1    | |
| .gitignore                                    | +1    | |
| .travis.yml                                   | -1    | | curl random urls
| appveyor.yml                                  | -1    | | sourceforge mingw urls
| Cargo.toml                                    | +1    | |
| Cargo.toml.orig                               | +1    | |
| LICENSE                                       | +1    | MIT
| README.md                                     | +1    | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | No unsafe code
| fs        | +1    | Examples/tests appear safe.
| io        | +1    | |
| docs      | +1    | |
| tests     | +1    | Could use more fuzzing tests

src/types.rs
------------
| Line | Notes |
| ----:| ----- |
| 215   | I'd like this to have a scarier name... but eh, at least it's sound.
| 250   | This drops invalid components... I think it should return an error on invalid components.  But at least it's sound and shouldn't be vulnerable to path navigation attacks?
| 250   | This doesn't forbid `CON` or similar.
| 298   | *Excellent* test, this is exactly what I want to see!
