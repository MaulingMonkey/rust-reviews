Handle basic #include s for GLSL.

Pros:
* 100% Safe/Sandboxed

Cons:
* No `#file` directives emitted
* No callback option for `#include`, must pre-register all possible includes.
* Repeat #include s simply ignored instead of interacting with preprocessor.

0.3.1
=====
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | high
| rating        | positive

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| src/error.rs                                  | +1 | Display for Error not double-clickable but provides good context.
| src/lib.rs                                    |  0 | Doesn't disambiguate quote style, no callbacks so you must pre-define every includable file.
| .cargo_vs_info.json                           | +1 | |
| .cargo-ok                                     | +1 | |
| Cargo.toml                                    | +1 | Apache 2.0 or MIT |
| Cargo.toml.orig                               | +1 | regex, lazy_static, \[dev\] indoc, criterion
| LICENSE-APACHE                                | +1 | |
| LICENSE-MIT                                   | +1 | |
| README.md                                     | +1 | Apache 2.0 or MIT, Contributions section

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1 | None, 100% safe
| miri      |
| fs        | +1 | None, instead you need to pre-`include(path, into_string)`.
| io        | +1 | |
| docs      | +1 | |
| tests     | +1 | Not in .crate, but appears to be there/fine in original repository
