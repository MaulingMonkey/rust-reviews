---
category:       Unsound
description:    Super basic casting/endian/swizzling with a history of unsoundness
msrv:           1.12.0
crev:           negative
---

# byteorder

## Pros

* MSRV policy!
* Basic swizzling/endian stuff
* You didn't have to write it

## Cons

* Excessive and distributed `unsafe` in serialization related code is hard to audit and makes me nervous
* History of unsoundness.  Most crate versions have alignment related bugs, and 0.2.x had overflow issues which regressed in 0.3.x.
* Very limited functionality

## Issues

| issue                     | severity  | broke     | fix      | desc |
| ------------------------- | --------- | --------- | -------- | ---- |
| <a name="na-1">N/A#1</a>  | ❗️ high    | [0.2.0]   | [0.2.2]  | Unsound: missing bounds checks for {`read`,`write`}`_num_bytes!`
| [rust-lang#22776]         | ❗️ high    | [0.2.0]<br>[0.3.3]🐵 | [0.2.11]<br>[0.3.8] | Unsound: dangling pointer `bytes` in `write_num_bytes!`<br>.. regression
| [#47]                     | ❗️ high    | [0.3.9]   | [0.5.2]  | Unsound: unaligned read for `read_num_bytes!`
| [#157]                    | ❗️ high    | [1.1.0]🐵 | [1.3.3]  | Unsound: unaligned read for `read_uint`\[`128`\]

🐵 Spotted in audit

[rust-lang#22776]:  https://github.com/rust-lang/rust/issues/22776
[#47]:              https://github.com/BurntSushi/byteorder/issues/47
[#157]:             https://github.com/BurntSushi/byteorder/pull/157
[N/A#1]:            #na-1

## Audit

I managed to miss several soundness bugs on my first pass through the code.

I suspect 1.3.3 and 1.3.4 are sound, but this crate could really stand to
centralize it's unsafe code in one spot where the absolute minimum amount
possible is used.  Instead, it's scattered across 5 KLOC, with a history of
unsoundness.  Considering the only thing this crate really does is cast between
numeric types, slices thereof, and endians - that's a bit much.

| version   | thoroughness  | understanding | rating | notes |
| --------- | ------------- | ------------- | ------ | ----- |
| [1.3.4]   | low           | medium        | ⚠️ negative   | 
| [1.3.3]   | low           | medium        | ⚠️ negative   | Fixed: ~~[#157]~~
| [1.3.2]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.3.1]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.3.0]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.7]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.6]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.5]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.4]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.3]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.2]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.1]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.2.0]   | low           | medium        | ❗️ dangerous   | Unsound!  [#157]
| [1.1.0]   | low           | medium        | ❗️ dangerous   | Unsound!  **[#157]**
| [1.0.0]   | low           | medium        | ❔ neutral    |
| [0.5.3]   | low           | medium        | ❔ neutral    |
| [0.5.2]   | low           | medium        | ❔ neutral    | Fixed: ~~[#47]~~
| [0.5.1]   | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.5.0]   | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.4.2]   | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.4.1]   | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.4.0]   | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.3.13]  | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.3.12]  | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.3.11]  | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.3.10]  | low           | medium        | ❗️ dangerous   | Unsound!  [#47]
| [0.3.9]   | low           | medium        | ❗️ dangerous   | Unsound!  **[#47]**
| [0.3.8]   | low           | medium        | ❔ neutral    | Fixed: ~~[rust-lang#22776]~~
| [0.3.7]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.3.6]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.3.5]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.3.4]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.3.3]   | low           | medium        | ❗️ dangerous   | Unsound!  **[rust-lang#22776]** (regression)
| [0.3.2]   | low           | medium        | ❔ neutral    |
| [0.3.1]   | low           | medium        | ❔ neutral    |
| [0.3.0]   | low           | medium        | ❔ neutral    |
| [0.2.14]  | low           | medium        | ❔ neutral    |
| [0.2.13]  | low           | medium        | ❔ neutral    |
| [0.2.12]  | low           | medium        | ❔ neutral    |
| [0.2.11]  | low           | medium        | ❔ neutral    | Fixed: ~~[rust-lang#22776]~~
| [0.2.10]  | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.9]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.8]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.7]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.6]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.5]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.4]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.3]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776]
| [0.2.2]   | low           | medium        | ❗️ dangerous   | Unsound!  [rust-lang#22776] (Fixed: ~~[N/A#1]~~)
| [0.2.1]   | low           | medium        | ❗️ dangerous   | Unsound!  [N/A#1] [rust-lang#22776]
| [0.2.0]   | low           | medium        | ❗️ dangerous   | Unsound!  **[N/A#1] [rust-lang#22776]**
| [0.1.1]   | medium        | medium        | ❔ neutral    | initial implementation
| [0.1.0]   | high          | medium        | ⚠️ negative   | empty

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[1.3.4]: #1.3.4
[1.3.3]: #1.3.3
[1.3.2]: #1.3.2
[1.3.1]: #1.3.1
[1.3.0]: #1.3.0
[1.2.7]: #1.2.7
[1.2.6]: #1.2.6
[1.2.5]: #1.2.5
[1.2.4]: #1.2.4
[1.2.3]: #1.2.3
[1.2.2]: #1.2.2
[1.2.1]: #1.2.1
[1.2.0]: #1.2.0
[1.1.0]: #1.1.0
[1.0.0]: #1.0.0
[0.5.3]: #0.5.3
[0.5.2]: #0.5.2
[0.5.1]: #0.5.1
[0.5.0]: #0.5.0
[0.4.2]: #0.4.2
[0.4.1]: #0.4.1
[0.4.0]: #0.4.0
[0.3.13]: #0.3.13
[0.3.12]: #0.3.12
[0.3.11]: #0.3.11
[0.3.10]: #0.3.10
[0.3.9]: #0.3.9
[0.3.8]: #0.3.8
[0.3.7]: #0.3.7
[0.3.6]: #0.3.6
[0.3.5]: #0.3.5
[0.3.4]: #0.3.4
[0.3.3]: #0.3.3
[0.3.2]: #0.3.2
[0.3.1]: #0.3.1
[0.3.0]: #0.3.0
[0.2.14]: #0.2.14
[0.2.13]: #0.2.13
[0.2.12]: #0.2.12
[0.2.11]: #0.2.11
[0.2.10]: #0.2.10
[0.2.9]: #0.2.9
[0.2.8]: #0.2.8
[0.2.7]: #0.2.7
[0.2.6]: #0.2.6
[0.2.5]: #0.2.5
[0.2.4]: #0.2.4
[0.2.3]: #0.2.3
[0.2.2]: #0.2.2
[0.2.1]: #0.2.1
[0.2.0]: #0.2.0
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="1.3.4">1.3.4</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| build.rs                          | ✔️ | Squelch `try!` warnings (MSRV)
| src\lib.rs                        | ✔️ | Squelch `try!` warnings (MSRV)

<h2 name="1.3.3">1.3.3</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ⚠️ | +`write_i8_into`, fix misaligned reads ~~[#157]~~

<h2 name="1.3.2">1.3.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.lock                        | ✔️ | New
| Cargo.toml                        | ✔️ | +`doc-comment`(dev)
| Cargo.toml.orig                   | ✔️ | +`doc-comment`(dev)
| README.md                         | ✔️
| build_script_build-*              | ⚠️ | should not be part of the crate
| byteorder-*                       | ⚠️ | should not be part of the crate
| src\io.rs                         | ⚠️ | New `read_i8_into` `unsafe { slice_to_u8_mut(dst) }` appears sound/safe
| src\lib.rs                        | ✔️❗️ | docs

<h2 name="1.3.1">1.3.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️
| Cargo.toml.orig                   | ✔️
| build.rs                          | ✔️

<h2 name="1.3.0">1.3.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| Cargo.toml                        | ✔️ | dev-deps
| Cargo.toml.orig                   | ✔️ | dev-deps, docs
| benches\bench.rs                  | ✔️
| build.rs                          | ✔️ | Auto-support i128 on rustc 1.26.0+
| src\io.rs                         | ✔️
| src\lib.rs                        | ✔️❗️

<h2 name="1.2.7">1.2.7</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo_vcs_info.json              | ✔️
| Cargo.toml                        | ✔️ | -`ci`, clarify `Unlicense/MIT` -> `Unlicense OR MIT`
| Cargo.toml.orig                   | ✔️ | -`ci`, clarify `Unlicense/MIT` -> `Unlicense OR MIT`

<h2 name="1.2.6">1.2.6</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| benches\bench.rs                  | ✔️
| src\lib.rs                        | ✔️❗️

<h2 name="1.2.5">1.2.5</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | quickcheck/rand version bumps
| Cargo.toml.orig                   | ✔️ | quickcheck/rand version bumps
| src\io.rs                         | ✔️ | docs
| src\lib.rs                        | ✔️❗️ | -`html_root_url`

<h2 name="1.2.4">1.2.4</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| src\io.rs                         | ✔️ | +`read_u48`, +`read_i48`
| src\lib.rs                        | ✔️❗️


<h2 name="1.2.3">1.2.3</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | Simplify docs

<h2 name="1.2.2">1.2.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .travis.yml                       | ✔️
| Cargo.toml                        | ✔️
| Cargo.toml.orig                   | ✔️
| ci\script.sh                      | ✔️
| src\io.rs                         | ✔️ | docs, -`mut`, +`#[deprecated]`
| src\lib.rs                        | ⚠️❗️ | Replaced a lot of transmutes with pointer cast chains

<h2 name="1.2.1">1.2.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| src\io.rs                         | ⚠️ | `unsafe { slice_to_u8_mut(...) }`s appear sound
| src\lib.rs                        | ✔️❗️

<h2 name="1.2.0">1.2.0</h2>

* Assume `u32` -> `f32` transmutes are sound (acceptable to me, considering so does std)

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| Cargo.toml                        | ✔️
| Cargo.toml.orig                   | ✔️
| README.md                         | ✔️
| src\io.rs                         | ✔️ | -`mut`s
| src\lib.rs                        | ✔️❗️ | -NaN nonsense, some safe new transmutes


<h2 name="1.1.0">1.1.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .travis.yml                       | ✔️
| CHANGELOG.md                      | ✔️ | "Litz Blitz evaluation" overview
| Cargo.toml                        | ✔️ | +`i128` feature
| Cargo.toml.orig                   | ✔️ | +`i128` feature
| benches\bench.rs                  | ✔️
| [src\io.rs](#1.1.0-src-io-rs)     | ⚠️ | Extension methods galore.  Well documented at least.
| [src\lib.rs](#1.0.0-src-lib-rs)   | ❗️ | Unaligned reads!  +docs, trait sealing.  A lot of new `unsafe` in macros with poorly enforced / documented type preconditions.

<h2 name="1.1.0-src-io-rs">src\io.rs</h2>

| Line | Notes |
| ---- | ----- |
|  512 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (u16) legal
|  547 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (u32) legal
|  585 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (u64) legal
|  627 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (u128) legal
|  662 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (i16) legal
|  697 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (i32) legal
|  735 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (i64) legal
|  777 | ✔️ `unsafe { slice_to_u8_mut(dst) }` - sound, all bit patterns of dst (i128) legal
|  824 | ✔️ `unsafe fn` - SNAN edge cases well documented
|  876 | ✔️ `unsafe fn` - SNAN edge cases well documented
| 1184 | ⚠️ `unsafe fn` slice_to_u8_mut - wildly unsafe, but internal and decntly documented

<h2 name="1.1.0-src-lib-rs">src\lib.rs</h2>

| Line | Notes |
| ---- | ----- |
|  842 | ✔️ `unsafe { transmute(n) }` - sound, `f32` -> `u32`
|  865 | ✔️ `unsafe { transmute(n) }` - sound, `f64` -> `u64`
|  984 | ✔️ `unsafe { transmute(n) }` - sound, `&mut [i16]`  -> `&mut [u16]`
| 1010 | ✔️ `unsafe { transmute(n) }` - sound, `&mut [i32]`  -> `&mut [u32]`
| 1036 | ✔️ `unsafe { transmute(n) }` - sound, `&mut [i64]`  -> `&mut [u64]`
| 1063 | ✔️ `unsafe { transmute(n) }` - sound, `&mut [i128]` -> `&mut [u128]`
| 1097 | ✔️ `unsafe fn`, `transmute(n)` - properly documented, `&mut [f32]` -> `&mut [u32]`
| 1132 | ✔️ `unsafe fn`, `transmute(n)` - properly documented, `&mut [f64]` -> `&mut [u64]`
| 1251 | ✔️ `unsafe { transmute(src) }` - sound, `&[i16]`  -> `&[u16]`
| 1276 | ✔️ `unsafe { transmute(src) }` - sound, `&[i32]`  -> `&[u32]`
| 1301 | ✔️ `unsafe { transmute(src) }` - sound, `&[i64]`  -> `&[u64]`
| 1327 | ✔️ `unsafe { transmute(src) }` - sound, `&[i128]` -> `&[u128]`
| 1355 | ✔️ `unsafe { transmute(src) }` - sound, `&[f32]`  -> `&[u32]` (all bit patterns legal)
| 1383 | ✔️ `unsafe { transmute(src) }` - sound, `&[f64]`  -> `&[u64]` (all bit patterns legal)
| 1504 | ✔️ `unsafe { transmute(numbers) }` - sound, `&[i16]`  -> `&[u16]`
| 1530 | ✔️ `unsafe { transmute(numbers) }` - sound, `&[i32]`  -> `&[u32]`
| 1556 | ✔️ `unsafe { transmute(numbers) }` - sound, `&[i64]`  -> `&[u64]`
| 1585 | ✔️ `unsafe { transmute(numbers) }` - sound, `&[i128]` -> `&[u128]`
| 1710 | ✔️ `unsafe { copy_nonoverlapping(...) }` - sound, `read_num_bytes!`        appears to be free from overflow, misalignment, or dangling memory bugs ($src u8 type enforced via copy_nonoverlapping signature)
| 1723 | ✔️ `unsafe { copy_nonoverlapping(...) }` - sound, `write_num_bytes!`       appears to be free from overflow, misalignment, or dangling memory bugs ($src u8 type enforced via copy_nonoverlapping signature)
| 1735 | ⚠️ `unsafe { copy_nonoverlapping(...) }` - unsound but internal `read_slice!` is always correctly used.  Would be sound if it `assert_eq!($size, ...)`ed like write_slice_native does.
| 1752 | ✔️ `unsafe { copy_nonoverlapping(...) }` - sound, `write_slice_native!`    appears to be free from overflow, misalignment, or dangling memory bugs ($src u8 type enforced via copy_nonoverlapping signature)
| 1799 | ❗️ **[#157]** `unsafe { copy_nonoverlapping(...) }` is sound, but the pointer cast inside the same block isn't (misaligned read!)
| 1812 | ❗️ **[#157]** `unsafe { copy_nonoverlapping(...) }` is sound, but the pointer cast inside the same block isn't (misaligned read!)
| 1844 | ✔️ `unsafe { copy_nonoverlapping(...) }` - appears to be free from overflow, misalignment, or dangling memory bugs (bytes u8 type enforced via copy_nonoverlapping signature)
| 1858 | ✔️ `unsafe { copy_nonoverlapping(...) }` - appears to be free from overflow, misalignment, or dangling memory bugs (bytes u8 type enforced via copy_nonoverlapping signature)
| 1966 | ✔️ `unsafe { transmute(*n) }` - sound, `f32` -> `u32`
| 1976 | ✔️ `unsafe { transmute(*n) }` - sound, `f64` -> `u64`
| 2010 | ❗️ **[#157]** `unsafe { copy_nonoverlapping(...) }` is sound, but the pointer cast inside the same block isn't (misaligned read!)
| 2022 | ❗️ **[#157]** `unsafe { copy_nonoverlapping(...) }` is sound, but the pointer cast inside the same block isn't (misaligned read!)
| 2053 | ✔️ `unsafe { transmute, copy_nonoverlapping, ... }` appears sound
| 2064 | ✔️ `unsafe { transmute, copy_nonoverlapping, ... }` appears sound
| 2169 | ✔️ `unsafe { transmute(n) }` - sound, `f32` -> `u32`
| 2179 | ✔️ `unsafe { transmute(n) }` - sound, `f64` -> `u64`
| 2201 | ✔️ `unsafe { transmute(u) }` - sound, `u32` -> `f32` but after special casing SNANs (verified bit patterns)
| 2220 | ✔️ `unsafe { transmute(u) }` - sound, `u64` -> `f64` but after special casing SNANs (verified bit patterns)
| 2809 | ❔ `unsafe { ... }` test code
| ...  | ❔ `unsafe { ... }` test code
| 3164 | ❔ `unsafe { ... }` test code

<h2 name="1.0.0">1.0.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .travis.yml                       | ✔️ | MSRV 1.12.0
| Cargo.toml                        | ✔️
| README.md                         | ✔️
| src\lib.rs                        | ✔️ | Mess of traits, test code overhauls

<h2 name="0.5.3">0.5.3</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | nostd fix

<h2 name="0.5.2">0.5.2</h2>

* Invert `no-std` -> `std` feature flag (minor breaking change!)

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| README.md                         | ✔️
| src\lib.rs                        | ⚠️ | Fixed unaligned read [[#47]].

<h2 name="0.5.1">0.5.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | `NetworkEndian`

<h2 name="0.5.0">0.5.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| README.md                         | ✔️ | Version bump
| src\lib.rs                        | ✔️ | Trimming
| src\new.rs                        | ✔️ | Trim custom error / {`read`,`write`}`_full`

<h2 name="0.4.2">0.4.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\new.rs                        | ✔️ | `write_[u]int` slice fixes

<h2 name="0.4.1">0.4.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\new.rs                        | ✔️ | `write_[u]int`

<h2 name="0.4.0">0.4.0</h2>

* `no-std` support (eww, negative feature flag)

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ⚠️ | Moved unsafe `read_num_bytes!` logic directly into {`read`,`write`}`_uint`

<h2 name="0.3.13">0.3.13</h2>

Trivial

<h2 name="0.3.12">0.3.12</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | Stop using wildcard deps
| README.md                         | ✔️ | Stop using wildcard deps

<h2 name="0.3.11">0.3.11</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | `#[inline]` spam
| src\new.rs                        | ✔️ | `#[inline]` spam

<h2 name="0.3.10">0.3.10</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .travis.yml                       | ✔️ | channels
| Cargo.toml                        | ✔️ | Keywords
| README.md                         | ✔️ | Remove mention of long dead std::old_io
| benches\bench.rs                  | ✔️
| src\lib.rs                        | ✔️ | docs

<h2 name="0.3.9">0.3.9</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ❗️ | Broke `read_num_bytes!` **again** with unaligned reads [[#47]].
| src\new.rs                        | ✔️

<h2 name="0.3.8">0.3.8</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| README.md                         | ✔️ | badge
| src\lib.rs                        | ⚠️ | Fixed [rust-lang#22776] regression, -`MarkerTrait`

<h2 name="0.3.7">0.3.7</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| README.md                         | ✔️ | Reader -> Read

<h2 name="0.3.6">0.3.6</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| COPYING                           | ✔️ | `Unlicense` -> `Unlicense OR MIT`
| Cargo.toml                        | ✔️ | `Unlicense` -> `Unlicense/MIT`
| LICENSE-MIT                       | ✔️ | `MIT`
| UNLICENSE                         | ✔️
| README.md                         | ✔️ | `Unlicense` -> `Unlicense OR MIT`


<h2 name="0.3.5">0.3.5</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]]
| src\new.rs                        | ✔️  | Error handling

<h2 name="0.3.4">0.3.4</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]]

<h2 name="0.3.3">0.3.3</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | -`bswap`
| src\lib.rs                        | ❗️ | Regression!  Re-introduced [rust-lang#22776] when removing bswap?

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗️

<h2 name="0.3.2">0.3.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | testing

<h2 name="0.3.1">0.3.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | `native_endian` stuff

<h2 name="0.3.0">0.3.0</h2>

* Dropped `std::old_io` support

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️
| src\new.rs                        | ✔️

<h2 name="0.2.14">0.2.14</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️ | `push_all` -> `extend`

<h2 name="0.2.13">0.2.13</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | +`bswap`
| src\lib.rs                        | ✔️ | `*_num_bytes!` -> `bswap`, dropped some `unsafe` (read/write macros)
| src\new.rs                        | ✔️ | error handling tweaks

<h2 name="0.2.12">0.2.12</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\new.rs                        | ✔️ | `Sized` tweaks
| src\old.rs                        | ✔️ | `Sized` tweaks

<h2 name="0.2.11">0.2.11</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ⚠️ | Fixed dangling pointer [[rust-lang#22776]].

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ⚠️

<h2 name="0.2.10">0.2.10</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]] `copy_nonoverlapping_memory` -> `copy_nonoverlapping`
| src\new.rs                        | ✔️ | `FromError`

<h2 name="0.2.9">0.2.9</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| README.md                         | ✔️ | Docs
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]] Trivial
| src\new.rs                        | ✔️ | `Error` / `FromError` / `Display`
| src\old.rs                        | ✔️ | old_io docs

<h2 name="0.2.8">0.2.8</h2>

Significant rewrite/refactor (std::old_io + std::io?)

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]]
| src\new.rs                        | ✔️
| src\old.rs                        | ✔️

<h2 name="0.2.7">0.2.7</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]] MarkerTrait, usize, old_io

<h2 name="0.2.6">0.2.6</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]]

<h2 name="0.2.5">0.2.5</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]].  `unsafe`: new `read_num_bytes!` overloads for `read_uint` appear sound

<h2 name="0.2.4">0.2.4</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ✔️❗️ | [[rust-lang#22776]]

<h2 name="0.2.3">0.2.3</h2>

Trivial

<h2 name="0.2.2">0.2.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| README.md                         | ✔️
| src\lib.rs                        | ❗️ | Fixed {`read`,`write`}`_num_bytes!` bounds checks [[N/A#1]], `bytes` still dangles [[rust-lang#22776]]

<h2 name="0.2.1">0.2.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ +`rand`(dev)
| README.md                         | ✔️ | TODO, Ideas
| src\lib.rs                        | ✔️❗️ | [[N/A#1], [rust-lang#22776]]

<h2 name="0.2.0">0.2.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ❗️ | {`read`,`write`}`_num_bytes!`: unsound: may overflow [[N/A#1]].  `let bytes = (...).as_ptr();` also dangles [[rust-lang#22776]]!

<h2 name="0.1.1">0.1.1</h2>

Initial implementation

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | +`quickcheck` (dev)
| src\lib.rs                        | ✔️

<h2 name="0.1.0">0.1.0</h2>

Empty placeholder crate

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src\lib.rs                        | ❔ | ~empty
| .cargo-ok                         | ✔️
| .gitignore                        | ✔️
| .travis.yml                       | ✔️
| Cargo.toml                        | ❔ | `Unlicense`
| Makefile                          | ❔ | not general purpouse
| README.md                         | ✔️
| session.vim                       | ✔️
| UNLICENSE                         | ✔️ | `Unlicense`

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | None
| io        | ✔️ | Reasonable
| docs      | ⚠️ | None
| tests     | ⚠️ | None



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
