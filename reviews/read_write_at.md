---
category:       I/O
description:    Offset read_at/write_at with &self
---

# read_write_at

Offset read_at/write_at with `&self`

Pros:
* No runtime dependencies
* Very thorough, well documented

Cons:
* `File` isn't `ReadAt`/`WriteAt` on windows due to seeks... reasonable but annoying
* A couple of minor API holes to be fixed

References:
[github](https://github.com/vi/read_write_at),
[docs.rs](https://docs.rs/read_write_at/),
[lib.rs](https://lib.rs/crates/read_write_at),
[crates.io](https://crates.io/crates/read_write_at)



## Alternatives

| ❔  | crate                   | ver   | rationale |
|-----| ----------------------- | ----- | --------- |
| ✔  | [buffered_offset_reader] | 0.6.0 | Less generalized.  Decent, slightly more convenient w/ `File` implementing read_at/write_at by default on windows too.
| ❌ | [io-at]                  | 0.4.1 | [write_at](https://docs.rs/io-at/0.4.1/io_at/trait.WriteAt.html#tymethod.write_at) requires `&mut self`
| ❌ | [positioned-io]          | 0.2.2 | [write_at](https://docs.rs/positioned-io/0.2.2/positioned_io/trait.WriteAt.html#tymethod.write_at) requires `&mut self`
| ❌ | [positioned-io-preview]  | 0.3.3 | [write_at](https://docs.rs/positioned-io-preview/0.3.3/positioned_io_preview/trait.WriteAt.html#tymethod.write_at) requires `&mut self`
| ❌ | [scroll]                 | 0.10.1 | Assumes you have a fully loaded byte buffer

[buffered_offset_reader]:   https://lib.rs/crates/buffered_offset_reader
[read_write_at]:            https://lib.rs/crates/read_write_at
[io-at]:                    https://lib.rs/crates/io-at
[positioned-io]:            https://lib.rs/crates/positioned-io
[positioned-io-preview]:    https://lib.rs/crates/positioned-io-preview
[scroll]:                   https://lib.rs/crates/scroll



## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.0] | high | high | ✔️ positive | Full review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         ❌ dangerous ⚠️❗️ negative ❔ neutral ✔️ positive ✔️ strong
-->

[0.1.0]: #0.1.0

<h2 name="0.1.0">0.1.0</h2>

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| <span>.</span>gitignore                                   | ✔️
| <span>.</span>travis<span>.</span>yml                     | ✔️ | MSRV 1.34.2
| Cargo<span>.</span>toml                                   | ⚠️ | `MIT/Apache-2.0`
| Cargo<span>.</span>toml<span>.</span>orig                 | ⚠️ | `MIT/Apache-2.0`
| LICENSE                                                   | ⚠️ | `MIT`
| README<span>.</span>md                                    | ⚠️ | `MIT/Apache-2.0`
| src\lib<span>.</span>rs                                   | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | `#![forbid(unsafe_code)]`
| fs        | ✔️ | Reasonable
| io        | ✔️ | Reasonable
| docs      | ✔️ | `#![deny(missing_docs)]`, reasonable
| tests     | ✔️ | Relatively minimal but OK

<h2 name="0.1.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 41    | ✔️ `read_exact_at` impl logic
| 71    | ✔️ `read_exact_at` impl logic
| 92    | ✔️ `ReadAtMut` for `ReadAt`
| 110   | ✔️ `write_all_at` impl logic
| 146   | ✔️ `write_all_at` impl logic
| 168   | ✔️ `WriteAtMut` for `WriteAt`
| 185   | ✔️ \*nix `WriteAt` for `File`
| 196   | ✔️ \*nix `ReadAt` for `File`
| 205   | ⚠️ windows `WriteAtMut` for `File` - I'd like `WriteAt` too though
| 215   | ⚠️ windows `ReadAtMut` for `File` - I'd like `WriteAt` too though
| 238   | ✔️ `ReadAtMut` for `ReadWriteSeek`
| 261   | ✔️ `WriteAtMut` for `ReadWriteSeek`
| 305   | ✔️ `ReadAtMut` for `DerefWrapper`
| 316   | ✔️ `WriteAtMut` for `DerefWrapper`
| 329   | ✔️ `ReadAt` for `RefCell`
| 342   | ✔️ `WriteAt` for `RefCell`
| 357   | ✔️ `ReadAt` for `Mutex<ReadAtMut>`
| 384   | ✔️ `WriteAt` for `Mutex<WriteAtMut>`
| 415   | ✔️ Skimmed reasonable looking tests


<!-- Templates

✔️❔⚠️❗️❌

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
