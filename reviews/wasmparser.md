---
category:       Debugging
description:    `.wasm` file parser
---

# wasmparser

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.15.3](#0.15.3) | low | medium | positive |

# 0.15.3

Looks great overall.
I haven't double-checked any logic against WASM specs.
I haven't verified that WASM validation is suitable to rely upon for JIT compilers or the like.

Detail
------

| File                  | Rating | Notes |
| --------------------- | ------ | ----- |
| examples/dump.rs      | +1    | io (safe) |
| examples/simple.rs    | +1    | io (safe) |
| fuzz/*                |       | UNREVIEWED (excluded from crate) |
| src/lib.rs            | +1    | |
| src/limits.rs         | +1    | |
| src/parser.rs         | +1    | |
| src/tests.rs          | +1    | io (safe) |
| src/validator.rs      | +1    | check_utf8 could be mostly replaced with stdlib? |
| tests/*.wasm          |       | Unreviewed... nothing but WASM though, should be OK |
| .gitignore            | +1    | |
| .travis.yml           | +1    | |
| Cargo.toml            | +1    | |
| Cargo.toml.orig       | +1    | |
| check-rustfmt.sh      | +1    | Globally overwrites rustfmt with specific version |
| format-all.sh         | +1    | |
| LICENSE               | +1    | Apache 2.0 |
| Readme.md             | +1    | |
| test-all.sh           | +1    |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | No unsafe code.
| fs        | +1    | Examples and tests only, reasonably used.
