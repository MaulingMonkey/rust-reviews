---
category:       Build Utility
description:    Parse `cargo metadata` and `cargo build --message-format=json`.
---

# cargo_metadata

Pros:
* Way better than parsing it yourself
* Safe code

Cons:
* If you're feeling particularly paranoid, `cargo metadata` could be passed bad
  feature names (see 0.8.2 review for details)

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.9.1](#0.9.1) | medium | medium | positive
| [0.9.0](#0.9.0) | medium | medium | positive
| [0.8.2](#0.8.2) | medium | medium | positive

0.9.1
=====

| Diff                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| .cargo_vcs_info.json                          | +1 | |
| Cargo.toml                                    | +1 | |
| Cargo.toml.orig                               | +1 | |
| src/dependency.rs                             | +1 | |
| src/lib.rs                                    | +1 | |
| src/messages.rs                               | +1 | |
| tests/test_samples.rs                         | +1 | |

0.9.0
=====

| Diff                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| .cargo_vcs_info.json                          | +1 | |
| Cargo.toml                                    | +1 | |
| Cargo.toml.orig                               | +1 | |
| src/errors.rs                                 | +1 | Added `Error::NoJson`
| src/lib.rs                                    | +1 | Various safe but breaking code changes
| src/messages.rs                               | +1 | |
| tests/selftest.rs                             | +1 | |
| tests/test_samples.rs                         | +1 | |

0.8.2
=====

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| src/dependency.rs                             | +1 | |
| src/diagnostic.rs                             | +1 | |
| src/errors.rs                                 | +1 | |
| [src/lib.rs](src/lib.rs)                      | 0 | MetadataCommand makes me slightly paranoid
| src/messages.rs                               | +1 | |
| tests/selftest.rs                             | +1 | |
| tests/test_samples.rs                         | +1 | |
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | 1.32.0 MSRV
| Cargo.toml                                    | +1 | |
| Cargo.toml.orig                               | +1 | |
| LICENSE-MIT                                   | +1 | |
| README.md                                     | +1 | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | None
| fs        | +1    | None
| io        | 0     | Can invoke `cargo metadata`.  Looks sane, but if passed malicious feature names etc...
| docs      | +1    | |
| tests     | +1    | |

src/lib.rs
----------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 495 | exec | shell access, and I'm paranoid about shell param escaping...
| 500 | exec | shell access, and I'm paranoid about shell param escaping...
