---
category:       Android
description:    Rust bindings for JNI interop.
---

# jni-sys

Good solid FFI crate.  Manual generation (I see comments!) concerns me, but upon review it looks to have been done correctly.

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.3.0](#0.3.0) | medium | high | positive

# 0.3.0

Verified against %LOCALAPPDATA%\Android\Sdk\ndk-bundle\toolchains\llvm\prebuilt\windows-x86_64\sysroot\usr\include\jni.h

Alternative ref: https://github.com/mapbox/jni.hpp/blob/master/test/openjdk/jni.h - this version has JNICALLs in JNINativeInterface which hint at the use of extern "system"

Detail
------

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| src/lib.rs                        |
| .gitignore                        | +1
| .travis.yml                       | +1
| appveyor.yml                      | +1
| Cargo.toml                        | +1
| Cargo.toml.orig                   | +1
| README.md                         | +1

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | Lots of it... but all necessary, and after careful review, all appears correct.
| fs        | +1    | None
| io        | +1    | None
| docs      | -1    | Nonexistant
| tests     |  0    | Not in crate

### src/lib.rs

| Line  | Rating | Notes |
| -----:| ------ | ----- |
| 7     | +1    | Looks sufficiently correct to me - VS defines this as *mut c_char, but *mut c_void is close enough IMO.
| 10    | +1    | all verified
| 20    | 0     | native version uses more newtypes, but without inheritence maybe this is sane in Rust.
| 39    | +1    | verified
| 51    | +1    | sure
| 57    | +1    | |
| 64    | -1    | DANGER - rust enum for C enum is begging for unsound if returned from FFI
| 71    | +1    | JNI constants here all look good
| 93    | 0     | JNINativeMethod - C version uses const char*, but the muts here aren't a *big* deal... I
| 99    | +1    | |
| 105   | +1    | JNINativeInterface is bloody huge, but all methods appear to match in name, position, and signature... |
| 115   |       | ...with the caveat that there's a lot of "system" convention when JNICALL isn't used throughout the android jni.h.  However, it *is* used in openjdk so that's probably fine?
| 116   |       | I'm also assuming `Option<unsafe extern "system" fn(...) -> ...>` is FFI compatible with C function pointers, which might be a bad assumption. |
| 157   |       | Native version isn't marked no return but I bet it is.
| 175   |       | Also, varargs functions degrade to "C" mode per https://stackoverflow.com/a/3615407
| 1411  |       | Reviewed all the way to here!
| 1413  | +1    | Sure
| 1421  | +1    | JNIEnv_ - Lack of impl with forwarding fns like jni.h has is vageuly annoying.
| 1425  | +1    | Sure
| 1433  | +1    | JavaVMOption - Again, mut differences, but otherwise OK.  Sure.
| 1438  | +1    | Sure
| 1446  | +1    | JavaVMInitArgs - LGTM
| 1453  | +1    | Sure
| 1461  | +1    | JavaVMAttachArgs - Again, mut differences, but otherwise OK.  Sure.
| 1467  | +1    | Sure
| 1475  | +1    | JNIInvokeInterface - LGTM
| 1501  | +1    | Sure
| 1507  | +1    | LHTM
