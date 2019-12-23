---
category:       Gamedev
description:    Hot reloading resources. Not browser friendly.
---

# warmy

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.13.0](#0.13.0) | low | low | positive
| [0.12.0](#0.12.0) | low | low | positive

# 0.13.0

RON support, lockfile/CI changes.  LGTM.

# 0.12.0

Looks good to me.  Some of the finer points are a little obtuse to me (RE: reloads, dependencies, an the inspect trait.)

Detail
------

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| examples/toml/hello.html                      | +1    | |
| examples/toml/main.rs                         | +1    | |
| src/context.rs                                | +1    | |
| src/json.rs                                   | +1    | |
| [src/key.rs](src/key.rs)                      | +1    | |
| src/lib.rs                                    | +1    | |
| src/load.rs                                   | +1    | |
| src/res.rs                                    | +1    | |
| src/toml.rs                                   | +1    | |
| tests/lib.rs                                  | +1    | |
| .cargo_vcs_info.json                          | +1    | |
| .cargo-ok                                     | +1    | |
| .gitignore                                    | +1    | |
| .travis.yml                                   | +1    | |
| Cargo.toml                                    | +1    | |
| Cargo.toml.orig                               | +1    | |
| CHANGELOG.md                                  | +1    | |
| LICENSE                                       | +1    | |
| README.md                                     | +1    | |
| rustfmt.toml                                  | +1    | Ew 2 space indents gross

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | No unsafe code
| fs        | +1    | Nothing fishy
| io        | +1    | Nothing fishy
| docs      | +1    | Good god there are a lot.  Needs more concrete motivating examples though.
| tests     | +1    | |

src/key.rs
----------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 12    | Key   | `'static` lifetime... minor leak? but I probably don't care?

TIL
---
cargo-sync-readme
cargo-outdated
