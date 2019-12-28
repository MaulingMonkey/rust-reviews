---
category:       FFI
description:    System calls for the Rust OS, Redox
---

# redox_syscall

# Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.56]  | low | low | negative | Full Review

[0.1.56]: #0156

# 0.1.56

Exposes unsound APIs, lots of unverified syscalls.

# Reviewed

| file                      | rating                | notes |
| ------------------------- | --------------------- | ----- |
| src\arch\*.rs             | :grey_question:       | Skimmed... looks reasonable, but didn't verify correct instructions / register invalidation.
| | | |
| src\io\dma.rs             | :grey_question:       | Some unsafe... looks correct, but not thoroughly tested.
| src\io\io.rs              | :heavy_check_mark:    | |
| src\io\mmio.rs            | :exclamation:         | UNSOUND (can construct uninitialized() T via \"safe\" `Mmio::new()`!)
| src\io\mod.rs             | :heavy_check_mark:    | |
| src\io\pio.rs             | :grey_question:       | Some unsafe... looks reasonable, but didn't verify correct instructions.
| | | |
| src\scheme\generate.sh    | :heavy_check_mark:    | |
| src\scheme\mod.rs         | :heavy_check_mark:    | |
| src\scheme\scheme*.rs     | :exclamation:         | UNSOUND (can construct arbitrary slices from arbitrary Packet s via `Scheme*::handle`)
| | | |
| .cargo_vcs_info.json      | :heavy_check_mark:    | |
| .cargo-ok                 | :heavy_check_mark:    | |
| .gitlab-ci.yml            | :heavy_check_mark:    | |
| Cargo.toml                | :heavy_check_mark:    | |
| Cargo.toml.orig           | :heavy_check_mark:    | |
| LICENSE                   | :heavy_check_mark:    | |
| README.md                 | :heavy_check_mark:    | |

# Skimmed

| file                  | notes |
| --------------------- | ----- |
| src\call.rs           | Lots of unsafe syscalls... unverified.
| src\data.rs           | UNSOUND (Map deref etc.)
| src\error.rs          | No tests for STR_ERROR, but at least it's sound.
| src\flag.rs           | Sound, magic constant city, meh.
| src\lib.rs            | LEAKS UNSOUND TRAITS into public interface!
| src\number.rs         | Safe, magic constant city.
| src\tests.rs          | Unsafe, but only #[test]s

# Issues

* [redox-os/syscall#29](https://gitlab.redox-os.org/redox-os/syscall/issues/29)
