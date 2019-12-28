# MaulingMonkey's Rust Reviews

This repository serves a few purpouses:
* To provide a quick overview and human readable versions of all [my cargo crev reviews](https://github.com/MaulingMonkey/crev-proofs).
* To provide a repository that [Dependabot](https://dependabot.com) can create issues against, to remind me to update my crev proofs.
* To provide a visual fallback via deps.rs:  [![deps.rs](https://deps.rs/repo/github/MaulingMonkey/rust-reviews/status.svg)](https://deps.rs/repo/github/MaulingMonkey/rust-reviews)

[crev-author]:      https://img.shields.io/badge/author-🐵-lightblue
[crev-none]:        https://img.shields.io/badge/crev-N/A-lightblue

[audio-rodio]:      https://img.shields.io/badge/🔊-rodio-green

[crev-positive]:    https://img.shields.io/badge/crev-✓-green
[crev-neutral]:     https://img.shields.io/badge/crev-%3D-lightgrey
[crev-negative]:    https://img.shields.io/badge/crev-✗-yellow
[crev-dangerous]:   https://img.shields.io/badge/crev-✗-red

## Android

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [cargo-dinghy](https://crates.io/crates/cargo-dinghy) | [![crev-positive]](reviews/cargo-dinghy.md) | `cargo` subcommand for building Android/iOS
| [cargo-ndk](https://crates.io/crates/cargo-ndk) | [![crev-positive]](reviews/cargo-ndk.md) | Kinda trivial `.apk` building.
| [dinghy-build](https://crates.io/crates/dinghy-build) | [![crev-positive]](reviews/dinghy-build.md) | 
| [dinghy-lib](https://crates.io/crates/dinghy-lib) | [![crev-neutral]](reviews/dinghy-lib.md) | 
| [jni-sys](https://crates.io/crates/jni-sys) | [![crev-positive]](reviews/jni-sys.md) | Rust bindings for JNI interop.
| [jni](https://crates.io/crates/jni) | [![crev-negative]](reviews/jni.md) | Unsafe and unsound. Has responded to fixes well though.

## Build Utility

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [autocfg](https://crates.io/crates/autocfg) | [![crev-positive]](reviews/autocfg.md) | Runs `rustc` to test for features / versions.
| [cargo_metadata](https://crates.io/crates/cargo_metadata) | [![crev-positive]](reviews/cargo_metadata.md) | Parse `cargo metadata` and `cargo build --message-format=json` output.
| [cfg-if](https://crates.io/crates/cfg-if) | [![crev-positive]](reviews/cfg-if.md) | Parse `cargo metadata` and `cargo build --message-format=json` output.
| [rustversion](https://crates.io/crates/rustversion) | [![crev-positive]](reviews/rustversion.md) | Attributes to do conditional compilation based on rust version/channel

## CLI Tools

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [cargo-edit](https://crates.io/crates/cargo-edit) | [![crev-neutral]](reviews/cargo-edit.md) | Add/remove/update Cargo.toml dependencies from the command line.

## Debugging

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [gimli](https://crates.io/crates/gimli) | [![crev-positive]](reviews/gimli.md) | DWARF debug info parsing.
| [wasm-dwarf](https://crates.io/crates/wasm-dwarf) | [![crev-none]](reviews/wasm-dwarf.md) | WASM Dwarf reader / .map generator
| [wasmparser](https://crates.io/crates/wasmparser) | [![crev-positive]](reviews/wasmparser.md) | `.wasm` file parser

## FFI

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [bindgen](https://crates.io/crates/bindgen) | [![crev-none]](reviews/bindgen.md) | Generate Rust bindings from C/C++ headers
| [cbindgen](https://crates.io/crates/cbindgen) | [![crev-none]](reviews/cbindgen.md) | Generate C/C++ headers for Rust code
| [com_impl](https://crates.io/crates/com_impl) | [![crev-none]](reviews/com_impl.md) | COM interop utilities.
| [foreign-types](https://crates.io/crates/foreign-types) | [![crev-neutral]](reviews/foreign-types.md) | Generate Rust wrappers around C types
| [libc](https://crates.io/crates/libc) | [![crev-none]](reviews/libc.md) | POSIX / C APIs megacrate.  You use this.
| [rust-ffi](https://crates.io/crates/rust-ffi) | [![crev-none]](reviews/rust-ffi.md) | Generate C/C++ headers for Rust code
| [winapi](https://crates.io/crates/winapi) | [![crev-none]](reviews/winapi.md) | Win32 / Windows APIs megacrate.  You use this.
| [winrt](https://crates.io/crates/winrt) | [![crev-none]](reviews/winrt.md) | C++/CX APIs megacrate.

## Game Engine

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [amethyst](https://crates.io/crates/amethyst) | [![crev-none]](reviews/amethyst.md) | Heavyweight data driven game engine.  Seems popular.
| [ggez](https://crates.io/crates/ggez) | [![crev-none]](reviews/ggez.md) | 
| [piston](https://crates.io/crates/piston) | [![crev-none]](reviews/piston.md) | 
| [quicksilver](https://crates.io/crates/quicksilver) | [![crev-none]](reviews/quicksilver.md) | Lightweight engine targeting Desktop & Browser

## Gamedev

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [glsl-include](https://crates.io/crates/glsl-include) | [![crev-positive]](reviews/glsl-include.md) | Handle basic `#include`s for GLSL.
| [legion](https://crates.io/crates/legion) | [![crev-negative]](reviews/legion.md) | A low-boilerplate, high performance archetype based ECS. Lots of unsafe, possibly unsound, overflow concerns, etc.
| [midir](https://crates.io/crates/midir) | [![crev-negative]](reviews/midir.md) | Pure rust MIDI device I/O. Good start, but probably unsound.
| [specs](https://crates.io/crates/specs) | [![crev-neutral]](reviews/specs.md) | High boilerplate ECS.  Fancy and parallel though.
| [tiled-json-rs](https://crates.io/crates/tiled-json-rs) | [![crev-negative]](reviews/tiled-json-rs.md) | [Tiled](https://www.mapeditor.org/) `.json` export file parser. Decent bones, but I'm concerned about path traversal attacks.
| [tiled](https://crates.io/crates/tiled) | [![crev-negative]](reviews/tiled.md) | [Tiled](https://www.mapeditor.org/) `.tmx` file parser. Decent bones, but I'm concerned about path traversal attacks.
| [warmy](https://crates.io/crates/warmy) | [![crev-positive]](reviews/warmy.md) | Hot reloading resources. Not browser friendly.

## General Utility

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [arrayvec](https://crates.io/crates/arrayvec) | [![crev-neutral]](reviews/arrayvec.md) | Vec clone (Fixed capacity, no heap). Prefer Vec?
| [dlopen](https://crates.io/crates/dlopen) | [![crev-none]](reviews/dlopen.md) | Safeish and unsafe APIs for loading `.so`s, `.dll`s at runtime.
| [inventory](https://crates.io/crates/inventory) | [![crev-positive]](reviews/inventory.md) | Decentralized static registration
| [lazy_static](https://crates.io/crates/lazy_static) | [![crev-positive]](reviews/lazy_static.md) | Static init at runtime.
| [libloading](https://crates.io/crates/libloading) | [![crev-none]](reviews/libloading.md) | Unsafe APIs for loading `.so`s, `.dll`s at runtime.
| [macro_rules_attribute](https://crates.io/crates/macro_rules_attribute) | [![crev-positive]](reviews/macro_rules_attribute.md) | Provides a #[derive(...)]-like attribute without needing your own proc macro crate.
| [require_unsafe_in_body](https://crates.io/crates/require_unsafe_in_body) | [![crev-positive]](reviews/require_unsafe_in_body.md) | Reducing the scope of `unsafe { ... }` in `unsafe fn`s.
| [shellexpand](https://crates.io/crates/shellexpand) | [![crev-positive]](reviews/shellexpand.md) | Expand unix style env vars within strings.
| [smallvec](https://crates.io/crates/smallvec) | [![crev-negative]](reviews/smallvec.md) | Vec clone (Small Buffer Optimization, Heap Fallback). Prefer Vec.
| [tempfile](https://crates.io/crates/tempfile) | [![crev-positive]](reviews/tempfile.md) | Create/cleanup temporary files and directories.
| [void](https://crates.io/crates/void) | [![crev-positive]](reviews/void.md) | Uninhabited type.

## Serialization

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [ascii](https://crates.io/crates/ascii) | [![crev-negative]](reviews/ascii.md) | ASCII conversion and parsing.
| [idna](https://crates.io/crates/idna) | [![crev-positive]](reviews/idna.md) | Encoding/decoding domain names/punycode.
| [serde](https://crates.io/crates/serde) | [![crev-none]](reviews/serde.md) | The crate used for serialization throughout the Rust ecosystem
| [serde_json](https://crates.io/crates/serde_json) | [![crev-none]](reviews/serde_json.md) | serde companion crate for (de)serializing `.json` files.
| [typetag](https://crates.io/crates/typetag) | [![crev-none]](reviews/typetag.md) | Deserialize `Box<dyn Trait>` based on `inventory` registrations.
| [xml-rs](https://crates.io/crates/xml-rs) | [![crev-positive]](reviews/xml-rs.md) | Encoding and decoding XML. Safe, sound, no deps.
| [zip](https://crates.io/crates/zip) | [![crev-positive]](reviews/zip.md) | Zipping/unzipping .zip archives.

## Unsound

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [actix-web](https://crates.io/crates/actix-web) | [![crev-dangerous]](reviews/actix-web.md) | I don't trust this project's attitude towards unsafe for something web facing. https://64.github.io/actix/
| [cargo-apk](https://crates.io/crates/cargo-apk) | [![crev-negative]](reviews/cargo-apk.md) | Glue code is full of unsafe and unsound.
| [crossterm](https://crates.io/crates/crossterm) | [![crev-negative]](reviews/crossterm.md) | Cross-platform console stuff.  No web support, soundness issues.
| [crossterm_cursor](https://crates.io/crates/crossterm_cursor) | [![crev-negative]](reviews/crossterm_cursor.md) | Cross-platform console cursor maniulation. Needs soundness fixes.
| [crossterm_input](https://crates.io/crates/crossterm_input) | [![crev-negative]](reviews/crossterm_input.md) | Cross-platform console input reading. Needs soundness fixes.
| [crossterm_screen](https://crates.io/crates/crossterm_screen) | [![crev-positive]](reviews/crossterm_screen.md) | 
| [crossterm_style](https://crates.io/crates/crossterm_style) | [![crev-negative]](reviews/crossterm_style.md) | 
| [memalloc](https://crates.io/crates/memalloc) | [![crev-dangerous]](reviews/memalloc.md) | Super brittle/dangerous at a fundamental level. Avoid.

## Web

| Crate | Review | Description |
| ----- | ------ | ----------- |
| [instant](https://crates.io/crates/instant) | [![crev-positive]](reviews/instant.md) | std::time::Instant alternative that doesn't panic on wasm targets.
| [js-sys](https://crates.io/crates/js-sys) | [![crev-none]](reviews/js-sys.md) | Browser API interop
| [rocket](https://crates.io/crates/rocket) | [![crev-none]](reviews/rocket.md) | Rust library for creating API servers.  Nice design.
| [stdweb](https://crates.io/crates/stdweb) | [![crev-none]](reviews/stdweb.md) | Browser API interop
| [web-sys](https://crates.io/crates/web-sys) | [![crev-none]](reviews/web-sys.md) | Browser API interop



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

## Initial Setup

* Install and configure cargo crev
    ```cmd
    cargo install cargo-crev
    :: ...?
    ```
* Add the following to your `cargo crev edit config`:
    ```yml
    ---
    version: -1
    ...
    open-cmd: "start \"\" \"C:\\Users\\UserName\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe\" -n --disable-extension rust-lang.rust --disable-extension matklad.ra-lsp --disable-extension kalitaalexey.vscode-rust"
     
    ```
    This disables these extensions, which all provide Rust intellisense, which has a tendency to pollute `%USERPROFILE%\.cargo\registry\src` with `Cargo.lock` files, `target` directories, or worse, which makes cargo crev angry:
    * rust-lang.rust
    * matklad.ra-lsp
    * kalitaalexey.vscode-rust
