---
category:       General Utility
description:    Expand unix style env vars within strings.
---

# shellexpand

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [1.0.0](#1.0.0) | medium | high | positive
| [0.1.0](#0.1.0) | medium | high | positive

Pros:
* 100% safe code.
* 0 dependencies.
* Does exactly what it's supposed to do.
* Great test coverage.

Cons:
* Might want to feature-gate functions which assume std::env access under the hood.
* Env access (but that's the whole point)
* No ~username/ support.
* No %ENV% support (windows style env var syntax... or maybe that's a feature?).

1.0.0
=====

* Minor breaking changes
* Typo fixes
* Missing example fixes

0.1.0
=====

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| [src/lib.rs](src/lib.rs)                      | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | Installs travis-cargo
| Cargo.lock                                    | +1 | |
| Cargo.toml                                    | +1 | No 3rd party deps
| LICENSE-APACHE                                | +1 | |
| LICENSE-MIT                                   | +1 | |
| Readme.md                                     | +1 | Properly dual licensed

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1 | None
| miri      | -1 | "can't call foreign function: GetCurrentProcess"
| fs        | +1 | None
| io        |  0 | Hardcoded env fns might need to be featured out for browser WASM targets
| docs      | +1 | |
| tests     | +1 | Doc tests

src/lib.rs
----------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 1     | lib.rs doc comments                       | +1
| 155   | fn full_with_context                      | +1
| 229   | fn full_with_context_no_errors            | +1
| 283   | fn full                                   | +1
| 295   | struct LookupError                        | +1
| 303   | impl Display for LookupError              | +1
| 309   | impl Error for LookupError                | +1
| 314   | macro_rules try_lookup!                   | +1
| 323   | fn is_valid_var_name_char                 | +1 - includes unicode, as mentioned in docs
| 393   | fn env_with_context                       | +1
| 506   | fn env_with_context_no_errors             | +1
| 552   | fn env                                    | +1
| 584   | fn tilde_with_context                     | 0 - example incomplete
| 633   | fn tilde                                  | +1
| 637   | mod tilde_tets                            | +1
| 675   | mod env_test                              | +1
| 821   | mod full_tests                            | +1

TIL
---
Apparently you *can* use references link style for badge images.  Huh!  I should use that for reviews...
