---
category:       I/O
description:    Offset read/write with `&self`
---

# buffered_offset_reader

Offset read/write with `&self`

Pros:
* No runtime dependencies
* `write_at(&self, ...)`

Cons:
* `BufOffsetReader` is a bit kludgy - `u64 as usize` casts should probably `Err` instead
* Implementing traits directly on `File` makes for inconsistent seek behavior when used by reference

References:
[github](https://github.com/sbillig/buffered_offset_reader),
[docs.rs](https://docs.rs/buffered_offset_reader/),
[lib.rs](https://lib.rs/crates/buffered_offset_reader),
[crates.io](https://crates.io/crates/buffered_offset_reader)



## Alternatives

| ❔  | crate                   | ver   | rationale |
|-----| ----------------------- | ----- | --------- |
| ✔️ | [read_write_at]          | 0.1.0 | More generalized, "saner", although `File` not implementing `ReadAt` / `WriteAt` by default on windows is annoying.
| ❌ | [io-at]                  | 0.4.1 | [write_at](https://docs.rs/io-at/0.4.1/io_at/trait.WriteAt.html#tymethod.write_at) requires `&mut self`
| ❌ | [positioned-io]          | 0.2.2 | [write_at](https://docs.rs/positioned-io/0.2.2/positioned_io/trait.WriteAt.html#tymethod.write_at) requires `&mut self`
| ❌ | [positioned-io-preview]  | 0.3.3 | [write_at](https://docs.rs/positioned-io-preview/0.3.3/positioned_io_preview/trait.WriteAt.html#tymethod.write_at) requires `&mut self`
| ❌ | [scroll]                 | 0.10.1 | Assumes you have a fully loaded byte buffer

[buffered_offset_reader]:   https://lib.rs/crates/io-at
[read_write_at]:            https://lib.rs/crates/read_write_at
[io-at]:                    https://lib.rs/crates/io-at
[positioned-io]:            https://lib.rs/crates/positioned-io
[positioned-io-preview]:    https://lib.rs/crates/positioned-io-preview
[scroll]:                   https://lib.rs/crates/scroll



## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.6.0] | high | high | ✔️ positive | size -> `usize`
| [0.5.0] | high | high | ✔️ positive | offset + size -> `u64`
| [0.4.0] | high | high | ✔️ positive | `BufOffsetReader` owns R
| [0.3.0] | high | high | ✔️ positive | `OffsetRead` for `&[u8]`
| [0.2.0] | high | high | ✔️ positive | +`OffsetWrite`
| [0.1.1] | high | high | ✔️ positive | 
| [0.1.0] | high | high | ✔️ positive | +`OffsetRead`

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         ❌ dangerous ⚠️❗️ negative ❔ neutral ✔️ positive ✔️ strong
-->

[0.6.0]: #0.6.0
[0.5.0]: #0.5.0
[0.4.0]: #0.4.0
[0.3.0]: #0.3.0
[0.2.0]: #0.2.0
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.6.0">0.6.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | size `u64` -> `usize`

<h2 name="0.5.0">0.5.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| benches\bench<span>.</span>rs                             | ✔️ | offset + size `usize` -> `u64`
| src\lib<span>.</span>rs                                   | ✔️ | offset + size `usize` -> `u64`

<h2 name="0.4.0">0.4.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| README<span>.</span>md                                    | ✔️ | Trivial
| benches\bench<span>.</span>rs                             | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | `BufOffsetReader` now owns `R`

TIL:
* [File::try_clone](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_clone) exists

<h2 name="0.3.0">0.3.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | `OffsetRead` for `&[u8]`, tests

<h2 name="0.2.0">0.2.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| README<span>.</span>md                                    | ✔️ | Whitespace
| src\lib<span>.</span>rs                                   | ✔️ | `clear` for BufOffsetReader, `OffsetWrite`

<h2 name="0.1.1">0.1.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | docs, readme, travis badge, exclude .travis.yml
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | docs, readme, travis badge, exclude .travis.yml
| README<span>.</span>md                                    | ✔️ | Fleshed out much more, badge links
| src\lib<span>.</span>rs                                   | ✔️ | Saner parens

<h2 name="0.1.0">0.1.0</h2>

Full Review
* Initial OffsetRead only

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| <span>.</span>gitignore                                   | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | `MIT OR Apache-2.0`
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | `MIT OR Apache-2.0`
| LICENSE-APACHE                                            | ✔️ | `Apache-2.0`
| LICENSE-MIT                                               | ✔️ | `MIT`
| README<span>.</span>md                                    | ✔️ | Minimal
| benches\bench<span>.</span>rs                             | ⚠️ | `make_temp_file` should use `write_all` ?
| src\lib<span>.</span>rs                                   | ✔️
| src\range<span>.</span>rs                                 | ✔️ | Could use better docs

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | No path manipulation, just safe std stuff
| io        | ✔️ | Safe/sane
| docs      | ✔️
| tests     | ✔️❔⚠️❗❌

<h2 name="0.1.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 78    | ⚠️ Could `self.buffer` be partially read with a stale `self.range`?
| 83    | Assumes `self.contains(r)`
| 92    | ✔️ impl OffsetReadMut for BufOffsetReader - awkwardly coded but correct
| 110   | ✔️ impl OffsetRead for File
| 127   | Skimmed tests

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
