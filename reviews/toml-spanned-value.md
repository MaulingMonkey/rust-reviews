---
category:       Serialization
description:    File line/col span for .toml values
---

# toml-spanned-value

File line/col span for .toml values

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.0] | low | medium | positive | Full Review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative medium positive strong
-->

[0.1.0]: #0.1.0

<h2 name="0.1.0">0.1.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .github\workflows\toml-spanned-value.yml | ✔️ | actions-rs
| src\lib.rs                        | ✔️ |
| src\map.rs                        | ✔️ | A lot of annoying boilerplate
| src\spanned_value.rs              | ✔️ |
| tests\datetime.rs                 | ✔️ |
| tests\spanned.rs                  | ✔️ |
| .cargo_vcs_info.json              | ✔️ |
| .cargo-ok                         | ✔️ |
| .gitignore                        | ✔️ |
| Cargo.toml                        | ✔️ | `MIT OR Apache-2.0`, indexmap
| Cargo.toml.orig                   | ✔️ | `MIT OR Apache-2.0`, indexmap
| LICENSE-APACHE                    | ✔️ | `Apache-2.0`
| LICENSE-MIT                       | ✔️ | `MIT`
| README.md                         | ✔️ |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | None
| io        | ✔️ | safe serde
| docs      | ✔️ | Excellent
| tests     | ✔️ | Looks good



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
