---
category:       General Utility
description:    impl crate for num_enum
msrv:           stable
---

# num_enum_derive

impl crate for num_enum

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.4.3]   | low | medium | positive | Diff Review - Empty version bump
| [0.4.2]   | low | medium | positive | Diff Review - Minor fixups
| [0.4.1]   | low | medium | positive | Diff Review - Empty version bump
| [0.4.0]   | low | medium | positive | Diff Review - Extracted from num_enum
| [0.0.0]   | high | high | strong | Full Review - Empty Placeholder

[0.4.3]: #0.4.3
[0.4.2]: #0.4.2
[0.4.1]: #0.4.1
[0.4.0]: #0.4.0
[0.0.0]: #0.0.0

<h2 name="0.4.3">0.4.3</h2>

Diff Review - Empty version bump



<h2 name="0.4.1">0.4.2</h2>

Diff Review - Minor fixups

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| .cargo_vcs_info.json  | ✔️ | |
| Cargo.toml            | ✔️ | Dependency bumps
| Cargo.toml.orig       | ✔️ | Dependency bumps
| src/lib.rs            | ✔️ | Minor repr/parse refactoring/fixes



<h2 name="0.4.1">0.4.1</h2>

Diff Review - Empty version bump



<h2 name="0.4.0">0.4.0</h2>

Diff Review - Extracted from num_enum

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| .cargo_vcs_info.json  | ✔️ | |
| .gitignore            | ✔️ | Removed
| Cargo.toml            | ✔️ | proc-macro-crate ?
| Cargo.toml.orig       | ✔️ | |
| LICENSE               | ✔️ | Looks like BSD-3-Clause
| src/lib.rs            | ✔️ | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ⚠️ | derive UnsafeFromPrimitive adds unsafe fn, but reasonable / without soundness holes
| fs        | ✔️ | None
| io        | ✔️ | proc_macro codegen only
| docs      | ✔️ | Improved use of doc comments over 0.3.x num_enum
| tests     | ❗️ | Elsewhere?



<h2 name="0.0.0">0.0.0</h2>

Full Review - Empty Placeholder

| File                  | Rating | Notes |
| --------------------- | ------ | ----- |
| src/lib.rs            | ✔️ | Empty placeholder
| .cargo_vcs_info.json  | ✔️ | |
| .cargo-ok             | ✔️ | |
| .gitignore            | ✔️ | |
| Cargo.toml            | ✔️ | BSD-3-Clause
| Cargo.toml.orig       | ✔️ | BSD-3-Clause

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
