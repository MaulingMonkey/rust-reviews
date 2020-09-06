---
category:       Async
description:    Basic 0-dependencies Fn-based Waker source.
---

# waker-fn

Basic 0-dependencies Fn-based [Waker] source.

This could eventually be made safe when [Wake] (not [Waker]!) stabilizes.
In the meantime, this crate manually creates a [Waker] via [RawWaker].
This requires some `unsafe` code, but this crate appropriately uses just about
the bare minimum necessary to accomplish the task, and appears to do so soundly and correctly.
Additionally, the code is minimal (63 LOC including comments and whitespace for 1.1.0) and straightforward.

[Wake]:     https://doc.rust-lang.org/std/task/trait.Wake.html
[Waker]:    https://doc.rust-lang.org/std/task/struct.Waker.html
[RawWaker]: https://doc.rust-lang.org/std/task/struct.RawWaker.html

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [1.1.0] | high | high | positive | Trivial no_std support
| [1.0.0] | high | high | positive | Full review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[1.1.0]: #1.1.0
[1.0.0]: #1.0.0

<h2 name="1.1.0">1.1.0</h2>

no_std support

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| src\lib.rs                        | ✔️

<h2 name="1.0.0">1.0.0</h2>

Full Review

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| .cargo_vcs_info.json              | ✔️
| .github\FUNDING.yml               | ✔️ | Sponsorship stuff, not billing customization sadly
| .github\workflows\build-and-test.yaml | ✔️
| .github\workflows\lint.yaml       | ✔️
| .github\workflows\security.yaml   | ✔️
| .gitignore                        | ✔️
| CHANGELOG.md                      | ✔️
| Cargo.toml                        | ✔️ | `Apache-2.0 OR MIT`
| Cargo.toml.orig                   | ✔️ | `Apache-2.0 OR MIT`
| LICENSE-APACHE                    | ✔️ | `Apache-2.0`
| LICENSE-MIT                       | ✔️ | `MIT`
| README.md                         | ✔️
| src\lib.rs                        | ⚠️ | `unsafe`, audited

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | The minimum necessary: `RawWaker` + `RawWakerVTable`
| fs        | ✔️ | None
| io        | ✔️ | None
| docs      | ✔️ | Reasonable
| tests     | ⚠️ | One doc test that doesn't exercise clone

<h2 name="1.0.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 26    | ✔️ `unsafe { Waker::from_raw(...) }` - appears sound.  `raw` should be valid & last.  Type signature of original Arc and cast-to (`*const F`) Arcs match exactly.
| 39    | ✔️ `unsafe fn clone_waker` - appears sound.  Resulting `RawWaker` has `raw` which should be valid & last.  Right vtable.
| 40    | ✔️ (unsafe) `Arc::from_raw` - appears sound (valid lifetime, right cast).  ManuallyDrop leaves refcount alone, forgetting a clone increments it.
| 45    | ✔️ `unsafe fn wake` - appears sound.  Consumes refcount.
| 46    | ✔️ (unsafe) `Arc::from_raw` - appears sound (valid lifetime, right cast).  This will consume a refcount (as expected!)
| 50    | ✔️ `unsafe fn wake_by_ref` - appears sound.  Leaves refcount alone.
| 51    | ✔️ (unsafe) `Arc::from_raw` - appears sound (valid lifetime, right cast).  ManuallyDrop leaves refcount alone.
| 55    | ✔️ `unsafe fn drop_waker` - appears sound.  Consumes refcount.
| 56    | ✔️ (unsafe) `Arc::from_raw` - appears sound (valid lifetime, right cast).  This will consume a refcount (as expected!).  Manual call to `drop` is unnecessary but harmless?

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
