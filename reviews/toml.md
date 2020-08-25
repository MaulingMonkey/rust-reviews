---
category:       Serialization
description:    serde .toml deserialization
---

# toml

serde .toml deserialization

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.5.6] | low | low | positive | Full Review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative medium positive strong
-->

[0.5.6]: #0.5.6

<h2 name="0.5.6">0.5.6</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .github\workflows\main.yml        | ✔️ | gh-pages push
| examples\decode.rs                | ✔️
| examples\enum_external.rs         | ✔️
| examples\toml2json.rs             | ✔️
| src\datetime.rs                   | ✔️
| src\de.rs                         | ✔️
| src\lib.rs                        | ✔️ | `#![forbid(unsafe_code)]`
| src\macros.rs                     | ✔️
| src\map.rs                        | ✔️
| src\ser.rs                        | ✔️
| src\spanned.rs                    | ✔️
| src\tokens.rs                     | ✔️
| src\value.rs                      | ✔️
| tests\enum_external_deserialize.rs | ✔️
| .cargo_vcs_info.json              | ✔️
| .cargo-ok                         | ✔️
| .gitignore                        | ✔️
| Cargo.lock                        | ✔️
| Cargo.toml                        | ✔️ | `MIT/Apache-2.0`
| Cargo.toml.orig                   | ✔️ | `MIT/Apache-2.0`
| LICENSE-APACHE                    | ✔️ | `Apache-2.0`
| LICENSE-MIT                       | ✔️ | `MIT`
| README.md                         | ✔️ | `MIT OR Apache-2.0`

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | Forbidden!
| fs        | ✔️ | examples only
| io        | ✔️ | examples, errors
| docs      | ✔️ | Excellent, `deny(missing_docs)`
| tests     | ✔️ | Good

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
