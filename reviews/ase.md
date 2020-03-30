---
category:       Gamedev
description:    Asesprite Format Reader
crev:           negative
---

# ase

A crate for reading [Aseprite](https://www.aseprite.org/) files.

~~Decent basis with room for improvement.~~

**Undertested, published crate unusably buggy.  Git master branch might be usable / a decent basis?:**

```toml
[dependencies]
ase = { git = "https://github.com/matheuslessarodrigues/ase-rs", branch = "753c7c866bb6438a6c04c8eecf328942bc0533a0" }
```

## Issues

| issue | severity  | desc |
| ----- | --------- | ---- |
| [#4]  | low       | Possible overflow/panic for ~65k pixel masks (256x256)
| N/A   | low       | Possible memory exhaustion via unlimited allocs

[#4]:   https://github.com/matheuslessarodrigues/ase-rs/issues/4

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [753c7c8](https://github.com/matheuslessarodrigues/ase-rs/commit/753c7c866bb6438a6c04c8eecf328942bc0533a0) | low | medium | neutral | Skimmed full src, lots of bugfixes. |
|           |        |        |          | **Unusably buggy bellow this point** |
| [0.1.3]   | medium | medium | negative | Diff review.  Some test coverage, bugfixes, minor refactoring. |
| [0.1.1]   | medium | medium | negative | Diff review.  Typo fixes. |
| [0.1.0]   | medium | medium | negative | Full review.  Not suitable for UGC - memory exhaustion and panic concerns |

[0.1.3]: #0.1.3
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.1.3">0.1.3</h2>

Diff review.
Various renaming, cel/path/slice chunk bugfixes.
Some basic tests.

<h2 name="0.1.1">0.1.1</h2>

Diff review.
Minor typo fixes, no behavior changes.

<h2 name="0.1.0">0.1.0</h2>

Full review.
Not suitable for UGC - memory exhaustion and panic concerns.

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src/chunk/cel_chunk.rs            | ✔️ | |
| src/chunk/cel_extra_chunk.rs      | ✔️ | |
| src/chunk/color_profile_chunk.rs  | ✔️ | |
| src/chunk/frame_tags_chunk.rs     | ✔️ | |
| src/chunk/layer_chunk.rs          | ✔️ | |
| src/chunk/mask_chunk.rs           | ⚠️ | 27: bitmap_data will likely panic on large masks due to overflowing a u16 (>65k pixels?)
| src/chunk/old_palette_chunk4.rs   | ✔️ | |
| src/chunk/old_palette_chunk11.rs  | ✔️ | |
| src/chunk/palette_chunk.rs        | ✔️ | |
| src/chunk/path_chunk.rs           | ✔️ | Discarded data
| src/chunk/slice_chunk.rs          | ✔️ | |
| src/chunk/user_data_chunk.rs      | ✔️ | |
| src/chunk.rs                      | ✔️ | |
| src/color.rs                      | ✔️ | |
| src/frame.rs                      | ✔️ | |
| src/header.rs                     | ✔️ | |
| src/helpers.rs                    | ⚠️ | 15: assumes valid ase files use valid utf8
| src/lib.rs                        | ⚠️ | 56: pointless dead code |
| .cargo_vcs_info.json              | ✔️ | |
| .cargo-ok                         | ✔️ | |
| .gitignore                        | ✔️ | |
| Cargo.toml                        | ✔️ | |
| Cargo.toml.orig                   | ✔️ | MIT
| LICENSE                           | ✔️ | MIT
| README.md                         | ✔️ | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️    | None
| fs        | ✔️    | None
| io        | ❔    | Possible DoS panics via memory exhaustion, large masks, could use fuzzing.  No RCEs.
| docs      | ❗️    | What docs?
| tests     | ❗️    | What tests?  Fuzzing recommended.

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
