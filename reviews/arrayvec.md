---
category:       General Utility
description:    Vec clone (Fixed capacity, no heap). Prefer Vec?
---

# arrayvec

Stack/value variable length arrays without heap fallback.

Pros:
* Maybe sound?
* Better than what you'll write.

Cons:
* History of unsoundness (0.4.10 and earlier)
* Disturbing amounts of unsafe


| version | thoroughness | understanding | rating | Notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.5.1](#051)   | medium | medium | neutral |
| [0.5.0](#050)   | medium | medium | neutral |
| [0.4.11](#0411) | high | medium | neutral  |
| [0.4.10](#0410) | high | medium | negative | Unsound
| [0.4.9](#049)   | high | medium | negative | Unsound
| [0.4.8](#048)   | high | medium | negative | Unsound
| [0.4.7](#047)   | high | medium | negative | Full review

0.5.1
=====

This version switched some slices possibly containing uninit (UB!) to use
pointers instead.  This makes `encode_utf8` unsafe, sadly.

| Diff                                  | Rating | Notes |
| ------------------------------------- | ------ | ----- |
| .cargo_vcs_info.json                  | +1 | |
| .gitignore                            | +1 | |
| Cargo.lock                            | +1 | Rust version bump?
| Cargo.toml                            | +1 | debug \[profile.*\]
| Cargo.toml.orig                       | +1 | debug \[profile.*\]
| README.rst                            | +1 | |
| *.{events,string_data,string_index}   | 0 | Binary test files, unreviewed
| src/array.rs                          | +1 | Removed `#[inline]`
| src/array_string.rs                   | +1 | Added `fn len`, removed `#[inline]`, use ptr instead of slice
| src/chars.rs                          | +1 | `encode_utf8` is now sadly unsafe, more test coverage
| src/lib.rs                            | +1 | Inline tweaks, more (correct) ptr use, add `as_*_ptr` to match Vec (safe/sound)

0.5.0
=====

| Diff                          | Rating | Notes |
| ----------------------------- | ------ | ----- |
| .cargo_vs_info.json           | +1 | |
| .travis.yml                   | +1 | MSRV bumped to 1.36.0, features tweaked.
| Cargo.toml                    | +1 | feature "serde-1" -> "serde", 2018 edition, drop cruft
| Cargo.toml.orig               | +1 | |
| README.rst                    | 0  | "(not yet released)" no longer accurate.
| benches/extend.rs             | +1 | +black_box
| build.rs                      | +1 | Dropped?
| src/array.rs                  | +1 | Improved safety docs, although could use more explaination of what relies on the invariants.  () and bool indexing for 1/2-len arrays.
| src/array_string.rs           | 0  | mem::zeroed -> MaybeUninitCopy::uninitialized.  Lots of Copy constraints, one transmute -> from_utf8_unchecked_mut (safer).
| src/lib.rs                    | 0  | truncate now unsafe (but sound), new try_extend_from_slice is unsafe (but sound).  ArrayVec::extend ZST handling is obtuse, would be unsound in C++, but I believe sound in Rust, maybe?
| src/maybe_uninit.rs           | +1 | |
| src/maybe_uninit_nodrop.rs    | +1 | Removed
| src/maybe_uninit_stable.rs    | +1 | Removed
| src/range.rs                  | +1 | Removed
| tests/serde.rs                | +1 | |
| tests/tests.rs                | +1 | New test cases

0.4.11
======
* ArrayVec should now also be sound in Rust 1.36.0+, probably, maybe.

0.4.10
======
* #[repr(C)]
* -Clone for MaybeUninit
* Apparently I missed more possible unsoundness.
* Unsafe is hard.

0.4.9
======
* ArrayString initialized to 0
* ArrayVec uses nightly MaybeUninit.
* Unfortunately stable still uses uninitialized!() so this is still negative.

0.4.8
=====
* IntoIter implemented Clone, unconcerning

0.4.7
=====

Prefer 0.4.11 which at least starts using MaybeUninit instead of uninitialized!(), which is fundamentally unsound.
Uses a disturbing amount of unsafe, but aside from uninitialized, it all at least appears to be correct after a careful reading.
Unlike smallvec, this doesn't fall back on the heap.
Better than whatever you'll write rolling your own, at least.

| File                      | Rating | Notes |
| ------------------------- | ------ | ----- |
| benches/arraystring.rs    | +1    | |
| benches/extend.rs         | +1    | |
| src/array_string.rs       | 0     | lots of unsafe, but I think sound
| src/array.rs              | 0     | fix_array_impl! hides unsafe, but not misused nor public
| src/char.rs               | +1    | Relied upon for soundness... thoroughly checked against https://en.wikipedia.org/wiki/UTF-8
| src/errors.rs             | +1    | |
| src/lib.rs                | 0     | lots of unsafe, but I think sound
| src/range.rs              | +1    | |
| tests/serde.rs            | +1    | |
| tests/tests.rs            | +1    | |
| .gitignore                | +1    | |
| .travis.yml               | +1    | |
| Cargo.toml                | +1    | |
| Cargo.toml.orig           | +1    | |
| custom.css                | +1    | |
| LICENSE-APACHE            | +1    | |
| LICENSE-MIT               | +1    | |
| README.rst                | +1    | |


| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1    | Overused
| fs        | +1    | Unused
| docs      | +1    | |
| tests     | 0     | Good coverage... not seeing any fuzz testing for all this unsafe though.



src/array_string.rs 
-------------------

### OK

| Line  | Notes |
| -----:| ----- |
| 56    | unsafe - new_array ~ uninitialized, Array is an unsafe trait though so only implement it if this is sound...?
| 95    | No CapacityError?  Inconsistent vs from...
| 160   | unsafe { ... } - looks correct
| 213   | unsafe { ... } - looks correct
| 216   | could be a slice copy instead
| 245   | unsafe { ... } - looks correct
| 271   | unsafe { ... } - looks correct
| 307   | unsafe { ... } - looks correct
| 318   | unsafe { ... } - looks correct
| 331   | unsafe fn - decent docs, looks correct, should be more explicit about uninitialized though
| 342   | unsafe fn - needs better docs, but looks correct
| 351   | unsafe { ... } - looks correct
| 361   | unsafe { ... } - scary transmute, but just from &mut [u8] to &mut str.  stdlib from_utf8_unchecked does equivalent pointer casts

src/array.rs 
-------------------

| Line  | Notes     |
| -----:| --------- |
| 80    | Aieee!    |
| 132   | unsafe { ... } - not sure this is sound for bools etc.
| 214   | unsafe { ... } - looks correct
| 246   | unsafe fn - exactly as spceified
| 306   | unsafe { ... } - looks correct
| 340   | unsafe { ... } - looks correct
| 511   | unsafe fn - exactly as specified
| 552   | unsafe { ... } - scary as heck... but Drain should keep self borrowed long enough, at least.
| 575   | unsafe { ... } - looks correct
| 604   | unsafe { ... } - looks correct
| 614   | unsafe { ... } - looks correct
| 707   | unsafe { ... } - looks correct.  Size could be reduced, relies on IntoIter's custom drop not dropping copied elements due to the index increment to avoid double drops.
| 724   | unsafe { ... } - looks correct.  Size could be reduced, relies on IntoIter's custom drop not dropping copied elements due to the length decrement to avoid double drops.
| 740   | unsafe { ... } - looks correct.  Implements the aforementioned IntoIter custom drop.
| 764   | unsafe Sync - I believe this is OK.
| 765   | unsafe Send - I believe this is OK.
| 775   | unsafe { ... } - looks correct.  Relies on set_len already being truncated to avoid double drops.
| 793   | unsafe { ... } - looks correct.  Relies on set_len already being truncated to avoid double drops.
| 809   | necessary to aovid memory leaks
| 812   | unsafe { ... } - looks correct.
| 851   | unsafe { ... } - looks correct.
| 1008  | unsafe { ... } - looks correct.
