---
category:       General Utility
description:    Simple basic thread pool
msrv:           1.9.0
---

# threadpool

Simple basic thread pool

Pros:
* Documented/tested MSRV
* Uses basic well tested mpsc channels
* No unsafe
* Well tested, documented, looks solid

Cons:
* No high performance work stealing queues to break up sync points for work submission
* No auto-scaling options
* Ignores OS thread pools (which might have better auto-scaling magic)
* Unclamped use of frequently mad num_cpus crate could result in ThreadPool::default causing far too many threads resulting in OOM crashes.

## Issues

| issue                     | severity  | broke     | fix       | desc |
| ------------------------- | --------- | --------- | --------- | ---- |
| [#6]                      | ❗️ high    | [0.1.1]   | [0.2.0]   | ScopedPool relied on unsound [JoinGuard]

[#6]:       https://github.com/rust-threadpool/rust-threadpool/pull/6

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [1.8.1] | medium  | medium| ✔️ positive | 
| [1.8.0] | medium  | medium| ✔️ positive | 
| [1.7.1] | medium  | medium| ✔️ positive | 
| [1.7.0] | medium  | medium| ✔️ positive | Join waves
| [1.6.0] | medium  | medium| ✔️ positive | Builder
| [1.5.0] | medium  | medium| ✔️ positive | Clone, Eq for ThreadPool, frewsxcv -> rust-threadpool
| [1.4.1] | medium  | medium| ✔️ positive | ThreadPool::default uses num_cpus
| [1.4.0] | medium  | medium| ✔️ positive | ThreadPool::queued_count, join
| [1.3.2] | medium  | high  | ✔️ positive | MSRV 1.9.0
| [1.3.1] | medium  | high  | ✔️ positive |
| [1.3.0] | medium  | high  | ✔️ positive | MSRV 1.4.0
| [1.2.0] | medium  | high  | ✔️ positive | ThreadPool::panic_count
| [1.1.1] | medium  | high  | ✔️ positive | 
| [1.1.0] | medium  | high  | ✔️ positive | ThreadPool::new_pool
| [1.0.2] | medium  | high  | ✔️ positive | RwLock -> AtomicUsize
| [1.0.1] | medium  | high  | ✔️ positive | Mutex -> RwLock
| [1.0.0] | medium  | high  | ✔️ positive | docs
| [0.2.1] | medium  | high  | ✔️ positive | ThreadPool::{active_count, max_count, set_threads}
| [0.2.0] | medium  | high  | ✔️ positive | \[~~[#6]~~\]: [JoinGuard] use removed (dropped ScopedPool), rust-lang -> frewsxcv
| [0.1.4] | medium  | high  | ⚠️ negative | \[[#6]\]: [JoinGuard] use hidden behind non-default "scoped-pool" feature
| [0.1.3] | medium  | high  | ❗️ negative | ~~`#[unsafe_destructor]`~~
| [0.1.2] | medium  | high  | ❗️ negative |
| [0.1.1] | medium  | high  | ❗️ negative | \[[#6]\]: [JoinGuard]
| [0.1.0] | medium  | high  | ❔ neutral | `#[unsafe_destructor]`

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[1.8.1]: #1.8.1
[1.8.0]: #1.8.0
[1.7.1]: #1.7.1
[1.7.0]: #1.7.0
[1.6.0]: #1.6.0
[1.5.0]: #1.5.0
[1.4.1]: #1.4.1
[1.4.0]: #1.4.0
[1.3.2]: #1.3.2
[1.3.1]: #1.3.1
[1.3.0]: #1.3.0
[1.2.0]: #1.2.0
[1.1.1]: #1.1.1
[1.1.0]: #1.1.0
[1.0.2]: #1.0.2
[1.0.1]: #1.0.1
[1.0.0]: #1.0.0
[0.2.1]: #0.2.1
[0.2.0]: #0.2.0
[0.1.4]: #0.1.4
[0.1.3]: #0.1.3
[0.1.2]: #0.1.2
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="1.8.1">1.8.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGES<span>.</span>md                                   | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | Formatting, markdown syntax typo

<h2 name="1.8.0">1.8.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️ | New file
| CHANGES<span>.</span>md                                   | ⚠️ | Clobbered 1.7.0 with 1.7.1's changes?
| Cargo<span>.</span>toml                                   | ✔️ | `include`, `num_cpus` 1.6 -> 1.13
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | `include`, `num_cpus` 1.6 -> 1.13
| src\lib<span>.</span>rs                                   | ✔️ | docs, formatting, var names

<h2 name="1.7.1">1.7.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>gitignore                                   | ✔️ | .kate-swp
| CHANGES<span>.</span>md                                   | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | join waves

<h2 name="1.7.0">1.7.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGES<span>.</span>md                                   | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | keywords, categories
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | keywords, categories
| src\lib<span>.</span>rs                                   | ✔️ | +`Builder`, stack size

<h2 name="1.6.0">1.6.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGES<span>.</span>md                                   | ✔️ | `frewsxcv/rust-threadpool` -> `rust-threadpool/rust-threadpool`
| Cargo<span>.</span>toml                                   | ✔️ | `frewsxcv/rust-threadpool` -> `rust-threadpool/rust-threadpool`, add to authors, lib.rs -> src/lib.rs
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | `frewsxcv/rust-threadpool` -> `rust-threadpool/rust-threadpool`, add to authors, lib.rs -> src/lib.rs
| README<span>.</span>md                                    | ✔️ | `frewsxcv/rust-threadpool` -> `rust-threadpool/rust-threadpool`
| src\lib<span>.</span>rs                                   | ✔️ | Panic messages, comments, `Clone`, `Eq` for `ThreadPool`

<h2 name="1.5.0">1.5.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGES<span>.</span>md                                   | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | +`num_cpus`
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | +`num_cpus`
| lib<span>.</span>rs                                       | ⚠️ | `Default` for `ThreadPool` (num_cpus lies enough that this should be clamped IMO)

<h2 name="1.4.1">1.4.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGES<span>.</span>md                                   | ✔️
| appveyor<span>.</span>yml                                 | ✔️ | stable, typo "treadpool" CRATE_NAME
| lib<span>.</span>rs                                       | ✔️ | Mass reformat, more docs, new_with_name deprecated in favor of with_name

<h2 name="1.4.0">1.4.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | +`stable` CI
| CHANGES<span>.</span>md                                   | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | **New** (autogen)
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | docs.rs link instead of custom docs, categories = ["concurrency"]
| README<span>.</span>md                                    | ✔️ | docs.rs link instead of custom docs
| lib<span>.</span>rs                                       | ✔️ | Docs, extract `ThreadPoolSharedData`. + ThreadPool::queued_count, join

<h2 name="1.3.2">1.3.2</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | MSRV bump from "1.4.0" -> "1.9.0"
| CHANGES<span>.</span>md                                   | ✔️
| README<span>.</span>md                                    | ✔️ | Header tweaks, document MSRV
| lib<span>.</span>rs                                       | ✔️ | uncomment `#[deprecated]` (stable in 1.9.0?), tweak assert format

<h2 name="1.3.1">1.3.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | Add to authors, flatten src/lib.rs -> lib.rs
| lib<span>.</span>rs                                       | ✔️ | Move bulk of ThreadPool docs onto crate, add more examples, docs, avoid set_num_threads underflow, Debug for ThreadPool

<h2 name="1.3.0">1.3.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | MSRV bump from  "1.0.0" -> "1.4.0"
| CHANGELOG<span>.</span>md                                 | ✔️
| README<span>.</span>md                                    | ✔️ | Link `crossbeam` as an alternative
| src\lib<span>.</span>rs                                   | ✔️ | More docs, formatting, deprecate set_threads for set_num_threads, sleep_ms -> sleep

<h2 name="1.2.0">1.2.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| rustup<span>.</span>sh                                    | ❔ | Unused, unreviewed
| src\lib<span>.</span>rs                                   | ✔️ | +`ThreadPool::panic_count`
| target-install\release\deps\\*<span>.</span>rlib          | ⚠️ | Unused, unreviewed, shouldn't have been part of the package

<h2 name="1.1.1">1.1.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | docs, formatting cleanups
| src\lib<span>.</span>rs<span>.</span>bk                   | ⚠️ | Unused, unreviewed, shouldn't have been part of package

<h2 name="1.1.0">1.1.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | +`ThreadPool::new_pool(name, threads)`

<h2 name="1.0.2">1.0.2</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | Mutex&lt;usize&gt; -> AtomicUsize, doc tweaks

<h2 name="1.0.1">1.0.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | Mutex -> RwLock

<h2 name="1.0.0">1.0.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | -`sudo`
| README<span>.</span>md                                    | ✔️ | `Apache-2.0 OR MIT`, doc link fix, similar lib links, contrib policy, deps example update

<h2 name="0.2.1">0.2.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | + `ThreadPool`::{active_count, max_count, set_threads}

<h2 name="0.2.0">0.2.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | 1.0.0 / beta / nightly coverage, travis-cargo pip
| Cargo<span>.</span>toml                                   | ✔️ | Links changed (rust-lang -> frewsxcv?!?)
| README<span>.</span>md                                    | ✔️ | Links changed (rust-lang -> frewsxcv?!?)
| src\lib<span>.</span>rs                                   | ✔️ | `Clone`able `ThreadPool`, remove unsound [JoinGuard]-using ScopedPool

<h2 name="0.1.4">0.1.4</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | `scoped-pool` feature
| Cargo<span>.</span>toml                                   | ✔️ | `scoped-pool` feature
| src\lib<span>.</span>rs                                   | ✔️ | `scoped-pool` feature

<h2 name="0.1.3">0.1.3</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | -`#[unsafe_destructor]`

<h2 name="0.1.2">0.1.2</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | Doc tweaks, `#[should_fail]` -> `#[should_panic]`

<h2 name="0.1.1">0.1.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ❗️ | ScopedPool using [JoinGuard]

* ✔️ lifetime for Thunk
* ❗️ ScopedPool relying on the removed-as-unsound [JoinGuard] / std\:\:thread\:\:scoped

<h2 name="0.1.0">0.1.0</h2>

Full Review

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>gitignore                                   | ✔️
| <span>.</span>travis<span>.</span>yml                     | ⚠️ | gh-pages publishing nonsense
| Cargo<span>.</span>toml                                   | ❔ | `MIT/Apache-2.0`
| LICENSE-APACHE                                            | ✔️ | `Apache-2.0`
| LICENSE-MIT                                               | ✔️ | `MIT`
| README<span>.</span>md                                    | ✔️ |
| src\lib<span>.</span>rs                                   | ⚠️ | `Apache-2.0 OR MIT` header, `#[unsafe_destructor]`

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ⚠️ | `#[unsafe_destructor]`
| fs        | ✔️ | None
| io        | ✔️ | None
| docs      | ✔️ | Excellent
| tests     | ✔️ | Decent

<h2 name="0.1.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 50    | ❔ `#[unsafe_destructor]` ?  Ancient attributes no longer worth worrying about?
| ...   | ✔️

[JoinGuard]:        https://github.com/rust-lang/rust/issues/24292

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
