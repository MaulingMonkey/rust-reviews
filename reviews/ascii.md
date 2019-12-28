---
category:       Serialization
description:    ASCII conversion and parsing.
---

0.9.2
=====

Lots of repeated unsafe.
Unsound transmutes due to non-#[repr(transparent)] struct around slice.
Unsound test code assumes gen_range meets it's safe API contract.
Not fully reviewed.

Detail
------

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src/serialization/ascii_char.rs   | +1    | thoroughness: low, understanding: high throughout |
| src/serialization/ascii_str.rs    | +1    | |
| src/serialization/ascii_string.rs | +1    | |
| src/serialization/mod.rs          | +1    | |
| src/ascii_char.rs                 | 0     | unsound test code?
| src/ascii_str.rs                  | -1    | UNSOUND - missing #[repr(transparent)]` !
| src/ascii_string.rs               | N/A   | Unreviewed |
| src/free_functions.rs             | N/A   | |
| src/lib.rs                        | N/A   | |
| .gitignore                        | N/A   | |
| .travis.yml                       | N/A   | |
| Cargo.toml                        | +1    | |
| Cargo.toml.orig                   | +1    | |
| LICENSE-APACHE                    | N/A   | |
| LICENSE_MIT                       | N/A   | |
| README.md                         | N/A   | |
| RELEASES.md                       | N/A   | |
| tests.rs                          | N/A   | |



| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1    | UNSOUND, disappointing lack of debug_assert!s.
| fs        |
| io        |
| docs      | +1    |
| tests     | +1    |


### src/ascii_char.rs

| Line  | Notes |
| -----:| ----- |
| 22    | This must contain every value between 0..=127 for soundness guarantees bellow.
| 476   | unsafe { ... } - looks sound.  case 1 handles 32..=126, case 2 handles 127, case 3 handles 0..=31.  Not wild about this impl but looks valid.  See https://en.wikipedia.org/wiki/Control_Pictures
| 498   | unsafe { ... } - looks sound.  'a' > 'A'
| 509   | unsafe { ... } - looks sound.
| 548   | unsafe { ... } - looks sound.  Duplicate logic, annoyingly.
| 557   | unsafe { ... } - looks sound.  Duplicate logic, annoyingly.
| 659   | unsafe fn - looks good.
| 670   | unsafe fn - disappointing lack of debug_assert!
| 678   | unsafe { ... } - looks sound.
| 686   | unsafe fn - disappointing lack of debug_assert!.  transmute from u8 to #[repr(u8)] enum... I believe that's sound.
| 694   | unsafe { ... } - looks sound.
| 702   | unsafe fn - looks sound.
| 714   | UNSOUND TEST CODE, no guarantee generic Gen actually generates within range.  Gated behind "quickcheck" feature.
| 735   | UNSOUND TEST CODE, no guarantee generic Gen actually generates within range.  Gated behind "quickcheck" feature.

### src/ascii_str.rs

| Line  | Notes |
| -----:| ----- |
| 116   | unsafe fn - looks good.
| 352   | UNSOUND unsafe { ... } - AsciiStr is not `#[repr(transparent)]` !
| 359   | UNSOUND unsafe { ... } - AsciiStr is not `#[repr(transparent)]` !
| 367   | UNSOUND unsafe { ... } - AsciiStr is not `#[repr(transparent)]` !
| 384   | scary transmuting impl_into! macro, audit all uses carefully
| 390   | UNSOUND unsafe { ... } - AsciiStr is not `#[repr(transparent)]` !
| 397   | UNSOUND unsafe { ... } - AsciiStr is not `#[repr(transparent)]` !
| 405   | UNSOUND unsafe { ... } - AsciiStr is not `#[repr(transparent)]` !
| 410   | I believe these invokes would be sound if AsciiStr was `#[repr(transparent)]`
| 668   | unsafe fn - looks good.
| 676   | unsafe fn - looks good.
| 689   | unsafe fn - looks good.
| 701   | unsafe fn - looks good.
| 713   | unsafe fn - looks good.
| 724   | unsafe fn - looks good.
| 734   | unsafe fn - looks good.
| 746   | unsafe fn - looks good.
| 756   | unsafe fn - looks good.
| 764   | unsafe { ... } - looks sound.
| 768   | unsafe fn - disappointing lack of debug_assert!.
| 777   | unsafe { ... } - looks sound.
| 781   | unsafe fn - disappointing lack of debug_assert!.
| 793   | unsafe fn - looks good.
| 800   | unsafe { ... } - looks sound.
| 804   | unsafe fn - disappointing lack of debug_assert!.
| 818   | unsafe fn - looks good.
