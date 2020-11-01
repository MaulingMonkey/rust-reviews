---
category:       General Utility
description:    `vec![]` but for fixed length arrays
msrv:           1.36.0
---

# array-macro

`vec![]` but for fixed length arrays

Pros:
* Solid and awesome

Cons:
* `unsafe` in macro prevents use with `#![forbid(unsafe_code)]`
* Bumps MSRV on patch versions (but so far only for damn good reasons)

## Issues

| issue                     | severity  | broke     | fix       | desc |
| ------------------------- | --------- | --------- | --------- | ---- |
| [#9] 🐵                   | ❗️ high    |           | [1.0.4]   | [core::mem::uninitialized](https://doc.rust-lang.org/core/mem/fn.uninitialized.html) is deprecated / undefined behavior
| [01940637]                | ❗️ high    | [1.0.4]   | [1.0.5]   | Catch-all traits defining `length` could mess with `array!`

[#9]:                   https://gitlab.com/KonradBorowski/array-macro/-/merge_requests/9
[01940637]:             https://gitlab.com/KonradBorowski/array-macro/-/commit/01940637dd8f3bfeeee3faf9639fa9ae52f19f4d

<!--
🐵 ❔ ⚠️ ❗️

[#1]:                   https://github.com/user/repository/issues/1
[user/repository#1]:    https://github.com/user/repository/issues/1
[user/repository#1]:    https://github.com/user/repository/pull/1
-->

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [1.0.5] | high | high | ✔️ positive | ~~[01940637]~~ Protection against evil catch-all traits defining `length`
| [1.0.4] | high | high | ❔ neutral | **[01940637]** ~~[#9]~~ `MSRV 1.36.0`, `MaybeUninit` should be safe + sound
| [1.0.3] | high | high | ❗️ negative | [#9] Good for the time, but `core::mem::uninitialized` is technically UB
| [1.0.2] | high | high | ❗️ negative | [#9] Good for the time, but `core::mem::uninitialized` is technically UB
| [1.0.1] | high | high | ❗️ negative | [#9] Good for the time, but `core::mem::uninitialized` is technically UB
| [1.0.0] | high | high | ❗️ negative | [#9] Good for the time, but `core::mem::uninitialized` is technically UB
| [0.1.2] | high | high | ❗️ negative | **[#9]** Good for the time, but `core::mem::uninitialized` is technically UB

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         ❌ dangerous ⚠️❗️ negative ❔ neutral ✔️ positive ✔️ strong
-->

[1.0.5]: #1.0.5
[1.0.4]: #1.0.4
[1.0.3]: #1.0.3
[1.0.2]: #1.0.2
[1.0.1]: #1.0.1
[1.0.0]: #1.0.0
[0.1.2]: #0.1.2

<h2 name="1.0.5">1.0.5</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | Protection from evil, internal comments explaining the awkward code
| tests\test<span>.</span>rs                                | ✔️ | Testing protection against evil

<h2 name="1.0.4">1.0.4</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>gitlab-ci<span>.</span>yml                  | ✔️ | MSRV 1.25.0 -> 1.36.0
| src\lib<span>.</span>rs                                   | ✔️ | `uninitialized` -> `MaybeUninit`, moved common code out of macro into crate

<h2 name="1.0.3">1.0.3</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| <span>.</span>gitlab-ci<span>.</span>yml                  | ✔️ | MSRV 1.25.0
| Cargo<span>.</span>toml                                   | ✔️ | moved repositories, badge
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | moved repositories, badge
| README<span>.</span>md                                    | ✔️ | Removed travis badge
| src\lib<span>.</span>rs                                   | ❗✔️ | +`local_inner_macros`, still uses `uninitialized`
| tests\test<span>.</span>rs                                | ✔️

<h2 name="1.0.2">1.0.2</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ❗✔️ | `#[allow(unsafe_code)]` in macro, still uses `uninitialized`
| tests\test<span>.</span>rs                                | ✔️ | eliminated `static mut`

<h2 name="1.0.1">1.0.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ❗✔️ | `::array_macros::__core::` -> `$crate::__core::`, still uses `uninitialized`

<h2 name="1.0.0">1.0.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ❗✔️ | `str` -> `String`, still uses `uninitialized`
| tests\test<span>.</span>rs                                | ⚠️✔️ | Closer scopes, still uses `static mut`

<h2 name="0.1.2">0.1.2</h2>

Decent for the time.  Caveats:

* Doesn't use `$crate::`
* Uses `core::mem::uninitialized` (modern code should use `MaybeUninit`)

Drop order on panic is "[correct](https://doc.rust-lang.org/reference/destructors.html)" (forward, and only created elements)

Writing `vec.position = i` *before* `ptr::write` looks awkward as heck but is correct.<br>
On the first loop iteration,  `vec.position = i = 0`, so nothing will be dropped if `callback(0)` panics.<br>
On the second loop iteration, `vec.position = i = 1`, so the previous element will be dropped if `callback(1)` panics.<br>
After `callback(count-1)`, no panic can occur before the entire array is assumed initialized, ergo skipping `vec.position = i = count-1` isn't a problem.<br>

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>gitignore                                   | ✔️
| <span>.</span>travis<span>.</span>yml                     | ✔️ | MSRV stable
| Cargo<span>.</span>toml                                   | ✔️ | MIT/Apache-2.0
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | MIT/Apache-2.0
| LICENSE-APACHE                                            | ✔️ | Apache-2.0
| LICENSE-MIT                                               | ✔️ | MIT
| README<span>.</span>md                                    | ✔️
| src\lib<span>.</span>rs                                   | ❗
| tests\test<span>.</span>rs                                | ⚠️ | `static mut`s could be made atomic for safety

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗  | `uninitialize` is technically UB
| fs        | ✔️ | None
| io        | ✔️ | None
| docs      | ✔️ | Good
| tests     | ✔️ | Good
