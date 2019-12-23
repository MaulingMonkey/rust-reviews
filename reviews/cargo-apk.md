---
category:       Unsound
description:    Glue code is full of unsafe and unsound.
---

# cargo-apk

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.4.0](#0.4.0) | low | medium | negative |

# 0.4.0

```
A mixed bag.
On the one hand, the core tool found in `src` seems fine, solid fundamentals.
On the other hand, the glue code is full chock full of unsafe, some of it unsound.
Worse, the FFI includes things that varies from android SDK to SDK from the loops of things.



injected-glue\ffi.rs:   -1
  Should be replaced with bindgen generated header?  Also, changes between SDKs, so really really really should not be simply hardcoded...!
  Ref: https://android.googlesource.com/platform/development/+/4948c163663ecc343c97e4c2a2139234f1d3273f/ndk/sources/android/native_app_glue/android_native_app_glue.h
  Ref: https://chromium.googlesource.com/android_tools/+/7fc902d157a9aed7a2b68adc9c69181b3a43cd58/ndk/sources/android/native_app_glue/android_native_app_glue.h
  45  LOOPER_ID_INPUT:    Wrong? (should be 2?: https://chromium.googlesource.com/android_tools/+/7fc902d157a9aed7a2b68adc9c69181b3a43cd58/ndk/sources/android/native_app_glue/android_native_app_glue.h#204)
      LOOPER_ID_EVENT?    Different name? https://android.googlesource.com/platform/development/+/4948c163663ecc343c97e4c2a2139234f1d3273f/ndk/sources/android/native_app_glue/android_native_app_glue.h
  46  LOOPER_ID_USER:     Wrong? (should be 3?: https://chromium.googlesource.com/android_tools/+/7fc902d157a9aed7a2b68adc9c69181b3a43cd58/ndk/sources/android/native_app_glue/android_native_app_glue.h#209)
  65+ Skimmed only

injected-glue\lib.rs:   -1
  92  static mut ANDROID_APP  should be UnsafeCell
  164 static mut G_MAINTHREAD_BOXED should be UnsafeCell, probable undefined behavior / race conditions
  172 scary transmutes
  182 Unsound when combined with
  191 'safe' access
  196 android_main2 is UNSOUND - dereferencing pointers, setting globals, etc. without being marked unsafe.
  197+ Skimmed only
  340: giant unsafe block
  347: uninitialized



src\ops\build.rs:       +1
    75 Medium:  I suspect this won't work on windows as it doesn't use .cmd scripts for the compiler/linker?
  131 Medium:  injected_glue_lib cmd.exec()?; followed by cmd.exec_with_output()?; (134)- does the glue get double compiled for no good reason?
  218 Medium:  unreachable! abused where panic! probably should be?
  423 Minor:   Hardcodes a lot of assumptions.  Good foundation though.

src\ops\install.rs:     +1
  Medium:  No way to specify device
src\ops\mod.rs:         +1
src\ops\run.rs:         +1
  Medium:  No way to specify device

src\config.rs:          +1
  Minor:  Could use more OsString
  Minor:  Non-optional opengles_version_*
src\main.rs:            +1

Cargo.toml:             +1
Cargo.toml.orig:        +1
glue_obj.rs:            Sketchy looking casts and `main` handling... but might be right
linker.rs:              Strange parse_arguments loop style... but safe, looks sane.
```
