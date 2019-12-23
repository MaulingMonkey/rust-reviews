# MaulingMonkey's Rust Reviews

This repository serves a few purpouses:
* To provide a quick overview and human readable versions of all [my cargo crev reviews](https://github.com/MaulingMonkey/crev-proofs).
* To provide a repository that [Dependabot](https://dependabot.com) can create issues against, to remind me to update my crev proofs.
* To provide a visual fallback via deps.rs:  [![deps.rs](https://deps.rs/repo/github/MaulingMonkey/rust-reviews/status.svg)](https://deps.rs/repo/github/MaulingMonkey/rust-reviews)

## General Utility

[audio-rodio]:      https://img.shields.io/badge/🔊-rodio-green

[crev-positive]:    https://img.shields.io/badge/crev-✓-green
[crev-neutral]:     https://img.shields.io/badge/crev-%3D-lightgrey
[crev-negative]:    https://img.shields.io/badge/crev-✗-yellow
[crev-dangerous]:   https://img.shields.io/badge/crev-✗-red

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [actix-web](https://crates.io/crates/actix-web) | [![crev-dangerous]](reviews/actix-web.md) | I don't trust this project's attitude towards unsafe for something web facing. https://64.github.io/actix/
| [amethyst](https://crates.io/crates/amethyst) | | Heavyweight data driven game engine.  Seems popular.
| [arrayvec](https://crates.io/crates/arrayvec) | [![crev-neutral]](reviews/arrayvec.md) | Vec clone (Fixed capacity, no heap). Prefer Vec?
| [ascii](https://crates.io/crates/ascii) | [![crev-negative]](reviews/ascii.md) | ASCII conversion and parsing.
| [autocfg](https://crates.io/crates/autocfg) | [![crev-positive]](reviews/autocfg.md) | Runs `rustc` to test for features / versions.
| [bindgen](https://crates.io/crates/bindgen) | | Generate Rust bindings from C/C++ headers
| [cargo-apk](https://crates.io/crates/cargo-apk) | [![crev-negative]](reviews/cargo-apk.md) | Glue code is full of unsafe and unsound.
| [cargo-dinghy](https://crates.io/crates/cargo-dinghy) | [![crev-positive]](reviews/cargo-dinghy.md) | `cargo` subcommand for building Android/iOS
| [cargo-edit](https://crates.io/crates/cargo-edit) | [![crev-neutral]](reviews/cargo-edit.md) | Add/remove/update Cargo.toml dependencies from the command line.
| [cargo-ndk](https://crates.io/crates/cargo-ndk) | [![crev-positive]](reviews/cargo-ndk.md) | Kinda trivial `.apk` building.
| [cargo_metadata](https://crates.io/crates/cargo_metadata) | [![crev-positive]](reviews/cargo_metadata.md) | Parse `cargo metadata` and `cargo build --message-format=json` output.
| [cbindgen](https://crates.io/crates/cbindgen) | | Generate C/C++ headers for Rust code
| [cfg-if](https://crates.io/crates/cfg-if) | [![crev-positive]](reviews/cfg-if.md) | Parse `cargo metadata` and `cargo build --message-format=json` output.
| [com_impl](https://crates.io/crates/com_impl) | | COM interop utilities.
| [crossterm](https://crates.io/crates/crossterm) | [![crev-negative]](reviews/crossterm.md) | Cross-platform console stuff.  No web support, soundness issues.
| [crossterm_cursor](https://crates.io/crates/crossterm_cursor) | [![crev-negative]](reviews/crossterm_cursor.md) | Cross-platform console cursor maniulation. Needs soundness fixes.
| [crossterm_input](https://crates.io/crates/crossterm_input) | [![crev-negative]](reviews/crossterm_input.md) | Cross-platform console input reading. Needs soundness fixes.
| [crossterm_screen](https://crates.io/crates/crossterm_screen) | [![crev-positive]](reviews/crossterm_screen.md) | 
| [crossterm_style](https://crates.io/crates/crossterm_style) | [![crev-negative]](reviews/crossterm_style.md) | 
| [dinghy-build](https://crates.io/crates/dinghy-build) | [![crev-positive]](reviews/dinghy-build.md) | 
| [dinghy-lib](https://crates.io/crates/dinghy-lib) | [![crev-neutral]](reviews/dinghy-lib.md) | 
| [dlopen](https://crates.io/crates/dlopen) | | Safeish and unsafe APIs for loading `.so`s, `.dll`s at runtime.
| [ggez](https://crates.io/crates/ggez) | | 
| [gimli](https://crates.io/crates/gimli) | [![crev-positive]](reviews/gimli.md) | DWARF debug info parsing.
| [glsl-include](https://crates.io/crates/glsl-include) | [![crev-positive]](reviews/glsl-include.md) | Handle basic `#include`s for GLSL.
| [idna](https://crates.io/crates/idna) | [![crev-positive]](reviews/idna.md) | Encoding/decoding domain names/punycode.
| [instant](https://crates.io/crates/instant) | [![crev-positive]](reviews/instant.md) | std::time::Instant alternative that doesn't panic on wasm targets.
| [inventory](https://crates.io/crates/inventory) | | Decentralize the registration of static data
| [jni-sys](https://crates.io/crates/jni-sys) | [![crev-positive]](reviews/jni-sys.md) | Rust bindings for JNI interop.
| [jni](https://crates.io/crates/jni) | [![crev-negative]](reviews/jni.md) | Unsafe and unsound. Has responded to fixes well though.
| [js-sys](https://crates.io/crates/js-sys) | | Browser API interop
| [lazy_static](https://crates.io/crates/lazy_static) | [![crev-positive]](reviews/lazy_static.md) | Static init at runtime.
| [legion](https://crates.io/crates/legion) | [![crev-negative]](reviews/legion.md) | A low-boilerplate, high performance archetype based ECS. Lots of unsafe, possibly unsound, overflow concerns, etc.
| [libc](https://crates.io/crates/libc) | | POSIX / C APIs megacrate.  You use this.
| [libloading](https://crates.io/crates/libloading) | | Unsafe APIs for loading `.so`s, `.dll`s at runtime.
| [macro_rules_attribute](https://crates.io/crates/macro_rules_attribute) | [![crev-positive]](reviews/macro_rules_attribute.md) | Provides a #[derive(...)]-like attribute without needing your own proc macro crate.
| [memalloc](https://crates.io/crates/memalloc) | [![crev-dangerous]](reviews/memalloc.md) | Super brittle/dangerous at a fundamental level. Avoid.
| [midir](https://crates.io/crates/midir) | [![crev-negative]](reviews/midir.md) | Pure rust MIDI device I/O. Good start, but probably unsound.
| [piston](https://crates.io/crates/piston) | | 
| [quicksilver](https://crates.io/crates/quicksilver) | | Lightweight engine targeting Desktop & Browser
| [require_unsafe_in_body](https://crates.io/crates/require_unsafe_in_body) | [![crev-positive]](reviews/require_unsafe_in_body.md) | Reducing the scope of `unsafe { ... }` in `unsafe fn`s.
| [rocket](https://crates.io/crates/rocket) | | Rust library for creating API servers.  Nice design.
| [rust-ffi](https://crates.io/crates/rust-ffi) | | Generate C/C++ headers for Rust code
| [serde](https://crates.io/crates/serde) | | The crate used for serialization throughout the Rust ecosystem
| [serde_json](https://crates.io/crates/serde_json) | | serde companion crate for (de)serializing `.json` files.
| [shellexpand](https://crates.io/crates/shellexpand) | [![crev-positive]](reviews/shellexpand.md) | Expand unix style env vars within strings.
| [smallvec](https://crates.io/crates/smallvec) | [![crev-negative]](reviews/smallvec.md) | Vec clone (Small Buffer Optimization, Heap Fallback). Prefer Vec.
| [specs](https://crates.io/crates/specs) | [![crev-neutral]](reviews/specs.md) | High boilerplate ECS.  Fancy and parallel though.
| [stdweb](https://crates.io/crates/stdweb) | | Browser API interop
| [tempfile](https://crates.io/crates/tempfile) | [![crev-positive]](reviews/tempfile.md) | Create/cleanup temporary files and directories.
| [tiled-json-rs](https://crates.io/crates/tiled-json-rs) | [![crev-negative]](reviews/tiled-json-rs.md) | [Tiled](https://www.mapeditor.org/) `.json` export file parser. Decent bones, but I'm concerned about path traversal attacks.
| [tiled](https://crates.io/crates/tiled) | [![crev-negative]](reviews/tiled.md) | [Tiled](https://www.mapeditor.org/) `.tmx` file parser. Decent bones, but I'm concerned about path traversal attacks.
| [typetag](https://crates.io/crates/typetag) | | Deserialize `Box<dyn Trait>` based on `inventory` registrations.
| [void](https://crates.io/crates/void) | [![crev-positive]](reviews/void.md) | Uninhabited type.
| [warmy](https://crates.io/crates/warmy) | [![crev-positive]](reviews/warmy.md) | Hot reloading resources. Not browser friendly.
| [wasmparser](https://crates.io/crates/wasmparser) | [![crev-positive]](reviews/wasmparser.md) | `.wasm` file parser
| [web-sys](https://crates.io/crates/web-sys) | | Browser API interop
| [winapi](https://crates.io/crates/winapi) | | Win32 / Windows APIs megacrate.  You use this.
| [winrt](https://crates.io/crates/winrt) | | C++/CX APIs megacrate.
| [xml-rs](https://crates.io/crates/xml-rs) | [![crev-positive]](reviews/xml-rs.md) | Encoding and decoding XML. Safe, sound, no deps.
| [zip](https://crates.io/crates/zip) | [![crev-positive]](reviews/zip.md) | Zipping/unzipping .zip archives.


# Procedures

## Adding a new review

* Add `cratename = "=0.0.1" to Cargo.toml
* Review via crev
    ```cmd
    cargo update
    cargo crev query review  cratename
    cargo crev open          cratename
    cargo crev review        cratename
    cargo crev push
    ```
* Create equivalent notes in new reviews/cratename.md
* Link in README.md
* git commit
    ```
    Add cratename review for v0.0.1
    ```
* git push

## Updating a review

* Bump version once in Cargo.toml
* Review via crev
    ```cmd
    cargo update
    cargo crev query review  cratename
    cargo crev open          cratename
    cargo crev diff          cratename --src=0.0.1 --dst=0.0.2  -u --color
    cargo crev review        cratename --diff
    cargo crev push
    ```
* Append equivalent notes to reviews/cratename.md
* Bump version in README.md
* git commit
    ```
    Update cratename review to v0.0.2

    Any extra notes

    Closes MaulingMonkey/rust-reviews#000
    ```
* git push
