---
category:       General Utility
description:    std::time::Instant alternative that doesn't panic on wasm targets.
---

# instant

Pros:
- Doesn't panic!
- Just std::time::Instant on native, no performance hit or anything

Cons:
- Just std::time::Instant on native, easy to accidentally use new std APIs unavailable on wasm
- f64 repr for Instant makes me nervous
- Unusual license choice for rust projects
- Could use more test coverage


| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.1.2](#0.1.2) | medium | high | positive |

# 0.1.2

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| .circleci/config.yml                          | +1 | |
| src/lib.rs                                    | +1 | |
| src/native.rs                                 | +1 | I would've preferred a wrap Instant for ensuring compat, but sure.
| src/wasm.rs                                   |  0 | f64 repr might accumulate poorly
| tests/wasm.rs                                 |  0 | test_instant_now could spuriously fail if elapsed() == 0, would like to see more test coverage
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | Overkill
| AUTHORS                                       | +1 | |
| Cargo.lock                                    |  0 | Pointless, shouldn't be part of the package
| Cargo.toml                                    | +1 | BSD-3-Clause
| Cargo.toml.orig                               | +1 | BSD-3-Clause
| LICENSE                                       | +1 | BSD-3-Clause?
| README.md                                     | +1 | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1 | None
| fs        | +1 | None
| io        | +1 | None
| docs      |  0 | Mostly unnecessary
| tests     |  0 | Could use more
