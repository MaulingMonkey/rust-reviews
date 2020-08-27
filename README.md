# MaulingMonkey's Rust Reviews

This repository serves a few purpouses:
* To provide a quick overview and human readable versions of all [my cargo crev reviews](https://github.com/MaulingMonkey/crev-proofs).
* To provide a repository that [Dependabot](https://dependabot.com) can create issues against, to remind me to update my crev proofs.
* To provide a visual fallback via deps.rs:  [![deps.rs](https://deps.rs/repo/github/MaulingMonkey/rust-reviews/status.svg)](https://deps.rs/repo/github/MaulingMonkey/rust-reviews)

[crev-author]:      https://img.shields.io/badge/author-🐵-green
[crev-none]:        https://img.shields.io/badge/crev-N/A-lightblue

[audio-rodio]:      https://img.shields.io/badge/🔊-rodio-green

[crev-positive]:    https://img.shields.io/badge/crev-✓-green
[crev-neutral]:     https://img.shields.io/badge/crev-%3D-lightgrey
[crev-negative]:    https://img.shields.io/badge/crev-✗-yellow
[crev-dangerous]:   https://img.shields.io/badge/crev-✗-red

# Categories

* [Android](#android)
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

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [cargo-dinghy](reviews/cargo-dinghy.md) | [![crev-positive]](reviews/cargo-dinghy.md) | [docs.rs](https://docs.rs/cargo-dinghy) <!-- [lib.rs](https://lib.rs/crates/cargo-dinghy) --> | `cargo` subcommand for building Android/iOS
| [cargo-ndk](reviews/cargo-ndk.md) | [![crev-positive]](reviews/cargo-ndk.md) | [docs.rs](https://docs.rs/cargo-ndk) <!-- [lib.rs](https://lib.rs/crates/cargo-ndk) --> | Kinda trivial `.apk` building.
| [dinghy-build](reviews/dinghy-build.md) | [![crev-positive]](reviews/dinghy-build.md) | [docs.rs](https://docs.rs/dinghy-build) <!-- [lib.rs](https://lib.rs/crates/dinghy-build) --> | 
| [dinghy-lib](reviews/dinghy-lib.md) | [![crev-neutral]](reviews/dinghy-lib.md) | [docs.rs](https://docs.rs/dinghy-lib) <!-- [lib.rs](https://lib.rs/crates/dinghy-lib) --> | 
| [jerk-build](reviews/jerk-build.md) | [![crev-author]](reviews/jerk-build.md) | [docs.rs](https://docs.rs/jerk-build) <!-- [lib.rs](https://lib.rs/crates/jerk-build) --> | Build Java alongside Rust via build.rs/metabuild scripts
| [jerk-test](reviews/jerk-test.md) | [![crev-author]](reviews/jerk-test.md) | [docs.rs](https://docs.rs/jerk-test) <!-- [lib.rs](https://lib.rs/crates/jerk-test) --> | Unit test Java built alongside Rust
| [jerk](reviews/jerk.md) | [![crev-author]](reviews/jerk.md) | [docs.rs](https://docs.rs/jerk) <!-- [lib.rs](https://lib.rs/crates/jerk) --> | Java path discovery and other utilities
| [jni-android-sys](reviews/jni-android-sys.md) | [![crev-author]](reviews/jni-android-sys.md) | [docs.rs](https://docs.rs/jni-android-sys) <!-- [lib.rs](https://lib.rs/crates/jni-android-sys) --> | Bindings to Android Java APIs
| [jni-bindgen](reviews/jni-bindgen.md) | [![crev-author]](reviews/jni-bindgen.md) | [docs.rs](https://docs.rs/jni-bindgen) <!-- [lib.rs](https://lib.rs/crates/jni-bindgen) --> | Java API binding code generator
| [jni-glue-macros](reviews/jni-glue-macros.md) | [![crev-author]](reviews/jni-glue-macros.md) | [docs.rs](https://docs.rs/jni-glue-macros) <!-- [lib.rs](https://lib.rs/crates/jni-glue-macros) --> | Proc macros to implement Java APIs from Rust
| [jni-glue](reviews/jni-glue.md) | [![crev-author]](reviews/jni-glue.md) | [docs.rs](https://docs.rs/jni-glue) <!-- [lib.rs](https://lib.rs/crates/jni-glue) --> | Safeish wrappers around jni-sys used by jni-bindgen bindings
| [jni-sys](reviews/jni-sys.md) | [![crev-positive]](reviews/jni-sys.md) | [docs.rs](https://docs.rs/jni-sys) <!-- [lib.rs](https://lib.rs/crates/jni-sys) --> | Rust bindings for JNI interop.
| [jni](reviews/jni.md) | [![crev-negative]](reviews/jni.md) | [docs.rs](https://docs.rs/jni) <!-- [lib.rs](https://lib.rs/crates/jni) --> | Unsafe and unsound. Has responded to fixes well though.

<h2 id="build-utility">Build Utility</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [autocfg](reviews/autocfg.md) | [![crev-positive]](reviews/autocfg.md) | [docs.rs](https://docs.rs/autocfg) <!-- [lib.rs](https://lib.rs/crates/autocfg) --> | Runs `rustc` to test for features / versions.
| [cargo_metadata](reviews/cargo_metadata.md) | [![crev-positive]](reviews/cargo_metadata.md) | [docs.rs](https://docs.rs/cargo_metadata) <!-- [lib.rs](https://lib.rs/crates/cargo_metadata) --> | Parse `cargo metadata` and `cargo build --message-format=json` output.
| [cfg-if](reviews/cfg-if.md) | [![crev-positive]](reviews/cfg-if.md) | [docs.rs](https://docs.rs/cfg-if) <!-- [lib.rs](https://lib.rs/crates/cfg-if) --> | Parse `cargo metadata` and `cargo build --message-format=json` output.
| [lies-impl](reviews/lies-impl.md) | [![crev-author]](reviews/lies-impl.md) | [docs.rs](https://docs.rs/lies-impl) <!-- [lib.rs](https://lib.rs/crates/lies-impl) --> | 
| [lies](reviews/lies.md) | [![crev-author]](reviews/lies.md) | [docs.rs](https://docs.rs/lies) <!-- [lib.rs](https://lib.rs/crates/lies) --> | Embed license text into your program via proc macros around cargo-about.
| [rustversion](reviews/rustversion.md) | [![crev-positive]](reviews/rustversion.md) | [docs.rs](https://docs.rs/rustversion) <!-- [lib.rs](https://lib.rs/crates/rustversion) --> | Attributes to do conditional compilation based on rust version/channel
| [vcpkg](reviews/vcpkg.md) | [![crev-positive]](reviews/vcpkg.md) | [docs.rs](https://docs.rs/vcpkg) <!-- [lib.rs](https://lib.rs/crates/vcpkg) --> | Build dependency to get C/C++ [vcpkg](https://github.com/Microsoft/vcpkg)s

<h2 id="cli-tools">CLI Tools</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [cargo-about](reviews/cargo-about.md) | [![crev-none]](reviews/cargo-about.md) | [docs.rs](https://docs.rs/cargo-about) <!-- [lib.rs](https://lib.rs/crates/cargo-about) --> | Validate dependency licenses and aggregate into a single .html file
| [cargo-crev](reviews/cargo-crev.md) | [![crev-none]](reviews/cargo-crev.md) | [docs.rs](https://docs.rs/cargo-crev) <!-- [lib.rs](https://lib.rs/crates/cargo-crev) --> | Share code reviews/audits through a web of trust
| [cargo-edit](reviews/cargo-edit.md) | [![crev-neutral]](reviews/cargo-edit.md) | [docs.rs](https://docs.rs/cargo-edit) <!-- [lib.rs](https://lib.rs/crates/cargo-edit) --> | Add/remove/update Cargo.toml dependencies from the command line.
| [cargo](reviews/cargo.md) | [![crev-positive]](reviews/cargo.md) | [docs.rs](https://docs.rs/cargo) <!-- [lib.rs](https://lib.rs/crates/cargo) --> | *The* rust build tool.

<h2 id="data-structure">Data Structure</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [arrayvec](reviews/arrayvec.md) | [![crev-neutral]](reviews/arrayvec.md) | [docs.rs](https://docs.rs/arrayvec) <!-- [lib.rs](https://lib.rs/crates/arrayvec) --> | Vec clone (Fixed capacity, no heap). Prefer Vec?
| [lazycell](reviews/lazycell.md) | [![crev-positive]](reviews/lazycell.md) | [docs.rs](https://docs.rs/lazycell) <!-- [lib.rs](https://lib.rs/crates/lazycell) --> | Similar to RefCell&lt;Option&lt;T&gt;&gt;, but you can keep T borrowed
| [smallvec](reviews/smallvec.md) | [![crev-negative]](reviews/smallvec.md) | [docs.rs](https://docs.rs/smallvec) <!-- [lib.rs](https://lib.rs/crates/smallvec) --> | Vec clone (Small Buffer Optimization, Heap Fallback). Prefer Vec.
| [smol_str](reviews/smol_str.md) | [![crev-none]](reviews/smol_str.md) | [docs.rs](https://docs.rs/smol_str) <!-- [lib.rs](https://lib.rs/crates/smol_str) --> | Immutable small string premature optimizations
| [void](reviews/void.md) | [![crev-positive]](reviews/void.md) | [docs.rs](https://docs.rs/void) <!-- [lib.rs](https://lib.rs/crates/void) --> | Uninhabited type.

<h2 id="debugging">Debugging</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [bugsalot](reviews/bugsalot.md) | [![crev-author]](reviews/bugsalot.md) | [docs.rs](https://docs.rs/bugsalot) <!-- [lib.rs](https://lib.rs/crates/bugsalot) --> | Breakpoints, debugger detection, fail-stable macros, etc.
| [gimli](reviews/gimli.md) | [![crev-positive]](reviews/gimli.md) | [docs.rs](https://docs.rs/gimli) <!-- [lib.rs](https://lib.rs/crates/gimli) --> | DWARF debug info parsing.
| [natvis-pdbs](reviews/natvis-pdbs.md) | [![crev-author]](reviews/natvis-pdbs.md) | [docs.rs](https://docs.rs/natvis-pdbs) <!-- [lib.rs](https://lib.rs/crates/natvis-pdbs) --> | Embed .natvis files into your .pdb s via build.rs/metabuild script.
| [vlq](reviews/vlq.md) | [![crev-positive]](reviews/vlq.md) | [docs.rs](https://docs.rs/vlq) <!-- [lib.rs](https://lib.rs/crates/vlq) --> | Sourcemap VLQ Base64 encode/decode
| [wasm-dwarf](reviews/wasm-dwarf.md) | [![crev-none]](reviews/wasm-dwarf.md) | [docs.rs](https://docs.rs/wasm-dwarf) <!-- [lib.rs](https://lib.rs/crates/wasm-dwarf) --> | WASM Dwarf reader / .map generator
| [wasmparser](reviews/wasmparser.md) | [![crev-positive]](reviews/wasmparser.md) | [docs.rs](https://docs.rs/wasmparser) <!-- [lib.rs](https://lib.rs/crates/wasmparser) --> | `.wasm` file parser

<h2 id="ffi">FFI</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [bindgen](reviews/bindgen.md) | [![crev-none]](reviews/bindgen.md) | [docs.rs](https://docs.rs/bindgen) <!-- [lib.rs](https://lib.rs/crates/bindgen) --> | Generate Rust bindings from C/C++ headers
| [cbindgen](reviews/cbindgen.md) | [![crev-none]](reviews/cbindgen.md) | [docs.rs](https://docs.rs/cbindgen) <!-- [lib.rs](https://lib.rs/crates/cbindgen) --> | Generate C/C++ headers for Rust code
| [cloudabi](reviews/cloudabi.md) | [![crev-neutral]](reviews/cloudabi.md) | [docs.rs](https://docs.rs/cloudabi) <!-- [lib.rs](https://lib.rs/crates/cloudabi) --> | Reduced capability-based POSIX subset/alternative.
| [com_impl](reviews/com_impl.md) | [![crev-none]](reviews/com_impl.md) | [docs.rs](https://docs.rs/com_impl) <!-- [lib.rs](https://lib.rs/crates/com_impl) --> | COM interop utilities.
| [foreign-types-shared](reviews/foreign-types-shared.md) | [![crev-negative]](reviews/foreign-types-shared.md) | [docs.rs](https://docs.rs/foreign-types-shared) <!-- [lib.rs](https://lib.rs/crates/foreign-types-shared) --> | foreign-types support crate
| [foreign-types](reviews/foreign-types.md) | [![crev-neutral]](reviews/foreign-types.md) | [docs.rs](https://docs.rs/foreign-types) <!-- [lib.rs](https://lib.rs/crates/foreign-types) --> | Generate Rust wrappers around C types
| [libc](reviews/libc.md) | [![crev-none]](reviews/libc.md) | [docs.rs](https://docs.rs/libc) <!-- [lib.rs](https://lib.rs/crates/libc) --> | POSIX / C APIs megacrate.  You use this.
| [redox_syscall](reviews/redox_syscall.md) | [![crev-negative]](reviews/redox_syscall.md) | [docs.rs](https://docs.rs/redox_syscall) <!-- [lib.rs](https://lib.rs/crates/redox_syscall) --> | System calls for the Rust OS, Redox
| [rust-ffi](reviews/rust-ffi.md) | [![crev-none]](reviews/rust-ffi.md) | [docs.rs](https://docs.rs/rust-ffi) <!-- [lib.rs](https://lib.rs/crates/rust-ffi) --> | Generate C/C++ headers for Rust code
| [winapi](reviews/winapi.md) | [![crev-none]](reviews/winapi.md) | [docs.rs](https://docs.rs/winapi) <!-- [lib.rs](https://lib.rs/crates/winapi) --> | Win32 / Windows APIs megacrate.  You use this.
| [winrt](reviews/winrt.md) | [![crev-none]](reviews/winrt.md) | [docs.rs](https://docs.rs/winrt) <!-- [lib.rs](https://lib.rs/crates/winrt) --> | C++/CX APIs megacrate.

<h2 id="game-engine">Game Engine</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [amethyst](reviews/amethyst.md) | [![crev-none]](reviews/amethyst.md) | [docs.rs](https://docs.rs/amethyst) <!-- [lib.rs](https://lib.rs/crates/amethyst) --> | Heavyweight data driven game engine.  Seems popular.
| [ggez](reviews/ggez.md) | [![crev-none]](reviews/ggez.md) | [docs.rs](https://docs.rs/ggez) <!-- [lib.rs](https://lib.rs/crates/ggez) --> | 
| [piston](reviews/piston.md) | [![crev-none]](reviews/piston.md) | [docs.rs](https://docs.rs/piston) <!-- [lib.rs](https://lib.rs/crates/piston) --> | 
| [quicksilver](reviews/quicksilver.md) | [![crev-none]](reviews/quicksilver.md) | [docs.rs](https://docs.rs/quicksilver) <!-- [lib.rs](https://lib.rs/crates/quicksilver) --> | Lightweight engine targeting Desktop &amp; Browser

<h2 id="gamedev">Gamedev</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [ase](reviews/ase.md) | [![crev-negative]](reviews/ase.md) | [docs.rs](https://docs.rs/ase) <!-- [lib.rs](https://lib.rs/crates/ase) --> | Asesprite Format Reader
| [glsl-include](reviews/glsl-include.md) | [![crev-positive]](reviews/glsl-include.md) | [docs.rs](https://docs.rs/glsl-include) <!-- [lib.rs](https://lib.rs/crates/glsl-include) --> | Handle basic `#include`s for GLSL.
| [legion](reviews/legion.md) | [![crev-negative]](reviews/legion.md) | [docs.rs](https://docs.rs/legion) <!-- [lib.rs](https://lib.rs/crates/legion) --> | A low-boilerplate, high performance archetype based ECS. Lots of unsafe, possibly unsound, overflow concerns, etc.
| [nines](reviews/nines.md) | [![crev-author]](reviews/nines.md) | [docs.rs](https://docs.rs/nines) <!-- [lib.rs](https://lib.rs/crates/nines) --> | 9-slice scaling math
| [rdrand](reviews/rdrand.md) | [![crev-neutral]](reviews/rdrand.md) | [docs.rs](https://docs.rs/rdrand) <!-- [lib.rs](https://lib.rs/crates/rdrand) --> | 🎲 Get random numbers 🎲
| [specs](reviews/specs.md) | [![crev-neutral]](reviews/specs.md) | [docs.rs](https://docs.rs/specs) <!-- [lib.rs](https://lib.rs/crates/specs) --> | High boilerplate ECS.  Fancy and parallel though.
| [tiled-json-rs](reviews/tiled-json-rs.md) | [![crev-negative]](reviews/tiled-json-rs.md) | [docs.rs](https://docs.rs/tiled-json-rs) <!-- [lib.rs](https://lib.rs/crates/tiled-json-rs) --> | [Tiled](https://www.mapeditor.org/) `.json` export file parser. Decent bones, but I&#39;m concerned about path traversal attacks.
| [tiled](reviews/tiled.md) | [![crev-negative]](reviews/tiled.md) | [docs.rs](https://docs.rs/tiled) <!-- [lib.rs](https://lib.rs/crates/tiled) --> | [Tiled](https://www.mapeditor.org/) `.tmx` file parser. Decent bones, but I&#39;m concerned about path traversal attacks.

<h2 id="general-utility">General Utility</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [crates-index](reviews/crates-index.md) | [![crev-none]](reviews/crates-index.md) | [docs.rs](https://docs.rs/crates-index) <!-- [lib.rs](https://lib.rs/crates/crates-index) --> | Parse the crates.io index
| [inventory](reviews/inventory.md) | [![crev-positive]](reviews/inventory.md) | [docs.rs](https://docs.rs/inventory) <!-- [lib.rs](https://lib.rs/crates/inventory) --> | Decentralized static registration
| [lazy_static](reviews/lazy_static.md) | [![crev-positive]](reviews/lazy_static.md) | [docs.rs](https://docs.rs/lazy_static) <!-- [lib.rs](https://lib.rs/crates/lazy_static) --> | Static init at runtime.
| [macro_rules_attribute](reviews/macro_rules_attribute.md) | [![crev-positive]](reviews/macro_rules_attribute.md) | [docs.rs](https://docs.rs/macro_rules_attribute) <!-- [lib.rs](https://lib.rs/crates/macro_rules_attribute) --> | Provides a #[derive(...)]-like attribute without needing your own proc macro crate.
| [matches](reviews/matches.md) | [![crev-positive]](reviews/matches.md) | [docs.rs](https://docs.rs/matches) <!-- [lib.rs](https://lib.rs/crates/matches) --> | `matches!(variable, SomeEnum::SomeCase) == true`
| [num_enum](reviews/num_enum.md) | [![crev-positive]](reviews/num_enum.md) | [docs.rs](https://docs.rs/num_enum) <!-- [lib.rs](https://lib.rs/crates/num_enum) --> | derive traits for enums
| [num_enum_derive](reviews/num_enum_derive.md) | [![crev-positive]](reviews/num_enum_derive.md) | [docs.rs](https://docs.rs/num_enum_derive) <!-- [lib.rs](https://lib.rs/crates/num_enum_derive) --> | impl crate for num_enum
| [require_unsafe_in_body](reviews/require_unsafe_in_body.md) | [![crev-positive]](reviews/require_unsafe_in_body.md) | [docs.rs](https://docs.rs/require_unsafe_in_body) <!-- [lib.rs](https://lib.rs/crates/require_unsafe_in_body) --> | Reducing the scope of `unsafe { ... }` in `unsafe fn`s.
| [wchar](reviews/wchar.md) | [![crev-none]](reviews/wchar.md) | [docs.rs](https://docs.rs/wchar) <!-- [lib.rs](https://lib.rs/crates/wchar) --> | Compile time UTF16 strings for windows `wchar_t *` interop.

<h2 id="graphics">Graphics</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [egli](reviews/egli.md) | [![crev-none]](reviews/egli.md) | [docs.rs](https://docs.rs/egli) <!-- [lib.rs](https://lib.rs/crates/egli) --> | EGL bindings - provides OpenGL (ES) contexts
| [khronos-egl](reviews/khronos-egl.md) | [![crev-none]](reviews/khronos-egl.md) | [docs.rs](https://docs.rs/khronos-egl) <!-- [lib.rs](https://lib.rs/crates/khronos-egl) --> | EGL bindings - provides OpenGL (ES) contexts

<h2 id="i/o">I/O</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [codepage-437](reviews/codepage-437.md) | [![crev-positive]](reviews/codepage-437.md) | [docs.rs](https://docs.rs/codepage-437) <!-- [lib.rs](https://lib.rs/crates/codepage-437) --> | Codepage 437 conversion functions
| [codespan-reporting](reviews/codespan-reporting.md) | [![crev-none]](reviews/codespan-reporting.md) | [docs.rs](https://docs.rs/codespan-reporting) <!-- [lib.rs](https://lib.rs/crates/codespan-reporting) --> | Beautiful cargo-like error reporting
| [codespan](reviews/codespan.md) | [![crev-none]](reviews/codespan.md) | [docs.rs](https://docs.rs/codespan) <!-- [lib.rs](https://lib.rs/crates/codespan) --> | Core structures for codespan-reporting
| [dlopen](reviews/dlopen.md) | [![crev-none]](reviews/dlopen.md) | [docs.rs](https://docs.rs/dlopen) <!-- [lib.rs](https://lib.rs/crates/dlopen) --> | Safeish and unsafe APIs for loading `.so`s, `.dll`s at runtime.
| [fs2](reviews/fs2.md) | [![crev-positive]](reviews/fs2.md) | [docs.rs](https://docs.rs/fs2) <!-- [lib.rs](https://lib.rs/crates/fs2) --> | Some extra filesystem utilities
| [libloading](reviews/libloading.md) | [![crev-none]](reviews/libloading.md) | [docs.rs](https://docs.rs/libloading) <!-- [lib.rs](https://lib.rs/crates/libloading) --> | Unsafe APIs for loading `.so`s, `.dll`s at runtime.
| [midir](reviews/midir.md) | [![crev-negative]](reviews/midir.md) | [docs.rs](https://docs.rs/midir) <!-- [lib.rs](https://lib.rs/crates/midir) --> | Pure rust MIDI device I/O. Good start, but probably unsound.
| [podio](reviews/podio.md) | [![crev-positive]](reviews/podio.md) | [docs.rs](https://docs.rs/podio) <!-- [lib.rs](https://lib.rs/crates/podio) --> | Utility extension methods for `Read` / `Write`
| [shellexpand](reviews/shellexpand.md) | [![crev-positive]](reviews/shellexpand.md) | [docs.rs](https://docs.rs/shellexpand) <!-- [lib.rs](https://lib.rs/crates/shellexpand) --> | Expand unix style env vars within strings.
| [tempfile](reviews/tempfile.md) | [![crev-positive]](reviews/tempfile.md) | [docs.rs](https://docs.rs/tempfile) <!-- [lib.rs](https://lib.rs/crates/tempfile) --> | Create/cleanup temporary files and directories.
| [warmy](reviews/warmy.md) | [![crev-positive]](reviews/warmy.md) | [docs.rs](https://docs.rs/warmy) <!-- [lib.rs](https://lib.rs/crates/warmy) --> | Hot reloading resources. Not browser friendly.

<h2 id="macros">Macros</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [proc-macro-crate](reviews/proc-macro-crate.md) | [![crev-neutral]](reviews/proc-macro-crate.md) | [docs.rs](https://docs.rs/proc-macro-crate) <!-- [lib.rs](https://lib.rs/crates/proc-macro-crate) --> | $crate for proc macros (prefer shim macros per review notes!)

<h2 id="serialization">Serialization</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [ascii](reviews/ascii.md) | [![crev-negative]](reviews/ascii.md) | [docs.rs](https://docs.rs/ascii) <!-- [lib.rs](https://lib.rs/crates/ascii) --> | ASCII conversion and parsing.
| [idna](reviews/idna.md) | [![crev-positive]](reviews/idna.md) | [docs.rs](https://docs.rs/idna) <!-- [lib.rs](https://lib.rs/crates/idna) --> | Encoding/decoding domain names/punycode.
| [itoa](reviews/itoa.md) | [![crev-neutral]](reviews/itoa.md) | [docs.rs](https://docs.rs/itoa) <!-- [lib.rs](https://lib.rs/crates/itoa) --> | Fast integer I/O
| [serde](reviews/serde.md) | [![crev-none]](reviews/serde.md) | [docs.rs](https://docs.rs/serde) <!-- [lib.rs](https://lib.rs/crates/serde) --> | The crate used for serialization throughout the Rust ecosystem
| [serde_json](reviews/serde_json.md) | [![crev-none]](reviews/serde_json.md) | [docs.rs](https://docs.rs/serde_json) <!-- [lib.rs](https://lib.rs/crates/serde_json) --> | serde companion crate for (de)serializing `.json` files.
| [sourcefile](reviews/sourcefile.md) | [![crev-positive]](reviews/sourcefile.md) | [docs.rs](https://docs.rs/sourcefile) <!-- [lib.rs](https://lib.rs/crates/sourcefile) --> | Source code file:line &lt;-&gt; offset conversion
| [toml-spanned-value](reviews/toml-spanned-value.md) | [![crev-none]](reviews/toml-spanned-value.md) | [docs.rs](https://docs.rs/toml-spanned-value) <!-- [lib.rs](https://lib.rs/crates/toml-spanned-value) --> | File line/col span for .toml values
| [toml](reviews/toml.md) | [![crev-none]](reviews/toml.md) | [docs.rs](https://docs.rs/toml) <!-- [lib.rs](https://lib.rs/crates/toml) --> | serde .toml deserialization
| [typetag](reviews/typetag.md) | [![crev-none]](reviews/typetag.md) | [docs.rs](https://docs.rs/typetag) <!-- [lib.rs](https://lib.rs/crates/typetag) --> | Deserialize Box&lt;dyn Trait&gt; based on inventory registrations.
| [xml-rs](reviews/xml-rs.md) | [![crev-positive]](reviews/xml-rs.md) | [docs.rs](https://docs.rs/xml-rs) <!-- [lib.rs](https://lib.rs/crates/xml-rs) --> | Encoding and decoding XML. Safe, sound, no deps.
| [zip](reviews/zip.md) | [![crev-positive]](reviews/zip.md) | [docs.rs](https://docs.rs/zip) <!-- [lib.rs](https://lib.rs/crates/zip) --> | Zipping/unzipping .zip archives.

<h2 id="unsound">Unsound</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [actix-web](reviews/actix-web.md) | [![crev-dangerous]](reviews/actix-web.md) | [docs.rs](https://docs.rs/actix-web) <!-- [lib.rs](https://lib.rs/crates/actix-web) --> | AVOID.  Closes soundness bugs unfixed.  Deletes external soundness bugs.
| [cargo-apk](reviews/cargo-apk.md) | [![crev-negative]](reviews/cargo-apk.md) | [docs.rs](https://docs.rs/cargo-apk) <!-- [lib.rs](https://lib.rs/crates/cargo-apk) --> | Glue code is full of unsafe and unsound.
| [crossterm](reviews/crossterm.md) | [![crev-negative]](reviews/crossterm.md) | [docs.rs](https://docs.rs/crossterm) <!-- [lib.rs](https://lib.rs/crates/crossterm) --> | Cross-platform console stuff.  No web support, soundness issues.
| [crossterm_cursor](reviews/crossterm_cursor.md) | [![crev-negative]](reviews/crossterm_cursor.md) | [docs.rs](https://docs.rs/crossterm_cursor) <!-- [lib.rs](https://lib.rs/crates/crossterm_cursor) --> | Cross-platform console cursor maniulation. Needs soundness fixes.
| [crossterm_input](reviews/crossterm_input.md) | [![crev-negative]](reviews/crossterm_input.md) | [docs.rs](https://docs.rs/crossterm_input) <!-- [lib.rs](https://lib.rs/crates/crossterm_input) --> | Cross-platform console input reading. Needs soundness fixes.
| [crossterm_screen](reviews/crossterm_screen.md) | [![crev-positive]](reviews/crossterm_screen.md) | [docs.rs](https://docs.rs/crossterm_screen) <!-- [lib.rs](https://lib.rs/crates/crossterm_screen) --> | 
| [crossterm_style](reviews/crossterm_style.md) | [![crev-negative]](reviews/crossterm_style.md) | [docs.rs](https://docs.rs/crossterm_style) <!-- [lib.rs](https://lib.rs/crates/crossterm_style) --> | 
| [egl](reviews/egl.md) | [![crev-dangerous]](reviews/egl.md) | [docs.rs](https://docs.rs/egl) <!-- [lib.rs](https://lib.rs/crates/egl) --> | AVOID.  Unsound as fuck, abandoned.  See khronos-egl for a sounder, maintained fork.
| [memalloc](reviews/memalloc.md) | [![crev-dangerous]](reviews/memalloc.md) | [docs.rs](https://docs.rs/memalloc) <!-- [lib.rs](https://lib.rs/crates/memalloc) --> | Super brittle/dangerous at a fundamental level. Avoid.
| [microprofile](reviews/microprofile.md) | [![crev-dangerous]](reviews/microprofile.md) | [docs.rs](https://docs.rs/microprofile) <!-- [lib.rs](https://lib.rs/crates/microprofile) --> | Bindings for a C++ flamegraph profiler

<h2 id="web">Web</h2>

| Crate | Review | Links | Description |
| ----- | ------ | ----- | ----------- |
| [cargo-web](reviews/cargo-web.md) | [![crev-none]](reviews/cargo-web.md) | [docs.rs](https://docs.rs/cargo-web) <!-- [lib.rs](https://lib.rs/crates/cargo-web) --> | .wasm packager for use with stdweb
| [instant](reviews/instant.md) | [![crev-positive]](reviews/instant.md) | [docs.rs](https://docs.rs/instant) <!-- [lib.rs](https://lib.rs/crates/instant) --> | std::time::Instant alternative that doesn&#39;t panic on wasm targets.
| [js-sys](reviews/js-sys.md) | [![crev-none]](reviews/js-sys.md) | [docs.rs](https://docs.rs/js-sys) <!-- [lib.rs](https://lib.rs/crates/js-sys) --> | Browser API interop
| [rocket](reviews/rocket.md) | [![crev-none]](reviews/rocket.md) | [docs.rs](https://docs.rs/rocket) <!-- [lib.rs](https://lib.rs/crates/rocket) --> | Rust library for creating API servers.  Nice design.
| [stdweb](reviews/stdweb.md) | [![crev-none]](reviews/stdweb.md) | [docs.rs](https://docs.rs/stdweb) <!-- [lib.rs](https://lib.rs/crates/stdweb) --> | Browser API interop
| [wasm-pack](reviews/wasm-pack.md) | [![crev-none]](reviews/wasm-pack.md) | [docs.rs](https://docs.rs/wasm-pack) <!-- [lib.rs](https://lib.rs/crates/wasm-pack) --> | .wasm packager for use with web-sys
| [web-sys](reviews/web-sys.md) | [![crev-none]](reviews/web-sys.md) | [docs.rs](https://docs.rs/web-sys) <!-- [lib.rs](https://lib.rs/crates/web-sys) --> | Browser API interop



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
