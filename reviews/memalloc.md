---
category:       Unsound
crev:           dangerous
description:    Super brittle/dangerous at a fundamental level. Avoid.
---

# 0.1.0

API design is super brittle.  Returning uninitialized memory seems like UB-bait.

Detail
------

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| [src/lib.rs](src/lib.rs)                      | -1    | Soundish, but unsafe as heck APIs.
| .cargo-ok                                     | +1    | |
| .gitignore                                    | +1    | |
| .travis.yml                                   | +1    | |
| Cargo.toml                                    | +1    | |
| README.md                                     | +1    | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1 | Soundish, but unsafe as heck API design.
| fs        | +1 | None
| io        | +1 | None
| docs      | +1 | |
| tests     | +1 | |

src/lib.rs
----------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 23    | allocate      | This looks like it returns uninitialized memory (only sizes capacity not actual size).  UB bait.  Unaligned.
| 38    | reallocate    | Minimal debug checks, constructs Vec from_raw_parts with size when the real vec had size 0.

