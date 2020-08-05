---
category:       General Utility
description:    Compile time UTF16 strings for windows `wchar_t *` interop.
---

# wchar

Compile time UTF16 strings for windows `wchar_t *` interop.

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.6.0]   | medium | medium | positive | `#![no_std]`, `Cargo.lock` files
| [0.5.0]   | medium | medium | positive | `proc_macro_hack`, null checks, more thorough testing
| [0.2.0]   | medium | medium | positive | +`wch_c!` (null terminated)
| [0.1.1]   | medium | medium | positive | Fixed doc test, improved readme
| [0.1.0]   | medium | medium | positive | |

[0.6.0]: #0.6.0
[0.5.0]: #0.5.0
[0.2.0]: #0.2.0
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.6.0">0.6.0 (wchar + wchar-impl)</h2>

* `#![no_std]`
* `Cargo.lock` files

<h2 name="0.5.0">0.5.0 (wchar + wchar-impl)</h2>

* `proc_macro_hack` + floating `wchar_impl`
* interior nul error checking
* improved test cases
* CI
* Edition 2018
* `trybuild` (dev-dependencies)
* `quote` (dependencies, via wchar-impl)

<h2 name="0.2.0">0.2.0 (wchar)</h2>

* Added `wch_c!` (null terminated)

<h2 name="0.1.1">0.1.1 (wchar)</h2>

* Fixed doc test
* Improved readme

<h2 name="0.1.0">0.1.0 (wchar)</h2>

| File                  | Rating | Notes |
| --------------------- | ------ | ----- |
| src/lib.rs            | ✔️ | |
| Cargo.toml            | ✔️ | proc-macro2, syn - overkill but OK.  MIT.
| Cargo.toml.orig       | ✔️ | proc-macro2, syn - overkill but OK.  MIT.
| LICENSE               | ✔️ | MIT
| README.md             | ✔️ | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | none
| fs        | ✔️ | none
| io        | ✔️ | only token streams
| docs      | ✔️ | excellent
| tests     | ✔️ | a few doc strings


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
