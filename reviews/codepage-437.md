---
category:       I/O
description:    Codepage 437 conversion functions
---

# codepage-437

Pros
* Encoding + decoding
* 100% Safe code

Cons
* No lossy encodes
* No handling of [combining character](https://en.wikipedia.org/wiki/Combining_character)s
* String alloc on error

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.0]   | medium | medium | positive | Full Reviewex



[0.1.0]: #0.1.0
<h2 name="0.1.0">0.1.0</h2>

Full Review

| File                                                          | Rating | Notes |
| ------------------------------------------------------------- | ------ | ----- |
| dialect-specs/cp437_control/documentation.md                  | ✔️
| dialect-specs/cp437_control/overlaps.rs                       | ✔️
| dialect-specs/cp437_control/values.tsv                        | ✔️
| dialect-specs/cp437_control/variants.tsv                      | ✔️
| dialect-specs/cp437_wingdings/documentation.md                | ✔️
| dialect-specs/cp437_wingdings/overlaps.rs                     | ✔️
| dialect-specs/cp437_wingdings/values.tsv                      | ✔️
| dialect-specs/cp437_wingdings/variants.tsv                    | ✔️
| src/decode.rs                                                 | ✔️ | infalliable
| src/dialect.rs                                                | ✔️
| src/encode.rs                                                 | ✔️
| src/lib.rs                                                    | ✔️
| test-data/**                                                  | ❔ | Opaque binaryish, probably fine
| tests/cp437_control/decode/borrow_from_cp437/borrowing.rs     | ✔️
| tests/cp437_control/decode/borrow_from_cp437/conversion.rs    | ✔️
| tests/cp437_control/decode/borrow_from_cp437/mod.rs           | ✔️
| tests/cp437_control/decode/from_cp437.rs                      | ✔️
| tests/cp437_control/decode/mod.rs                             | ✔️
| tests/cp437_control/encode/to_cp437/borrowing.rs              | ✔️
| tests/cp437_control/encode/to_cp437/conversion.rs             | ✔️
| tests/cp437_control/encode/to_cp437/mod.rs                    | ✔️
| tests/cp437_control/encode/encode.rs                          | ✔️
| tests/cp437_control/encode/into_cp437.rs                      | ✔️
| tests/cp437_control/encode/mod.rs                             | ✔️
| tests/cp437_control/mod.rs                                    | ✔️
| tests/cp437_wingdings/decode/borrow_from_cp437/borrowing.rs   | ✔️
| tests/cp437_wingdings/decode/borrow_from_cp437/conversion.rs  | ✔️
| tests/cp437_wingdings/decode/borrow_from_cp437/mod.rs         | ✔️
| tests/cp437_wingdings/decode/from_cp437.rs                    | ✔️
| tests/cp437_wingdings/decode/mod.rs                           | ✔️
| tests/cp437_wingdings/encode/to_cp437/borrowing.rs            | ✔️
| tests/cp437_wingdings/encode/to_cp437/conversion.rs           | ✔️
| tests/cp437_wingdings/encode/to_cp437/mod.rs                  | ✔️
| tests/cp437_wingdings/encode/encode.rs                        | ✔️
| tests/cp437_wingdings/encode/into_cp437.rs                    | ✔️
| tests/cp437_wingdings/encode/mod.rs                           | ✔️
| tests/cp437_wingdings/mod.rs                                  | ✔️
| tests/dialect/mod.rs                                          | ✔️
| tests/dialect/remap.rs                                        | ✔️
| tests/mod.rs                                                  | ✔️
| .cargo-ok                                                     | ✔️
| .gitignore                                                    | ✔️
| .travis.yml                                                   | ⚠️ | `gpg --import`, `openssl ...` key manipulation (docs builds - why not switch to docs.rs?)
| appveyor.yml                                                  | ✔️
| build.rs                                                      | ✔️ | fs, io
| Cargo.toml                                                    | ✔️ | MIT
| Cargo.toml.orig                                               | ✔️ | MIT
| codepage-437.subline-project                                  | ✔️
| LICENSE                                                       | ✔️ | MIT
| README.md                                                     | ✔️ | Badges
| rustfmt.toml                                                  | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | In build.rs, keep an eye on
| io        | ✔️ | In build.rs, keep an eye on
| docs      | ✔️ | Excellent
| tests     | ✔️ | Excellent

<!-- Templates

✔️
❔
⚠️
❗️

#### :exclamation:  \[1\] Unsound ...
#### \[1\] Note ...
[1]: #exclamation--1-unsound-...
[2]: #1-note-...
[user/repository#1]: https://github.com/user/repository/issues/1
[user/repository#1]: https://github.com/user/repository/pull/1



# DiffVersionTemplate

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| 

# Full File Version Template

| Line  | Notes |
| -----:| ----- |
| 

-->
