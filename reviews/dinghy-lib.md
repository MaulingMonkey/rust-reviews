---
category:       Android
description:    
---

# dinghy-lib

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
|  0.4.16           | low | low | neutral | Trivial version bumps, thread -> std::thread.
|  0.4.15           | low | low | neutral | Trivial version bumps.
|  0.4.14           | low | low | neutral | Trivial version bumps, lots of pointless style changes, a few warning fixes (missing `dyn`s etc.)
|  0.4.13           | low | low | neutral | Trivial version bumps.
|  0.4.12           | low | low | neutral | Trivial version bumps.
| [0.4.11](#0.4.11) | low | low | neutral | Full review

# 0.4.11

Concerns:
- Looks like lots of stuff might not work on windows... although there *is* windows-specific code, so maybe?
- Lots of unimplemented!()
- iOS support is chock full of unsafe { ... } for FFI.  I haven't verified the FFI signatures.
- iOS support also uses unsafe { ... } for several objective C casts.  Needs some sanity checked utility functions.
- iOS support has some potential UB during panics due to unwinding over FFI boundaries.
- Implements some code signing stuff for iOS
- Few unit tests visible in the crate itself (maybe they're separate and unpackaged?)
- Sandboxing concerns
    - Remotes into other devices, including over ssh.
    - Frequent use of shell commands could lead to build server RCEs given malicious project metadata
    - Malicious projects will just use a build.rs file though, front door is open so to speak.



Details
-------

- src/android/device.rs           +1
    - 14  Odd place to install...
- src/android/mod.rs              +1
    - 155 Could also check android studio SDK install path
- src/host/device.rs              +1
- src/host/mod.rs                 +1
- src/host/platform.rs            +1
- src/ios/device.rs               0
    - 50  Not sure if the underlying iOS APIs are thread safe, but this seems acceptable.
    - 277 unsafe { ... } for FFI.  Looks generally safe except for scary mem::transmute(kCFBooleanTrue), but even that may be right.
    - 303 unsafe { ... } for FFI.  Looks safe to me.
    - 367 unsafe { ... } for FFI.  Looks safe to me, but needlessly large.
    - 419 unsafe { ... } for FFI.  Looks safe to me.  
    - 440 unsafe { ... } for FFI.  Looks safe to me.
    - 454 unsafe { ... } for FFI.  Looks safe to me.
    - 487 unsafe { ... } for FFI.  Looks safe to me, but needlessly large.
    - 519 This thread just eats errors.
    - 683 unsafe { ... } for FFI.  Transmute... probably safe.
    - 690 unsafe { ... } for FFI.  Looks safe to me.
    - 691 unsafe { ... } for FFI.  Scary Core Foundation related transmutes... probably OK, but some utilities to sanity check these conversions in debug would be nice.
    - 696 unsafe { ... } for FFI.  Another scary-but-probably-safe transmute.
- src/ios/helpers.py              +1
- src/ios/mobiledevice_sys.rs     0
    - FFI, not perfectly verified since I don't have an OS X machine to check the headers out on.
    - Ref: https://github.com/PanayotCankov/device.io/blob/master/idb/MobileDevice.h
    - Various minor const differences, a few functions missing in ref, a few likely improved definitions in places.
    - 33  am_device_notification_callback_info has    "extra" field vs reference, "subscription".  iOS internal struct?
- src/ios/mod.rs                  0
    - 43  unsafe { ... } for FFI.  Looks safe to me.
    - 55  technically unsound inner fn (uses ptrs)
    - 61  Scary looking as hell, but I think this is just going Box (40) -> void* (48) -> Box (here).
    - 62  UNDEFINED BEHAVIOR:  Possible panic unwind through FFI, technically an hazard.
        I would've missed this edge case but for the comment, so I'm not docking points.
        Unlikely to cause severe problems, but would be worth fixing.
- src/ios/platform.rs             +1
- src/ios/xcode.rs                +1
    - 25  Dead code not reviewed
    - 91  I have not thoroughly audited this code signing stuff, but looks OK.
    - 199 com.zoy.kali.Dinghy?  A bit hardcoded...
- src/platform/mod.rs                 +1
- src/platform/regular_platform.rs    +1
    - 44  Since when does "regular" mean "*nix" - might not work on windows.
- src/script/device.rs            +1
- src/script/mod.rs               +1
- src/ssh/device.rs               0
- src/ssh/mod.rs                  +1
- src/compiler.rs                 +1
    - 475 Isn't this *unbanned*?
- src/config.rs                   +1
- src/device.rs                   +1
- src/errors.rs                   +1
- src/lib.rs                      +1
    - 87  random sleep? why?
- src/overlay.rs                  +1
- src/project.rs                  +1
- src/toolchain.rs                +1
- src/utils.rs                    +1
- build.rs                        +1
- Cargo.toml                      +1
- Cargo.toml.orig                 +1



TIL
---
Neat loop pattern:

```rust
for (a,     b,      c) in &[
    ("a",   "b",    "c"),
    ("aa",  "bb",   "cc"),
] {
    ...
}
```

Sysroot paths:  ndk/toolchains/llvm/prebuilt/sysroot/usr/lib/{binutils_cpu}-linux-{abi_kind}"
