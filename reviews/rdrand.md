---
category:       Gamedev
description:    🎲 Get random numbers 🎲
---

# rdrand

# Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.6.0]   | medium | medium | neutral | Diff review
| [0.5.2]   | medium | medium | neutral | Diff review
| [0.4.0]   | medium | medium | negative | Full review

[0.6.0]: #060--052
[0.5.2]: #060--052
[0.4.0]: #040

# 0.6.0 / 0.5.2

Sound, although buggy AMD hardware makes implementing CryptoRng for RdRand/RdSeed a little unnerving to me ( https://github.com/nagisa/rust_rdrand/issues/12 )

## Reviewed

* benches\rdrand.rs:  +1
* benches\rdseed.rs:  +1
* benches\std.rs:     +1
* src\changelog.rs:   +1
* src\lib.rs:         0
    * All of `$gen::try_fill_bytes::imp` being `unsafe` is still concerning... but I believe it's sound now.
    * `RdRand`/`RdSeed` implement `CrytoRng`, which makes buggy AMD hardware concerning: https://github.com/nagisa/rust_rdrand/issues/12

## Concerns reduced since 0.4.0:
* https://github.com/nagisa/rust_rdrand/commit/7af432c6e315fde053d0056d1b7df893a865711a
    * unsafe blocks appear much larger than they need to be.

## Concerns fixed since 0.4.0:
* https://github.com/nagisa/rust_rdrand/commit/26a0a2f9d885fbbb8e14fa47c8a48e366cf15455
    * `mem::uninitialized()` u32s
    * `loop_rand!` uses `mem::uninitialized()` for $el:ty, easy to misuse!  Requires unsafe{} so technically sound.  Not exported.
* https://github.com/nagisa/rust_rdrand/commit/7af432c6e315fde053d0056d1b7df893a865711a
    * `$gen::try_fill_bytes`:  UNSOUND!  word and buffer reference the same data.  As both are &mut Ts, this is 100% undefined behavior.
    * `ptr::copy_nonoverlapping`: This should really use a slice copy which should be just as safe...?  But maybe missing from core?
* Verified vs https://www.amd.com/system/files/TechDocs/24594.pdf
    * `is_x86_feature_detected`: I have not verified this is correct.

# 0.4.0

`$gen::try_fill_bytes` invokes undefined behavior (overlapping `&mut u32` and `&mut [u8]`): https://github.com/nagisa/rust_rdrand/issues/13.  0.5.x removed some use of `uninitialized`.

## Reviewed

* benches\rdrand.rs:  +1
* benches\rdseed.rs:  +1
* benches\std.rs:     +1
* src\changelog.rs:   +1
* src\lib.rs:  Concerns:
    * `mem::uninitialized()` `u32`s
    * `is_x86_feature_detected`: I have not verified this is correct.
    * `loop_rand!` uses `mem::uninitialized()` for `$el:ty`, easy to misuse!  Requires `unsafe{}` so technically sound.  Not exported.
    * `$gen::try_fill_bytes`:  UNSOUND!  word and buffer reference the same data.  As both are `&mut T`s, this is 100% undefined behavior.
    * `unsafe` blocks appear much larger than they need to be.
    * `ptr::copy_nonoverlapping`: This should really use a slice copy which should be just as safe...?  But maybe missing from core?
