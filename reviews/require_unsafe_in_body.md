---
category:       General Utility
description:    Reducing the scope of `unsafe { ... }` in `unsafe fn`s.
---

# require_unsafe_in_body

Reducing the scope of `unsafe { ... }` in `unsafe fn`s.

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.2.0](#0.2.0)               | low | low | positive
| [0.2.0-alpha](#0.2.0-alpha)   | medium | low | positive
| [0.1.2](#0.1.2)               | medium | low | positive

# 0.2.0

More refactoring surrounding generics.

# 0.2.0-alpha

Significant refactoring, adding support for generics.  LGTM?

# 0.1.2

Seems solid, although my syn-fu is weak, limiting my ability to review this.

Detail
------

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| src/docs/require_unsafe_in_bodies.md          | +1 | |
| src/docs/require_unsafe_in_body.md            | +1 | |
| src/utils/macros.rs                           | +1 | |
| src/utils/mod.rs                              | +1 | |
| src/lib.rs                                    | +1 | understanding: low - I'm barely following along
| src/tests.rs                                  | +1 | |
| tests/ui/body_on_method_2.rs                  | +1 | |
| tests/ui/body_on_method_2.stderr              | +1 | |
| tests/ui/body_on_method.rs                    | +1 | |
| tests/ui/body_on_method.stderr                | +1 | |
| tests/ui/readme.rs                            | +1 | |
| tests/ui/readme.stderr                        | +1 | |
| tests/impl_method_2.rs                        | +1 | The cfgs on unit-tests look the wrong way around?
| tests/impl_method.rs                          | +1 | The cfgs on unit-tests look the wrong way around?
| tests/trait_default_method.rs                 | +1 | The cfgs on unit-tests look the wrong way around?
| tests/ui.rs                                   | +1 | The cfgs on unit-tests look the wrong way around?
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| Cargo.toml                                    | +1 | MIT
| Cargo.toml.orig                               | +1 | MIT
| LICENSE                                       | +1 | MIT
| Makefile                                      | +1 | Unixy
| README.md                                     | +1 | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | Wrangles unsafe but doesn't actually use it itself
| fs        | +1    | None
| io        | +1    | Modifies codegen through syn
| docs      | +1    | |
| tests     | +1    | |

TIL
---

```rust
let Struct { ref member, ref mut member2, .. } = to_destructure;
```
