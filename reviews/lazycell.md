---
category:       General Utility
description:    Similar to RefCell<Option<T>>, but you can keep T borrowed
msrv:           1.24.0
---

# lazycell

# Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [1.2.1]   | high | medium | positive | Diff review
| [1.2.0]   | high | medium | positive | Full review, appears high quality

[1.2.1]: #1.2.1
[1.2.0]: #1.2.0


<h2 name="1.2.1">1.2.1</h2>

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| .cargo_vcs_info.json  | ✔️ | New
| CHANGELOG.md          | ✔️ | |
| Cargo.toml            | ✔️ | Version bump only
| Cargo.toml.orig       | ✔️ | Version bump only
| LICENSE-MIT           | ✔️ | Copyright year bump only
| src/lib.rs            | ✔️ | New clone impls are safe, sane, and well unit tested

<h2 name="1.2.0">1.2.0</h2>

| file                              | rating | notes |
| --------------------------------- | ------ | ----- |
| [src/lib.rs](#1.2.0/src/lib.rs)   | ✔️ |  MIT OR Apache-2.0
| .cargo-ok                         | ✔️ | |
| Cargo.toml                        | ❔ | MIT/Apache-2.0
| Cargo.toml.orig                   | ❔ | MIT/Apache-2.0
| CHANGELOG.md                      | ✔️ | Custom hyperlinks, TIL
| LICENSE-APACHE                    | ✔️ | |
| LICENSE-MIT                       | ✔️ | |
| README.md                         | ❔ | MIT and/or Apache-2.0

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | Used roughly only as much as necessary, with small scopes.  All appears to be sound.
| fs        | ✔️ | None
| io        | ✔️ | None
| docs      | ✔️ | Good quality
| tests     | ✔️ | LGTM

<h2 name="1.2.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 5-9   | ✔️ Explicitly "MIT OR Apache-2.0"
| 63    | ✔️ UnsafeCell means LazyCell derives Send, but not Sync (e.g. this is "single threaded" in the sense that there is 1 thread accessing at most at a time.)
| 76    | ✔️ `unsafe { ... }` appears sound - "single threaded", slot is only modified if it was None, in which case there were no outstanding borrows to the data.
| 96    | ✔️ `unsafe { ... }` appears sound - `&mut self` means we have exclusive access, so there should be no outstanding borrows.
| 110   | ✔️ `unsafe { ... }` appears sound - after the fn returns, there should only be an inner borrow if inner was Some.
| 119   | ✔️ `unsafe { ... }` appears sound - after the fn returns, there should only be an inner borrow if inner was Some.  Also, exclusive access.
| 119   | ✔️ `unsafe { ... }` appears sound - after the fn returns, there should only be an inner borrow if inner was Some.  Also, exclusive access.
| 204   | ✔️ `unsafe { ... }` appears sound - block is outright unnecessary except for MSRV compatability purpouses
| 214   | ✔️ `unsafe { ... }` appears sound - after the fn returns, there should be no inner borrow.
| 226   | ⚠️ Despite UnsafeCell, AtomicLazyCell is Send/Sync depending on T
| 250   | ⚠️ `unsafe { ... }` appears sound, but my acquire/release knowledge here is weak.  But I'm fairly sure this is the classic acquire/release pattern.
| 274   | ✔️ `unsafe { ... }` appears sound - `&mut self` means we have exclusive access, so there should be no outstanding borrows.  This also means updating state need not be atomic.
| 278   | ✔️ Atomic only queried
| 289   | ⚠️ `unsafe { ... }` appears sound, but my acquire knowledge here is weak.  But I'm fairly sure this is a classic acquire.
| 300   | ✔️ `unsafe { ... }` appears sound - block is outright unnecessary except for MSRV compatability purpouses
| 311   | ⚠️ `unsafe { ... }` appears sound, but my acquire knowledge here is weak.  But I'm fairly sure this is a classic acquire.
| 317   | ✔️ `unsafe impl Sync<T: Sync + Send>` appears sound.  The `Send` constraint is necessary as fns like `fill` might be called on a thread other than the one that drops the `AtomicLazyCell`.
| 319   | ✔️ `unsafe impl Send<T: Send>` appears sound.
| 322+  | ✔️ skimmed test code

While the use of atomics here is straightforward, in a very classical pattern,
and while I'm relatively confident it's all correct, my weakness here downgrades
my crev understanding to medium here.  This is entirely a reflection on me, and
not the code.

## TIL

The changelog markdown shows using custom HTML for anchors.
I've immediately applied this knowledge to this very review!
Doesn't help with VS Code previewing, sadly, but that's a niche need anyways.


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



-->
