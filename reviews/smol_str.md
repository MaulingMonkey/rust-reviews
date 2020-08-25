---
category:       Data Structure
description:    Immutable small string premature optimizations
---

# smol_str

Immutable small string premature optimizations

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.16] | medium | medium | neutral | Diff: simplification
| [0.1.15] | medium | medium | neutral | Diff: features
| [0.1.14] | medium | medium | neutral | Diff: features
| [0.1.13] | medium | medium | neutral | Diff: 2018 edition
| [0.1.12] | medium | medium | neutral | Diff: `FromIterator`
| [0.1.11] | medium | medium | neutral | Diff: `Ord`, `PartialOrd`, `FromIterator<char>`
| [0.1.10] | medium | medium | neutral | Diff: `Borrow<str>`
| [0.1.9] | medium | medium | neutral | Diff: `Default`
| [0.1.8] | medium | medium | neutral | Diff: `From<SmolStr>`
| [0.1.7] | medium | medium | neutral | Diff: `is_empty`
| [0.1.6] | medium | medium | neutral | Diff: `len`
| [0.1.5] | medium | medium | neutral | Diff: serde
| [0.1.4] | medium | medium | neutral | Diff: `Eq`
| [0.1.3] | medium | medium | neutral | Diff: `Hash`
| [0.1.2] | medium | medium | neutral | Diff: `PartialEq`, generic `new`/`From`
| [0.1.1] | medium | medium | neutral | Diff: `From<&str>`
| [0.1.0] | medium | medium | neutral | Full review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative medium positive strong
-->

[0.1.16]: #0.1.16
[0.1.15]: #0.1.15
[0.1.14]: #0.1.14
[0.1.13]: #0.1.13
[0.1.12]: #0.1.12
[0.1.11]: #0.1.11
[0.1.10]: #0.1.10
[0.1.9]: #0.1.9
[0.1.8]: #0.1.8
[0.1.7]: #0.1.7
[0.1.6]: #0.1.6
[0.1.5]: #0.1.5
[0.1.4]: #0.1.4
[0.1.3]: #0.1.3
[0.1.2]: #0.1.2
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.1.16">0.1.16</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | simplified bounds, tweaks

Also re-reviewed Repr::Inline construction sites, they all still seem legit.
That said, they're getting much less local to review, making that one `unsafe { ... }` block annoying.

<h2 name="0.1.15">0.1.15</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | more serde feature fixes
| Cargo.toml.orig                   | ✔️ | more serde feature fixes

<h2 name="0.1.14">0.1.14</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | serde feature fixes
| Cargo.toml.orig                   | ✔️ | serde feature fixes

<h2 name="0.1.13">0.1.13</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | 2018 edition
| Cargo.toml.orig                   | ✔️ | 2018 edition, serde dev-dep
| src\lib.rs                        | ✔️ | Docs, doc tests, serde overhaul
| tests\test.rs                     | ✔️ | greatly expanded serde tests

<h2 name="0.1.12">0.1.12</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | bench, criterion, serde optimization
| Cargo.toml.orig                   | ✔️ | bench, criterion, serde optimization
| benches\building.rs               | ✔️ | criterion benchmarks
| bors.toml                         | ✔️ |
| src\lib.rs                        | ✔️ | re-reformat, +`new_inline_from_ascii`, +`FromIterator<...> for SmolStr`
| tests\test.rs                     | ✔️ | +`const_fn_ctor`, +`from_string_iter`, `from_str_iter`

<h2 name="0.1.11">0.1.11</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | +`,`
| src\lib.rs                        | ✔️ | +`Ord`, +`PartialOrd`, +`FromIterator<char>`, reformat
| tests\test.rs                     | ✔️ | +`test_from_iterator`

<h2 name="0.1.10">0.1.10</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | +`is_heap_allocated`, +`Borrow<str> for SmolStr`, +`#[inline]`
| tests\test.rs                     | ✔️ | +`test_search_in_hashmap`

<h2 name="0.1.9">0.1.9</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | +`Default for SmolStr`

<h2 name="0.1.8">0.1.8</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo_vcs_info.json              | ✔️ | Added
| src\lib.rs                        | ✔️ | + `From<SmolStr> for String`
| tests\test.rs                     | ✔️ | coverage

<h2 name="0.1.7">0.1.7</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | typo fixes
| Cargo.toml.orig                   | ✔️ | typo fixes
| README.md                         | ✔️ | reformatting, typo fix
| src\lib.rs                        | ✔️ | reformatting, +`is_empty`, eliminated silly inline magic nonsense
| tests\test.rs                     | ✔️ | +`is_empty` coverage, !serde test fixes


<h2 name="0.1.6">0.1.6</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | + `len` methods
| tests\test.rs                     | ✔️ | coverage

<h2 name="0.1.5">0.1.5</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .travis.yml                       | ✔️ | + `cargo test --all-features`
| Cargo.toml                        | ✔️ | + `serde`
| Cargo.toml.orig                   | ✔️ | + `serde`
| src\lib.rs                        | ✔️ | + `serde` support
| tests\test.rs                     | ✔️ | + `test_serde`

<h2 name="0.1.4">0.1.4</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | + `Eq`

<h2 name="0.1.3">0.1.3</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .travis.yml                       | ✔️ | remove `cargo run --example serde`
| src\lib.rs                        | ✔️ | + `Hash`

<h2 name="0.1.2">0.1.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | + `PartialEq`, generic `new`/`From`

<h2 name="0.1.1">0.1.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | + `From<&str>`

<h2 name="0.1.0">0.1.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| .gitignore                        | ✔️
| .travis.yml                       | ✔️
| Cargo.toml                        | ✔️ | `MIT OR Apache-2.0`, only dev deps
| Cargo.toml.orig                   | ✔️ | `MIT OR Apache-2.0`, only dev deps
| LICENSE-APACHE                    | ✔️ | `Apache 2.0`
| LICENSE-MIT                       | ✔️ | `MIT`
| README.md                         | ✔️
| src\lib.rs                        | ✔️
| tests\test.rs                     | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | A tiny bit
| fs        | ✔️ | None
| io        | ✔️ | None
| docs      | ⚠️ | Limited method docs, but straightforward
| tests     | ❔  | `proptest!` magic only


<h2 name="0.1.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 151   | ✔️ `unsafe { from_utf8_unchecked(buf) }` on normal inline repr (e.g. not `len == WS_TAG` special case) should be safe, only constructable via Repr::new


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
