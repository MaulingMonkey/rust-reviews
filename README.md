# MaulingMonkey's Rust Reviews

This repository serves a few purpouses:
* To provide a quick overview and human readable versions of all [my cargo crev reviews](https://github.com/MaulingMonkey/crev-proofs).
* To provide a repository that [Dependabot](https://dependabot.com) can create issues against, to remind me to update my crev proofs.
* To provide a visual fallback via deps.rs:  [![deps.rs](https://deps.rs/repo/github/MaulingMonkey/rust-reviews/status.svg)](https://deps.rs/repo/github/MaulingMonkey/rust-reviews)

| crate                                                                     | reviewed  | rating | description |
| ------------------------------------------------------------------------- | --------- | ------ | ----------- |
| **General Utility** | | | |
| [arrayvec](https://crates.io/crates/arrayvec)                             | 0.4.11    | ![neutral](https://img.shields.io/badge/-neutral-lightgrey)   | Vec clone (Fixed capacity, no heap).  Prefer Vec?
| [smallvec](https://crates.io/crates/smallvec)                             | 0.6.10    | ![negative](https://img.shields.io/badge/-negative-yellow)    | Vec clone (Small Buffer Optimization, Heap Fallback).  Prefer Vec.
| [autocfg](https://crates.io/crates/autocfg)                               | 0.1.6     | ![positive](https://img.shields.io/badge/-positive-green)     | `build.rs` script utility for auto-detecting compiler version/feature support.
| [cfg-if](https://crates.io/crates/cfg-if)                                 | 0.1.9     | ![positive](https://img.shields.io/badge/-positive-green)     | Utility for managing cfg attribute soup.
| [lazy_static](https://crates.io/crates/lazy_static)                       | 1.4.0     | ![positive](https://img.shields.io/badge/-positive-green)     | Static init at runtime.
| [idna](https://crates.io/crates/idna)                                     | 0.2.0     | ![positive](https://img.shields.io/badge/-positive-green)     | Encoding/decoding domain names/punycode.
| [require_unsafe_in_body](https://crates.io/crates/require_unsafe_in_body) | 0.2.0     | ![positive](https://img.shields.io/badge/-positive-green)     | Reducing the scope of `unsafe { ... }` in `unsafe fn`s.
| [tempfile](https://crates.io/crates/tempfile)                             | 3.1.0     | ![positive](https://img.shields.io/badge/-positive-green)     | Create/cleanup temporary files and directories.
| [void](https://crates.io/crates/void)                                     | 1.0.2     | ![positive](https://img.shields.io/badge/-positive-green)     | Uninhabited type.
| [xml-rs](https://crates.io/crates/xml-rs)                                 | [0.8.0](reviews/xml-rs.md#0.8.0)                                  | ![positive](https://img.shields.io/badge/-positive-green)     | Encoding and decoding XML.  Safe, sound, no deps.
| [zip](https://crates.io/crates/zip)                                       | [0.5.2](reviews/zip.md#0.5.2) ... [0.5.3](reviews/zip.md#0.5.3)   | ![positive](https://img.shields.io/badge/-positive-green)     | Zipping/unzipping `.zip` archives.
| **Gamedev** | | | |
| [tiled](https://crates.io/crates/tiled)                                   | 0.8.0     | ![negative](https://img.shields.io/badge/-negative-yellow)    | [Tiled](https://www.mapeditor.org) `.tmx` file parser.  Decent bones, but I'm concerned about path traversal attacks.
| [tiled-json-rs](https://crates.io/crates/tiled-json-rs)                   | 0.2.6     | ![negative](https://img.shields.io/badge/-negative-yellow)    | [Tiled](https://www.mapeditor.org) `.json` export file parser.  Decent bones, but I'm concerned about path traversal attacks.
| [midir](https://crates.io/crates/midir)                                   | 0.5.0     | ![negative](https://img.shields.io/badge/-negative-yellow)    | Pure rust MIDI device I/O.  Good start, but probably unsound.
| [warmy](https://crates.io/crates/warmy)                                   | 0.13.0    | ![positive](https://img.shields.io/badge/-positive-green)     | Hot reloading resources.  Not browser friendly.
| **Android** | | | |
| [cargo-ndk](https://crates.io/crates/cargo-ndk)                           | 0.3.0     | ![positive](https://img.shields.io/badge/-positive-green)     | Kinda trivial apk building.
| [cargo-dinghy](https://crates.io/crates/cargo-dinghy)                     | 0.4.12    | ![positive](https://img.shields.io/badge/-positive-green)     | `cargo` subcommand for building Android/iOS
| [dinghy-build](https://crates.io/crates/dinghy-build)                     | 0.4.12    | ![positive](https://img.shields.io/badge/-positive-green)     | |
| [dinghy-lib](https://crates.io/crates/dinghy-lib)                         | 0.4.12    | ![neutral](https://img.shields.io/badge/-neutral-lightgrey)   | |
| [jni-sys](https://crates.io/crates/jni-sys)                               | 0.3.0     | ![positive](https://img.shields.io/badge/-positive-green)     | Rust bindings for JNI interop.
| **Debug** | | | |
| [gimli](https://crates.io/crates/gimli)                                   | 0.15.0    | ![positive](https://img.shields.io/badge/-positive-green)     | DWARF debug info parsing.
| **Unsound** | | | |
| [ascii](https://crates.io/crates/ascii)                                   | 0.9.2     | ![negative](https://img.shields.io/badge/-negative-yellow)    | Super full of unsafe and unsound.  Has responded to fixes well though.
| [cargo-apk](https://crates.io/crates/cargo-apk)                           | 0.4.0     | ![negative](https://img.shields.io/badge/-negative-yellow)    | Glue code is full of unsafe and unsound.
| [jni](https://crates.io/crates/jni)                                       | 0.13.0    | ![negative](https://img.shields.io/badge/-negative-yellow)    | Unsafe and unsound.  Has responded to fixes well though.
| **Avoid** | | | |
| actix-\*                                                                  | \*        | ![negative](https://img.shields.io/badge/-negative-red)       | I don't trust this project's attitude towards unsafe for something web facing.  https://64.github.io/actix/
| [memalloc](https://crates.io/crates/memalloc)                             | 0.1.0     | ![negative](https://img.shields.io/badge/-negative-red)       | Super brittle/dangerous at a fundamental level.  Avoid.

<!--
| [CRATENAME](https://crates.io/crates/CRATENAME)                           |           | ![positive](https://img.shields.io/badge/-positive-green)     | 
| [CRATENAME](https://crates.io/crates/CRATENAME)                           |           | ![neutral](https://img.shields.io/badge/-neutral-lightgrey)   | 
| [CRATENAME](https://crates.io/crates/CRATENAME)                           |           | ![negative](https://img.shields.io/badge/-negative-yellow)    | 
| [CRATENAME](https://crates.io/crates/CRATENAME)                           |           | ![negative](https://img.shields.io/badge/-negative-red)       | 
-->
