# MaulingMonkey's Rust Reviews

This repository serves a few purpouses:
* To provide a quick overview and human readable versions of all [my cargo crev reviews](https://github.com/MaulingMonkey/crev-proofs).
* To provide a repository that [Dependabot](https://dependabot.com) can create issues against, to remind me to update my crev proofs.
* To provide a visual fallback via deps.rs:  [![deps.rs](https://deps.rs/repo/github/MaulingMonkey/rust-reviews/status.svg)](https://deps.rs/repo/github/MaulingMonkey/rust-reviews)

[crev-author]:      https://img.shields.io/badge/-🐵-green
[crev-none]:        https://img.shields.io/badge/-%3F-lightblue

[audio-rodio]:      https://img.shields.io/badge/🔊-rodio-green

[crev-positive]:    https://img.shields.io/badge/-✓-green
[crev-neutral]:     https://img.shields.io/badge/-%3D-lightgrey
[crev-negative]:    https://img.shields.io/badge/-✗-yellow
[crev-dangerous]:   https://img.shields.io/badge/-✗-red

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Legend&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Description |
| ----------------------------- | ----------- |
| ![crev-author]    Author      | I wrote this!
| ![crev-positive]  Positive    | Seems safe/sound/possibly useful
| ![crev-neutral]   Neutral     | This crate is OK, but might have better alternatives
| ![crev-negative]  Negative    | I have serious concerns, possibly including: too much `unsafe`, `panic!`-prone, history of soundness bugs, general brittleness, or lacking critical functionality.  Might still be a good basis for cleanup / forking.
| ![crev-dangerous] Dangerous   | Unsound, vulnerable, or likely to be (now or in the future based on poor history)
| ![crev-none]      N/A         | Haven't properly reviewed the code yet

# Categories

* [Android](#android)
* [Async](#async)
* [Build Utility](#build-utility)
* [CLI Tools](#cli-tools)
* [Data Structure](#data-structure)
* [Debugging](#debugging)
* [FFI](#ffi)
* [Game Engine](#game-engine)
* [Gamedev](#gamedev)
* [General Utility](#general-utility)
* [Graphics](#graphics)
* [I/O](#i/o)
* [Macros](#macros)
* [Serialization](#serialization)
* [Unsound](#unsound)
* [Web](#web)

<h2 id="android">Android</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-positive] &nbsp; [cargo-dinghy](reviews/cargo-dinghy.md) | [docs.rs](https://docs.rs/cargo-dinghy) <!-- [lib.rs](https://lib.rs/crates/cargo-dinghy) --> | `cargo` subcommand for building Android/iOS
| ![crev-positive] &nbsp; [cargo-ndk](reviews/cargo-ndk.md) | [docs.rs](https://docs.rs/cargo-ndk) <!-- [lib.rs](https://lib.rs/crates/cargo-ndk) --> | Kinda trivial `.apk` building.
| ![crev-positive] &nbsp; [dinghy-build](reviews/dinghy-build.md) | [docs.rs](https://docs.rs/dinghy-build) <!-- [lib.rs](https://lib.rs/crates/dinghy-build) --> | 
| ![crev-neutral] &nbsp; [dinghy-lib](reviews/dinghy-lib.md) | [docs.rs](https://docs.rs/dinghy-lib) <!-- [lib.rs](https://lib.rs/crates/dinghy-lib) --> | 
| ![crev-author] &nbsp; [jerk](reviews/jerk.md) | [docs.rs](https://docs.rs/jerk) <!-- [lib.rs](https://lib.rs/crates/jerk) --> | Java path discovery and other utilities
| ![crev-author] &nbsp; [jerk-build](reviews/jerk-build.md) | [docs.rs](https://docs.rs/jerk-build) <!-- [lib.rs](https://lib.rs/crates/jerk-build) --> | Build Java alongside Rust via build.rs/metabuild scripts
| ![crev-author] &nbsp; [jerk-test](reviews/jerk-test.md) | [docs.rs](https://docs.rs/jerk-test) <!-- [lib.rs](https://lib.rs/crates/jerk-test) --> | Unit test Java built alongside Rust
| ![crev-negative] &nbsp; [jni](reviews/jni.md) | [docs.rs](https://docs.rs/jni) <!-- [lib.rs](https://lib.rs/crates/jni) --> | Unsafe and unsound. Has responded to fixes well though.
| ![crev-author] &nbsp; [jni-android-sys](reviews/jni-android-sys.md) | [docs.rs](https://docs.rs/jni-android-sys) <!-- [lib.rs](https://lib.rs/crates/jni-android-sys) --> | Bindings to Android Java APIs
| ![crev-author] &nbsp; [jni-bindgen](reviews/jni-bindgen.md) | [docs.rs](https://docs.rs/jni-bindgen) <!-- [lib.rs](https://lib.rs/crates/jni-bindgen) --> | Java API binding code generator
| ![crev-author] &nbsp; [jni-glue](reviews/jni-glue.md) | [docs.rs](https://docs.rs/jni-glue) <!-- [lib.rs](https://lib.rs/crates/jni-glue) --> | Safeish wrappers around jni-sys used by jni-bindgen bindings
| ![crev-author] &nbsp; [jni-glue-macros](reviews/jni-glue-macros.md) | [docs.rs](https://docs.rs/jni-glue-macros) <!-- [lib.rs](https://lib.rs/crates/jni-glue-macros) --> | Proc macros to implement Java APIs from Rust
| ![crev-positive] &nbsp; [jni-sys](reviews/jni-sys.md) | [docs.rs](https://docs.rs/jni-sys) <!-- [lib.rs](https://lib.rs/crates/jni-sys) --> | Rust bindings for JNI interop.

<h2 id="async">Async</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-positive] &nbsp; [futures](reviews/futures.md) | [docs.rs](https://docs.rs/futures) <!-- [lib.rs](https://lib.rs/crates/futures) --> | Asyncronous streams, sinks, executors, tasks, I/O, etc.
| ![crev-none] &nbsp; [tokio](reviews/tokio.md) | [docs.rs](https://docs.rs/tokio) <!-- [lib.rs](https://lib.rs/crates/tokio) --> | Asyncronous I/O runtime/framework
| ![crev-positive] &nbsp; [waker-fn](reviews/waker-fn.md) | [docs.rs](https://docs.rs/waker-fn) <!-- [lib.rs](https://lib.rs/crates/waker-fn) --> | Basic 0-dependencies Fn-based Waker source.
| ![crev-none] &nbsp; [wasm_bindgen_futures](reviews/wasm_bindgen_futures.md) | [docs.rs](https://docs.rs/wasm_bindgen_futures) <!-- [lib.rs](https://lib.rs/crates/wasm_bindgen_futures) --> | Convert JS `Promise`s to/from Rust `Future`s

<h2 id="build-utility">Build Utility</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-positive] &nbsp; [autocfg](reviews/autocfg.md) | [docs.rs](https://docs.rs/autocfg) <!-- [lib.rs](https://lib.rs/crates/autocfg) --> | Runs `rustc` to test for features / versions.
| ![crev-positive] &nbsp; [cargo_metadata](reviews/cargo_metadata.md) | [docs.rs](https://docs.rs/cargo_metadata) <!-- [lib.rs](https://lib.rs/crates/cargo_metadata) --> | Parse `cargo metadata` and `cargo build --message-format=json` output.
| ![crev-positive] &nbsp; [cfg-if](reviews/cfg-if.md) | [docs.rs](https://docs.rs/cfg-if) <!-- [lib.rs](https://lib.rs/crates/cfg-if) --> | Parse `cargo metadata` and `cargo build --message-format=json` output.
| ![crev-author] &nbsp; [lies](reviews/lies.md) | [docs.rs](https://docs.rs/lies) <!-- [lib.rs](https://lib.rs/crates/lies) --> | Embed license text into your program via proc macros around cargo-about.
| ![crev-author] &nbsp; [lies-impl](reviews/lies-impl.md) | [docs.rs](https://docs.rs/lies-impl) <!-- [lib.rs](https://lib.rs/crates/lies-impl) --> | 
| ![crev-positive] &nbsp; [rustversion](reviews/rustversion.md) | [docs.rs](https://docs.rs/rustversion) <!-- [lib.rs](https://lib.rs/crates/rustversion) --> | Attributes to do conditional compilation based on rust version/channel
| ![crev-positive] &nbsp; [vcpkg](reviews/vcpkg.md) | [docs.rs](https://docs.rs/vcpkg) <!-- [lib.rs](https://lib.rs/crates/vcpkg) --> | Build dependency to get C/C++ [vcpkg](https://github.com/Microsoft/vcpkg)s
| ![crev-positive] &nbsp; [winres](reviews/winres.md) | [docs.rs](https://docs.rs/winres) <!-- [lib.rs](https://lib.rs/crates/winres) --> | Embed resources (icons, versions, ...) into your executables.

<h2 id="cli-tools">CLI Tools</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-positive] &nbsp; [cargo](reviews/cargo.md) | [docs.rs](https://docs.rs/cargo) <!-- [lib.rs](https://lib.rs/crates/cargo) --> | *The* rust build tool.
| ![crev-none] &nbsp; [cargo-about](reviews/cargo-about.md) | [docs.rs](https://docs.rs/cargo-about) <!-- [lib.rs](https://lib.rs/crates/cargo-about) --> | Validate dependency licenses and aggregate into a single .html file
| ![crev-none] &nbsp; [cargo-crev](reviews/cargo-crev.md) | [docs.rs](https://docs.rs/cargo-crev) <!-- [lib.rs](https://lib.rs/crates/cargo-crev) --> | Share code reviews/audits through a web of trust
| ![crev-neutral] &nbsp; [cargo-edit](reviews/cargo-edit.md) | [docs.rs](https://docs.rs/cargo-edit) <!-- [lib.rs](https://lib.rs/crates/cargo-edit) --> | Add/remove/update Cargo.toml dependencies from the command line.

<h2 id="data-structure">Data Structure</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-neutral] &nbsp; [arrayvec](reviews/arrayvec.md) | [docs.rs](https://docs.rs/arrayvec) <!-- [lib.rs](https://lib.rs/crates/arrayvec) --> | Vec clone (Fixed capacity, no heap). Prefer Vec?
| ![crev-positive] &nbsp; [lazycell](reviews/lazycell.md) | [docs.rs](https://docs.rs/lazycell) <!-- [lib.rs](https://lib.rs/crates/lazycell) --> | Similar to RefCell&lt;Option&lt;T&gt;&gt;, but you can keep T borrowed
| ![crev-negative] &nbsp; [smallvec](reviews/smallvec.md) | [docs.rs](https://docs.rs/smallvec) <!-- [lib.rs](https://lib.rs/crates/smallvec) --> | Vec clone (Small Buffer Optimization, Heap Fallback). Prefer Vec.
| ![crev-none] &nbsp; [smol_str](reviews/smol_str.md) | [docs.rs](https://docs.rs/smol_str) <!-- [lib.rs](https://lib.rs/crates/smol_str) --> | Immutable small string premature optimizations
| ![crev-positive] &nbsp; [void](reviews/void.md) | [docs.rs](https://docs.rs/void) <!-- [lib.rs](https://lib.rs/crates/void) --> | Uninhabited type.

<h2 id="debugging">Debugging</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-author] &nbsp; [bugsalot](reviews/bugsalot.md) | [docs.rs](https://docs.rs/bugsalot) <!-- [lib.rs](https://lib.rs/crates/bugsalot) --> | Breakpoints, debugger detection, fail-stable macros, etc.
| ![crev-positive] &nbsp; [gimli](reviews/gimli.md) | [docs.rs](https://docs.rs/gimli) <!-- [lib.rs](https://lib.rs/crates/gimli) --> | DWARF debug info parsing.
| ![crev-author] &nbsp; [natvis-pdbs](reviews/natvis-pdbs.md) | [docs.rs](https://docs.rs/natvis-pdbs) <!-- [lib.rs](https://lib.rs/crates/natvis-pdbs) --> | Embed .natvis files into your .pdb s via build.rs/metabuild script.
| ![crev-positive] &nbsp; [vlq](reviews/vlq.md) | [docs.rs](https://docs.rs/vlq) <!-- [lib.rs](https://lib.rs/crates/vlq) --> | Sourcemap VLQ Base64 encode/decode
| ![crev-none] &nbsp; [wasm-dwarf](reviews/wasm-dwarf.md) | [docs.rs](https://docs.rs/wasm-dwarf) <!-- [lib.rs](https://lib.rs/crates/wasm-dwarf) --> | WASM Dwarf reader / .map generator
| ![crev-positive] &nbsp; [wasmparser](reviews/wasmparser.md) | [docs.rs](https://docs.rs/wasmparser) <!-- [lib.rs](https://lib.rs/crates/wasmparser) --> | `.wasm` file parser

<h2 id="ffi">FFI</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-none] &nbsp; [bindgen](reviews/bindgen.md) | [docs.rs](https://docs.rs/bindgen) <!-- [lib.rs](https://lib.rs/crates/bindgen) --> | Generate Rust bindings from C/C++ headers
| ![crev-none] &nbsp; [cbindgen](reviews/cbindgen.md) | [docs.rs](https://docs.rs/cbindgen) <!-- [lib.rs](https://lib.rs/crates/cbindgen) --> | Generate C/C++ headers for Rust code
| ![crev-neutral] &nbsp; [cloudabi](reviews/cloudabi.md) | [docs.rs](https://docs.rs/cloudabi) <!-- [lib.rs](https://lib.rs/crates/cloudabi) --> | Reduced capability-based POSIX subset/alternative.
| ![crev-none] &nbsp; [com_impl](reviews/com_impl.md) | [docs.rs](https://docs.rs/com_impl) <!-- [lib.rs](https://lib.rs/crates/com_impl) --> | COM interop utilities.
| ![crev-neutral] &nbsp; [foreign-types](reviews/foreign-types.md) | [docs.rs](https://docs.rs/foreign-types) <!-- [lib.rs](https://lib.rs/crates/foreign-types) --> | Generate Rust wrappers around C types
| ![crev-negative] &nbsp; [foreign-types-shared](reviews/foreign-types-shared.md) | [docs.rs](https://docs.rs/foreign-types-shared) <!-- [lib.rs](https://lib.rs/crates/foreign-types-shared) --> | foreign-types support crate
| ![crev-none] &nbsp; [libc](reviews/libc.md) | [docs.rs](https://docs.rs/libc) <!-- [lib.rs](https://lib.rs/crates/libc) --> | POSIX / C APIs megacrate.  You use this.
| ![crev-negative] &nbsp; [redox_syscall](reviews/redox_syscall.md) | [docs.rs](https://docs.rs/redox_syscall) <!-- [lib.rs](https://lib.rs/crates/redox_syscall) --> | System calls for the Rust OS, Redox
| ![crev-none] &nbsp; [rust-ffi](reviews/rust-ffi.md) | [docs.rs](https://docs.rs/rust-ffi) <!-- [lib.rs](https://lib.rs/crates/rust-ffi) --> | Generate C/C++ headers for Rust code
| ![crev-none] &nbsp; [winapi](reviews/winapi.md) | [docs.rs](https://docs.rs/winapi) <!-- [lib.rs](https://lib.rs/crates/winapi) --> | Win32 / Windows APIs megacrate.  You use this.
| ![crev-none] &nbsp; [winrt](reviews/winrt.md) | [docs.rs](https://docs.rs/winrt) <!-- [lib.rs](https://lib.rs/crates/winrt) --> | C++/CX APIs megacrate.

<h2 id="game-engine">Game Engine</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-none] &nbsp; [amethyst](reviews/amethyst.md) | [docs.rs](https://docs.rs/amethyst) <!-- [lib.rs](https://lib.rs/crates/amethyst) --> | Heavyweight data driven game engine.  Seems popular.
| ![crev-none] &nbsp; [ggez](reviews/ggez.md) | [docs.rs](https://docs.rs/ggez) <!-- [lib.rs](https://lib.rs/crates/ggez) --> | 
| ![crev-none] &nbsp; [piston](reviews/piston.md) | [docs.rs](https://docs.rs/piston) <!-- [lib.rs](https://lib.rs/crates/piston) --> | 
| ![crev-none] &nbsp; [quicksilver](reviews/quicksilver.md) | [docs.rs](https://docs.rs/quicksilver) <!-- [lib.rs](https://lib.rs/crates/quicksilver) --> | Lightweight engine targeting Desktop &amp; Browser

<h2 id="gamedev">Gamedev</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-negative] &nbsp; [ase](reviews/ase.md) | [docs.rs](https://docs.rs/ase) <!-- [lib.rs](https://lib.rs/crates/ase) --> | Asesprite Format Reader
| ![crev-positive] &nbsp; [glsl-include](reviews/glsl-include.md) | [docs.rs](https://docs.rs/glsl-include) <!-- [lib.rs](https://lib.rs/crates/glsl-include) --> | Handle basic `#include`s for GLSL.
| ![crev-negative] &nbsp; [legion](reviews/legion.md) | [docs.rs](https://docs.rs/legion) <!-- [lib.rs](https://lib.rs/crates/legion) --> | A low-boilerplate, high performance archetype based ECS. Lots of unsafe, possibly unsound, overflow concerns, etc.
| ![crev-author] &nbsp; [nines](reviews/nines.md) | [docs.rs](https://docs.rs/nines) <!-- [lib.rs](https://lib.rs/crates/nines) --> | 9-slice scaling math
| ![crev-neutral] &nbsp; [rdrand](reviews/rdrand.md) | [docs.rs](https://docs.rs/rdrand) <!-- [lib.rs](https://lib.rs/crates/rdrand) --> | 🎲 Get random numbers 🎲
| ![crev-neutral] &nbsp; [specs](reviews/specs.md) | [docs.rs](https://docs.rs/specs) <!-- [lib.rs](https://lib.rs/crates/specs) --> | High boilerplate ECS.  Fancy and parallel though.
| ![crev-negative] &nbsp; [tiled](reviews/tiled.md) | [docs.rs](https://docs.rs/tiled) <!-- [lib.rs](https://lib.rs/crates/tiled) --> | [Tiled](https://www.mapeditor.org/) `.tmx` file parser. Decent bones, but I&#39;m concerned about path traversal attacks.
| ![crev-negative] &nbsp; [tiled-json-rs](reviews/tiled-json-rs.md) | [docs.rs](https://docs.rs/tiled-json-rs) <!-- [lib.rs](https://lib.rs/crates/tiled-json-rs) --> | [Tiled](https://www.mapeditor.org/) `.json` export file parser. Decent bones, but I&#39;m concerned about path traversal attacks.

<h2 id="general-utility">General Utility</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-none] &nbsp; [crates-index](reviews/crates-index.md) | [docs.rs](https://docs.rs/crates-index) <!-- [lib.rs](https://lib.rs/crates/crates-index) --> | Parse the crates.io index
| ![crev-positive] &nbsp; [icon-pie](reviews/icon-pie.md) | [docs.rs](https://docs.rs/icon-pie) <!-- [lib.rs](https://lib.rs/crates/icon-pie) --> | Generate .ico / .icns
| ![crev-positive] &nbsp; [inventory](reviews/inventory.md) | [docs.rs](https://docs.rs/inventory) <!-- [lib.rs](https://lib.rs/crates/inventory) --> | Decentralized static registration
| ![crev-positive] &nbsp; [lazy_static](reviews/lazy_static.md) | [docs.rs](https://docs.rs/lazy_static) <!-- [lib.rs](https://lib.rs/crates/lazy_static) --> | Static init at runtime.
| ![crev-positive] &nbsp; [macro_rules_attribute](reviews/macro_rules_attribute.md) | [docs.rs](https://docs.rs/macro_rules_attribute) <!-- [lib.rs](https://lib.rs/crates/macro_rules_attribute) --> | Provides a #[derive(...)]-like attribute without needing your own proc macro crate.
| ![crev-positive] &nbsp; [matches](reviews/matches.md) | [docs.rs](https://docs.rs/matches) <!-- [lib.rs](https://lib.rs/crates/matches) --> | `matches!(variable, SomeEnum::SomeCase) == true`
| ![crev-positive] &nbsp; [num_cpus](reviews/num_cpus.md) | [docs.rs](https://docs.rs/num_cpus) <!-- [lib.rs](https://lib.rs/crates/num_cpus) --> | Queries the OS for the number of CPU cores you have
| ![crev-positive] &nbsp; [num_enum](reviews/num_enum.md) | [docs.rs](https://docs.rs/num_enum) <!-- [lib.rs](https://lib.rs/crates/num_enum) --> | derive traits for enums
| ![crev-positive] &nbsp; [num_enum_derive](reviews/num_enum_derive.md) | [docs.rs](https://docs.rs/num_enum_derive) <!-- [lib.rs](https://lib.rs/crates/num_enum_derive) --> | impl crate for num_enum
| ![crev-positive] &nbsp; [require_unsafe_in_body](reviews/require_unsafe_in_body.md) | [docs.rs](https://docs.rs/require_unsafe_in_body) <!-- [lib.rs](https://lib.rs/crates/require_unsafe_in_body) --> | Reducing the scope of `unsafe { ... }` in `unsafe fn`s.
| ![crev-positive] &nbsp; [threadpool](reviews/threadpool.md) | [docs.rs](https://docs.rs/threadpool) <!-- [lib.rs](https://lib.rs/crates/threadpool) --> | Simple basic thread pool
| ![crev-none] &nbsp; [wchar](reviews/wchar.md) | [docs.rs](https://docs.rs/wchar) <!-- [lib.rs](https://lib.rs/crates/wchar) --> | Compile time UTF16 strings for windows `wchar_t *` interop.

<h2 id="graphics">Graphics</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-none] &nbsp; [egli](reviews/egli.md) | [docs.rs](https://docs.rs/egli) <!-- [lib.rs](https://lib.rs/crates/egli) --> | EGL bindings - provides OpenGL (ES) contexts
| ![crev-none] &nbsp; [khronos-egl](reviews/khronos-egl.md) | [docs.rs](https://docs.rs/khronos-egl) <!-- [lib.rs](https://lib.rs/crates/khronos-egl) --> | EGL bindings - provides OpenGL (ES) contexts

<h2 id="i/o">I/O</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-positive] &nbsp; [codepage-437](reviews/codepage-437.md) | [docs.rs](https://docs.rs/codepage-437) <!-- [lib.rs](https://lib.rs/crates/codepage-437) --> | Codepage 437 conversion functions
| ![crev-none] &nbsp; [codespan](reviews/codespan.md) | [docs.rs](https://docs.rs/codespan) <!-- [lib.rs](https://lib.rs/crates/codespan) --> | Core structures for codespan-reporting
| ![crev-none] &nbsp; [codespan-reporting](reviews/codespan-reporting.md) | [docs.rs](https://docs.rs/codespan-reporting) <!-- [lib.rs](https://lib.rs/crates/codespan-reporting) --> | Beautiful cargo-like error reporting
| ![crev-none] &nbsp; [dlopen](reviews/dlopen.md) | [docs.rs](https://docs.rs/dlopen) <!-- [lib.rs](https://lib.rs/crates/dlopen) --> | Safeish and unsafe APIs for loading `.so`s, `.dll`s at runtime.
| ![crev-positive] &nbsp; [fs2](reviews/fs2.md) | [docs.rs](https://docs.rs/fs2) <!-- [lib.rs](https://lib.rs/crates/fs2) --> | Some extra filesystem utilities
| ![crev-none] &nbsp; [libloading](reviews/libloading.md) | [docs.rs](https://docs.rs/libloading) <!-- [lib.rs](https://lib.rs/crates/libloading) --> | Unsafe APIs for loading `.so`s, `.dll`s at runtime.
| ![crev-negative] &nbsp; [midir](reviews/midir.md) | [docs.rs](https://docs.rs/midir) <!-- [lib.rs](https://lib.rs/crates/midir) --> | Pure rust MIDI device I/O. Good start, but probably unsound.
| ![crev-positive] &nbsp; [podio](reviews/podio.md) | [docs.rs](https://docs.rs/podio) <!-- [lib.rs](https://lib.rs/crates/podio) --> | Utility extension methods for `Read` / `Write`
| ![crev-positive] &nbsp; [shellexpand](reviews/shellexpand.md) | [docs.rs](https://docs.rs/shellexpand) <!-- [lib.rs](https://lib.rs/crates/shellexpand) --> | Expand unix style env vars within strings.
| ![crev-positive] &nbsp; [tempfile](reviews/tempfile.md) | [docs.rs](https://docs.rs/tempfile) <!-- [lib.rs](https://lib.rs/crates/tempfile) --> | Create/cleanup temporary files and directories.
| ![crev-positive] &nbsp; [vfs](reviews/vfs.md) | [docs.rs](https://docs.rs/vfs) <!-- [lib.rs](https://lib.rs/crates/vfs) --> | Filesystem virtualization
| ![crev-positive] &nbsp; [warmy](reviews/warmy.md) | [docs.rs](https://docs.rs/warmy) <!-- [lib.rs](https://lib.rs/crates/warmy) --> | Hot reloading resources. Not browser friendly.

<h2 id="macros">Macros</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-neutral] &nbsp; [proc-macro-crate](reviews/proc-macro-crate.md) | [docs.rs](https://docs.rs/proc-macro-crate) <!-- [lib.rs](https://lib.rs/crates/proc-macro-crate) --> | $crate for proc macros (prefer shim macros per review notes!)

<h2 id="serialization">Serialization</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-negative] &nbsp; [ascii](reviews/ascii.md) | [docs.rs](https://docs.rs/ascii) <!-- [lib.rs](https://lib.rs/crates/ascii) --> | ASCII conversion and parsing.
| ![crev-positive] &nbsp; [ico](reviews/ico.md) | [docs.rs](https://docs.rs/ico) <!-- [lib.rs](https://lib.rs/crates/ico) --> | Encoders/decoders for .ico and .cur file formats
| ![crev-negative] &nbsp; [icon_baker](reviews/icon_baker.md) | [docs.rs](https://docs.rs/icon_baker) <!-- [lib.rs](https://lib.rs/crates/icon_baker) --> | Generate .ico / .icns
| ![crev-positive] &nbsp; [idna](reviews/idna.md) | [docs.rs](https://docs.rs/idna) <!-- [lib.rs](https://lib.rs/crates/idna) --> | Encoding/decoding domain names/punycode.
| ![crev-neutral] &nbsp; [itoa](reviews/itoa.md) | [docs.rs](https://docs.rs/itoa) <!-- [lib.rs](https://lib.rs/crates/itoa) --> | Fast integer I/O
| ![crev-none] &nbsp; [serde](reviews/serde.md) | [docs.rs](https://docs.rs/serde) <!-- [lib.rs](https://lib.rs/crates/serde) --> | The crate used for serialization throughout the Rust ecosystem
| ![crev-none] &nbsp; [serde_json](reviews/serde_json.md) | [docs.rs](https://docs.rs/serde_json) <!-- [lib.rs](https://lib.rs/crates/serde_json) --> | serde companion crate for (de)serializing `.json` files.
| ![crev-positive] &nbsp; [sourcefile](reviews/sourcefile.md) | [docs.rs](https://docs.rs/sourcefile) <!-- [lib.rs](https://lib.rs/crates/sourcefile) --> | Source code file:line &lt;-&gt; offset conversion
| ![crev-none] &nbsp; [toml](reviews/toml.md) | [docs.rs](https://docs.rs/toml) <!-- [lib.rs](https://lib.rs/crates/toml) --> | serde .toml deserialization
| ![crev-none] &nbsp; [toml-spanned-value](reviews/toml-spanned-value.md) | [docs.rs](https://docs.rs/toml-spanned-value) <!-- [lib.rs](https://lib.rs/crates/toml-spanned-value) --> | File line/col span for .toml values
| ![crev-none] &nbsp; [typetag](reviews/typetag.md) | [docs.rs](https://docs.rs/typetag) <!-- [lib.rs](https://lib.rs/crates/typetag) --> | Deserialize Box&lt;dyn Trait&gt; based on inventory registrations.
| ![crev-positive] &nbsp; [xml-rs](reviews/xml-rs.md) | [docs.rs](https://docs.rs/xml-rs) <!-- [lib.rs](https://lib.rs/crates/xml-rs) --> | Encoding and decoding XML. Safe, sound, no deps.
| ![crev-positive] &nbsp; [zip](reviews/zip.md) | [docs.rs](https://docs.rs/zip) <!-- [lib.rs](https://lib.rs/crates/zip) --> | Zipping/unzipping .zip archives.

<h2 id="unsound">Unsound</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-dangerous] &nbsp; [actix-web](reviews/actix-web.md) | [docs.rs](https://docs.rs/actix-web) <!-- [lib.rs](https://lib.rs/crates/actix-web) --> | AVOID.  Closes soundness bugs unfixed.  Deletes external soundness bugs.
| ![crev-negative] &nbsp; [byteorder](reviews/byteorder.md) | [docs.rs](https://docs.rs/byteorder) <!-- [lib.rs](https://lib.rs/crates/byteorder) --> | Super basic casting/endian/swizzling with a history of unsoundness
| ![crev-negative] &nbsp; [cargo-apk](reviews/cargo-apk.md) | [docs.rs](https://docs.rs/cargo-apk) <!-- [lib.rs](https://lib.rs/crates/cargo-apk) --> | Glue code is full of unsafe and unsound.
| ![crev-negative] &nbsp; [crossterm](reviews/crossterm.md) | [docs.rs](https://docs.rs/crossterm) <!-- [lib.rs](https://lib.rs/crates/crossterm) --> | Cross-platform console stuff.  No web support, soundness issues.
| ![crev-negative] &nbsp; [crossterm_cursor](reviews/crossterm_cursor.md) | [docs.rs](https://docs.rs/crossterm_cursor) <!-- [lib.rs](https://lib.rs/crates/crossterm_cursor) --> | Cross-platform console cursor maniulation. Needs soundness fixes.
| ![crev-negative] &nbsp; [crossterm_input](reviews/crossterm_input.md) | [docs.rs](https://docs.rs/crossterm_input) <!-- [lib.rs](https://lib.rs/crates/crossterm_input) --> | Cross-platform console input reading. Needs soundness fixes.
| ![crev-positive] &nbsp; [crossterm_screen](reviews/crossterm_screen.md) | [docs.rs](https://docs.rs/crossterm_screen) <!-- [lib.rs](https://lib.rs/crates/crossterm_screen) --> | 
| ![crev-negative] &nbsp; [crossterm_style](reviews/crossterm_style.md) | [docs.rs](https://docs.rs/crossterm_style) <!-- [lib.rs](https://lib.rs/crates/crossterm_style) --> | 
| ![crev-dangerous] &nbsp; [egl](reviews/egl.md) | [docs.rs](https://docs.rs/egl) <!-- [lib.rs](https://lib.rs/crates/egl) --> | AVOID.  Unsound as fuck, abandoned.  See khronos-egl for a sounder, maintained fork.
| ![crev-dangerous] &nbsp; [memalloc](reviews/memalloc.md) | [docs.rs](https://docs.rs/memalloc) <!-- [lib.rs](https://lib.rs/crates/memalloc) --> | Super brittle/dangerous at a fundamental level. Avoid.
| ![crev-dangerous] &nbsp; [microprofile](reviews/microprofile.md) | [docs.rs](https://docs.rs/microprofile) <!-- [lib.rs](https://lib.rs/crates/microprofile) --> | Bindings for a C++ flamegraph profiler

<h2 id="web">Web</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
| ![crev-none] &nbsp; [cargo-web](reviews/cargo-web.md) | [docs.rs](https://docs.rs/cargo-web) <!-- [lib.rs](https://lib.rs/crates/cargo-web) --> | .wasm packager for use with stdweb
| ![crev-positive] &nbsp; [instant](reviews/instant.md) | [docs.rs](https://docs.rs/instant) <!-- [lib.rs](https://lib.rs/crates/instant) --> | std::time::Instant alternative that doesn&#39;t panic on wasm targets.
| ![crev-none] &nbsp; [js-sys](reviews/js-sys.md) | [docs.rs](https://docs.rs/js-sys) <!-- [lib.rs](https://lib.rs/crates/js-sys) --> | Browser API interop
| ![crev-none] &nbsp; [rocket](reviews/rocket.md) | [docs.rs](https://docs.rs/rocket) <!-- [lib.rs](https://lib.rs/crates/rocket) --> | Rust library for creating API servers.  Nice design.
| ![crev-none] &nbsp; [stdweb](reviews/stdweb.md) | [docs.rs](https://docs.rs/stdweb) <!-- [lib.rs](https://lib.rs/crates/stdweb) --> | Browser API interop
| ![crev-none] &nbsp; [wasm-pack](reviews/wasm-pack.md) | [docs.rs](https://docs.rs/wasm-pack) <!-- [lib.rs](https://lib.rs/crates/wasm-pack) --> | .wasm packager for use with web-sys
| ![crev-none] &nbsp; [web-sys](reviews/web-sys.md) | [docs.rs](https://docs.rs/web-sys) <!-- [lib.rs](https://lib.rs/crates/web-sys) --> | Browser API interop



# Procedures

## Newfangled Reviews

```sh
# Display versions in VS Code
cargo versions byteorder
```

```sh
# Prefer cmd.exe for keepass purpouses

# Generate template and open secondary vscode window with all versions open
cargo review --all byteorder
cargo open byteorder *

# Diff versions
cls && cargo diff byteorder 0.1.1
cls && cargo diff byteorder 0.2.0
...

# Crosspost to crev
cargo install cargo-crev
cargo crev crate review -u --advisory            byteorder --vers 0.2.11
cargo crev crate review -u --advisory            byteorder --vers 0.3.8
cargo crev crate review -u --skip-activity-check byteorder --vers 1.3.4
cargo crev repo git diff HEAD~1
cargo crev repo publish
```

```yml
# Combined advisory/review/flags/alternatives template
advisories:
  - ids: []
    severity: medium
    range: major
    comment: ""
review:
  thoroughness: low
  understanding: medium
  rating: positive
flags:
  unmaintained: false
alternatives:
  - source: "https://crates.io"
    name: ""
comment: |-
```
