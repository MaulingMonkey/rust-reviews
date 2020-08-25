---
category:       General Utility
description:    Parse the crates.io index
---

# crates-index

Parse the crates.io index

Pros:
* Works!

Cons:
* Doesn't use the same binary cached data that `cargo` does
* Less than obvious how to handle updates in a less abusive way

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.15.2]  | medium       | medium        | positive | full review

[0.15.2]: #0.15.2

<h2 name="0.15.2">0.15.2</h2>

| File                  | Rating | Notes |
| --------------------- | ------ | ----- |
| src/lib.rs            | ✔️
| .cargo_vcs_info.json  | ✔️
| .cargo-ok             | ✔️
| .gitignore            | ✔️
| .travis.yml           | ✔️
| Cargo.lock            | ❔ | Should review smol_str, hex, home
| Cargo.toml            | ✔️ | Apache-2.0
| Cargo.toml.orig       | ✔️ | Apache-2.0
| LICENSE               | ✔️ | Apache-2.0
| README.md             | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️    | None
| fs        | ✔️    | User provided paths + glob
| io        | ✔️
| docs      | ✔️
| tests     | ✔️

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
