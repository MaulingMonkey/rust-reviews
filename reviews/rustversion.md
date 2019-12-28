---
category:     General Utility
description:  Attributes to do conditional compilation based on rust version/channel
---

# rustversion

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [1.0.0](#100) | high | medium | positive | Full Review

# 1.0.0

| file                  | rating | notes |
| --------------------- | ------ | ----- |
| src/attr.rs           | :heavy_check_mark: | |
| src/bound.rs          | :heavy_check_mark: | |
| src/date.rs           | :heavy_check_mark: | Custom parse impls
| src/expr.rs           | :heavy_check_mark: | Custom keywords
| src/lib.rs            | :heavy_check_mark: | Custom HTML docs, error handling
| src/rustc.rs          | :heavy_check_mark: | Command \[[1](#note-1--pub-fn-version-uses-command)\]
| src/time.rs           | :grey_question:    | Parsing date times seems a bit out-of-scope for this crate.  OTOH, this is only used for an error message example by date.rs.
| src/version.rs        | :heavy_check_mark: | |
| .cargo_vcs_info.json  | :heavy_check_mark: | |
| .cargo-ok             | :heavy_check_mark: | |
| .gitignore            | :heavy_check_mark: | |
| .travis.yml           | :heavy_check_mark: | Tests 1.31.0 (MSRV?), stable, beta, nightly
| Cargo.lock            | :heavy_check_mark: | Recent-looking proc-macro2, quote, syn, unicode-xid |
| Cargo.toml            | :heavy_check_mark: | `license = "MIT OR Apache-2.0"`
| Cargo.toml.orig       | :heavy_check_mark: | `license = "MIT OR Apache-2.0"`
| LICENSE-APACHE        | :heavy_check_mark: | Skimmed
| LICENSE-MIT           | :heavy_check_mark: | |
| README.md             | :heavy_check_mark: | MIT OR Apache-2.0

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | :heavy_check_mark: | None
| fs        | :heavy_check_mark: | None
| io        | :heavy_check_mark: | Command \[[1](#note-1--pub-fn-version-uses-command)\]
| docs      | :heavy_check_mark: | Duplicated between readme and src/lib.rs
| tests     | :grey_question:    | Could use more test coverage

#### Note 1:  [pub fn version() uses Command](https://github.com/dtolnay/rustversion/blob/60e155b2f6bd849264d0ffa472248dec6d85e79f/src/rustc.rs#L51-L55)

Executes `%RUSTC% --version`.
`%RUSTC%` is the only user controlled arg, and already set/read/executed by cargo.
Baddies could simply modify `%PATH%` and do worse damage.
LGTM! :+1:

#### Other Takeaways

I really wish I had dived into this crate's src before doing my own proc macros.
`syn` actually looks quite neato, I should probably replace my hand rolled logic in `jni-glue-macros` with it.

I should also consider testing with [-Z minimal-versions](https://github.com/dtolnay/rustversion/blob/3d2bb86e71d025a9013243f2810061edade7cd27/.travis.yml#L12-L17) (may require nightly currently?)
