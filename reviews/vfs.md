---
category:       I/O
description:    Filesystem virtualization
msrv:           1.32.0
---

# vfs

Basic VFS support!

Decent crate seeing some usage.  Worth building upon.

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.4.0] | low | medium | positive | ⚠️ MIT license dropped
| [0.3.0] | low | medium | positive | MSRV bump
| [0.2.1] | low | medium | positive
| [0.2.0] | low | medium | positive
| [0.1.2] | low | medium | positive
| [0.1.1] | low | medium | positive | ✔️ `Apache-2.0/MIT`
| [0.1.0] | low | medium | negative | ❗️ No license

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[0.4.0]: #0.4.0
[0.3.0]: #0.3.0
[0.2.1]: #0.2.1
[0.2.0]: #0.2.0
[0.1.2]: #0.1.2
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.4.0">0.4.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .githooks\pre-commit              | ⚠️ | eww, `git add --update` ruins `git add -p`
| .github\workflows\compile.yml     | ✔️
| .travis.yml                       | ✔️ | MSRV `1.45.0` -> `1.32.0`
| Cargo.toml                        | ⚠️ | -`MIT`, only `Apache-2.0` now!
| Cargo.toml.orig                   | ⚠️ | -`MIT`, only `Apache-2.0` now!
| LICENSE                           | ✔️ | `Apache-2.0`
| README.md                         | ✔️ | MSRV 1.32.0
| src\error.rs                      | ✔️
| src\filesystem.rs                 | ✔️
| src\impls\altroot.rs              | ❔ | No move_file / move_dir support?
| src\impls\memory.rs               | ✔️
| src\impls\overlay.rs              | ✔️
| src\impls\physical.rs             | ✔️
| src\lib.rs                        | ✔️
| src\path.rs                       | ✔️
| src\test_macros.rs                | ✔️

<h2 name="0.3.0">0.3.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .gitignore                        | ✔️
| .travis.yml                       | ✔️ | MSRV `1.8.0` -> `1.45.0`
| Cargo.toml                        | ✔️
| Cargo.toml.orig                   | ✔️ | Edition 2018, +`thiserror`, +`uuid`(dev)
| README.md                         | ✔️ | + Changelog
| src\error.rs                      | ✔️
| src\filesystem.rs                 | ✔️ | `trait VFS` -> `trait FileSystem`, `VPath` -> `VfsPath` ?
| src\impls\altroot.rs              | ⚠️ | This doesn't protect against r"C:\a\b\c".join(r"C:\d\e\f") on windows escaping the alt root
| src\impls\memory.rs               | ✔️
| src\impls\mod.rs                  | ✔️
| src\impls\physical.rs             | ✔️
| src\lib.rs                        | ✔️
| src\path.rs                       | ✔️
| src\test_macros.rs                | ✔️
| src\util.rs                       | ✔️


<h2 name="0.2.1">0.2.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo_vcs_info.json              | ✔️
| Cargo.toml                        | ✔️
| Cargo.toml.orig                   | ✔️ | New
| src\altroot.rs                    | ✔️ | Not a safety mechanism, but appropriately noted in the docs
| src\lib.rs                        | ✔️ | pub fields on `OpenOptions`

<h2 name="0.2.0">0.2.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | + `rm`, `rmrf`
| src\memory.rs                     | ✔️ | NOTE: Recursive `rm`, should maybe not remove if it's a directory and non-empty?
| src\physical.rs                   | ✔️

<h2 name="0.1.2">0.1.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | + `to_path_buf`
| src\memory.rs                     | ✔️
| src\physical.rs                   | ✔️

<h2 name="0.1.1">0.1.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | `Apache-2.0/MIT`
| src\lib.rs                        | ✔️ | Merged `vfs.rs` into here, tweaked traits, +`OpenOptions`
| src\memory.rs                     | ✔️
| src\physical.rs                   | ✔️
| src\util.rs                       | ✔️

<h2 name="0.1.0">0.1.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| .editorconfig                     | ✔️
| .gitignore                        | ✔️
| .travis.yml                       | ✔️ | MSRV `1.8.0`
| Cargo.toml                        | ❗️ | No license specified
| README.md                         | ❗️ | No license specified
| src\lib.rs                        | ✔️
| src\memory.rs                     | ✔️
| src\physical.rs                   | ✔️
| src\vfs.rs                        | ✔️
| wercker.yml                       | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | Reasonable passthrough paths
| io        | ✔️ | Reasonable passthrough
| docs      | ✔️
| tests     | ✔️

<!-- Templates

✔️❔⚠️❗️

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
