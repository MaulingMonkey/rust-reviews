---
category:       I/O
description:    Core structures for codespan-reporting
---

# codespan

Core structures for codespan-reporting

Pros:
* Beautifully documented codebase

Neutral:
* Large diffs, constant internal refactoring makes for constant improvements and constant rereviews

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.9.5] | ⚠️ low | medium | ✔️ positive
| [0.9.4] | ⚠️ low | medium | ✔️ positive
| [0.9.3] | ⚠️ low | medium | ✔️ positive
| [0.9.2] | ⚠️ low | medium | ✔️ positive
| [0.9.1] | ⚠️ low | medium | ✔️ positive
| [0.9.0] | ⚠️ low | medium | ✔️ positive
| [0.8.0] | ⚠️ low | medium | ✔️ positive
| [0.7.0] | ⚠️ low | medium | ✔️ positive
| [0.6.0] | ⚠️ low | medium | ✔️ positive
| [0.5.0] | ⚠️ low | medium | ✔️ positive
| [0.4.1] | ⚠️ low | medium | ✔️ positive
| [0.4.0] | ⚠️ low | medium | ✔️ positive | Major overhaul
| [0.3.0] | medium | medium | ❗️ negative | Makes assumptions about `PathBuf` allocs for `heapsize`
| [0.2.1] | medium | medium | ❗️ negative | Makes assumptions about `PathBuf` allocs for `heapsize`
| [0.2.0] | medium | medium | ❗️ negative | Makes assumptions about `PathBuf` allocs for `heapsize`
| [0.1.3] | medium | medium | ❗️ negative | Makes assumptions about `PathBuf` allocs for `heapsize`
| [0.1.2] | medium | medium | ✔️ positive
| [0.1.1] | medium | medium | ✔️ positive
| [0.1.0] | medium | medium | ✔️ positive | Full review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative medium positive strong
-->

[0.9.5]: #0.9.5
[0.9.4]: #0.9.4
[0.9.3]: #0.9.3
[0.9.2]: #0.9.2
[0.9.1]: #0.9.1
[0.9.0]: #0.9.0
[0.8.0]: #0.8.0
[0.7.0]: #0.7.0
[0.6.0]: #0.6.0
[0.5.0]: #0.5.0
[0.4.1]: #0.4.1
[0.4.0]: #0.4.0
[0.3.0]: #0.3.0
[0.2.1]: #0.2.1
[0.2.0]: #0.2.0
[0.1.3]: #0.1.3
[0.1.2]: #0.1.2
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.9.5">0.9.5</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| src\file.rs                       | ✔️

<h2 name="0.9.4">0.9.4</h2>
<h2 name="0.9.3">0.9.3</h2>
<h2 name="0.9.2">0.9.2</h2>

Trivial version bumps, no real changes

<h2 name="0.9.1">0.9.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| src\index.rs                      | ✔️ | `From` impls
| src\span.rs                       | ✔️ | `From` impls

<h2 name="0.9.0">0.9.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️ | Huzzah!
| Cargo.toml                        | ✔️ | `README.md` fix, +`codespan-reporting`, -`unicode-segmentation`, -`pretty_assertions` (dev)
| Cargo.toml.orig                   | ✔️ | `README.md` fix, +`codespan-reporting`, -`unicode-segmentation`, -`pretty_assertions` (dev)
| README.md                         | ✔️ | `README.md` fix
| src\file.rs                       | ✔️
| src\index.rs                      | ✔️

<h2 name="0.8.0">0.8.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\file.rs                       | ✔️ | `OsStr`
| src\span.rs                       | ✔️

<h2 name="0.7.0">0.7.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | -`heapsize`
| Cargo.toml.orig                   | ✔️ | -`heapsize`
| src\file.rs                       | ✔️
| src\index.rs                      | ✔️
| src\lib.rs                        | ✔️
| src\location.rs                   | ✔️
| src\span.rs                       | ✔️

<h2 name="0.6.0">0.6.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | `unicode-segmentation` version bump
| Cargo.toml.orig                   | ✔️ | `unicode-segmentation` version bump
| src\file.rs                       | ✔️ | `Default` for `Files`

<h2 name="0.5.0">0.5.0</h2>

Trivial

<h2 name="0.4.1">0.4.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\span.rs                       | ✔️ | `Default` for `Span`

<h2 name="0.4.0">0.4.0</h2>

Major overhaul
* CodeMap dropped?
* FileMap => Files?
* Fixed direct unsound heapsize use

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | -`failure`, -`itertools`, +`unicode-segmentation`
| Cargo.toml.orig                   | ✔️ | -`failure`, -`itertools`, +`unicode-segmentation`
| src\file.rs                       | ✔️
| src\index.rs                      | ✔️
| src\lib.rs                        | ✔️
| src\location.rs                   | ✔️
| src\span.rs                       | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | heapsize / unsafe removed

<h2 name="0.3.0">0.3.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | 2018 edition
| Cargo.toml.orig                   | ✔️ | 2018 edition
| src\codemap.rs                    | ✔️
| src\filemap.rs                    | ❗️ | =`unsafe` heapsize PathBuf alloc assumptions
| src\index.rs                      | ✔️
| src\lib.rs                        | ✔️
| src\span.rs                       | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗️ | `heapsize` PathBuf allocation assumptions appear bogus

<h2 name="0.2.1">0.2.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | `itertools` version bump
| Cargo.toml.orig                   | ✔️ | `itertools` version bump
| src\codemap.rs                    | ✔️ | +`iter()`
| src\filemap.rs                    | ❗️ | =`unsafe` heapsize PathBuf alloc assumptions
| src\index.rs                      | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗️ | `heapsize` PathBuf allocation assumptions appear bogus

<h2 name="0.2.0">0.2.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo_vcs_info.json              | ✔️ | New
| Cargo.toml.orig                   | ✔️ | Deps reorder
| src\filemap.rs                    | ❗️ | =`unsafe` heapsize PathBuf alloc assumptions

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗️ | `heapsize` PathBuf allocation assumptions appear bogus

<h2 name="0.1.3">0.1.3</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | +`heapsize`
| Cargo.toml.orig                   | ✔️ | +`heapsize`
| src\codemap.rs                    | ✔️
| src\filemap.rs                    | ❗️ | +`unsafe` heapsize PathBuf alloc assumptions
| src\index.rs                      | ✔️
| src\lib.rs                        | ✔️
| src\span.rs                       | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗️ | `heapsize` PathBuf allocation assumptions appear bogus

<h2 name="0.1.3/src/filemap.rs">src/filemap.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 79    | ❗️ `unsafe { heapsize::heap_size_of(s.as_ptr()) }` - makes assumptions about PathBuf allocating on the heap, which is not guaranteed (custom allocators can mess badly with this)

<h2 name="0.1.2">0.1.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ❔ | Relative README.md?, itertools (dev)
| Cargo.toml.orig                   | ❔ | Relative README.md?, itertools (dev)
| src\codemap.rs                    | ✔️
| src\filemap.rs                    | ✔️
| src\index.rs                      | ✔️
| src\lib.rs                        | ✔️
| src\span.rs                       | ✔️

<h2 name="0.1.1">0.1.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | metadata
| Cargo.toml.orig                   | ✔️ | metadata

<h2 name="0.1.0">0.1.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| Cargo.toml                        | ✔️ | `Apache-2.0`, serde, failure, pretty_assertions (dev)
| Cargo.toml.orig                   | ✔️ | `Apache-2.0`, serde, failure, pretty_assertions (dev)
| README.md                         | ✔️
| src\codemap.rs                    | ✔️
| src\filemap.rs                    | ✔️
| src\index.rs                      | ✔️
| src\lib.rs                        | ✔️
| src\span.rs                       | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None so far
| fs        | ✔️ | FileMap::from_disk
| io        | ✔️ | FileMap
| docs      | ❔ | Initially poor
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
