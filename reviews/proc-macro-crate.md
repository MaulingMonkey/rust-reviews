---
category:       Macros
description:    $crate for proc macros (prefer shim macros per review notes!)
crev:           neutral
---

# proc-macro-crate

$crate for proc macros

**Prefer shim macros where possible** (they're less brittle / more portable - thank [dtolnay](https://github.com/MaulingMonkey/rust-reviews/issues/71#issuecomment-601514580) for the suggestion!):

```rust
#[doc(hidden)] pub use impl_crate::my_macro_impl;

#[macro_export] macro_rules! my_macro {
    ($($args:tt)*) => {
        $crate::my_macro_impl!{$crate $($args)*}
    };
}
```

```rust
#[proc_macro] pub fn my_macro_impl(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let my_crate = tokens.next().unwrap();
    ...
}
```

Where not possible, consider hardcoding a fallback for non-cargo build environments such as Bazel instead of panicing (since proc_macro_crate relies on parsing Cargo.toml):

```rust
let my_crate = proc_macro_crate::crate_name("my-crate").unwrap_or("my_crate".to_string());
```

## Audit

| version | thoroughness | understanding | rating | Notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.1.4] | medium | medium | positive | Diff
| [0.1.3] | medium | medium | positive | Diff
| [0.1.2] | medium | medium | positive | Diff
| [0.1.1] | medium | medium | positive | Diff
| [0.1.0] | medium | medium | positive | Full review

[0.1.4]:    #014
[0.1.3]:    #013
[0.1.2]:    #012
[0.1.1]:    #011
[0.1.0]:    #010

## 0.1.4

* Version bumps, typos, formatting
* Start searching `[target.*.dependencies]` too

## 0.1.3

Dropped `'static` lifetime requirements

## 0.1.2

- Added syn, proc-macro2 deps
- Sanitized "-" to "_"

## 0.1.1

Minor metadata tweaks

## 0.1.0

| file                  | rating | notes |
| --------------------- | ------ | ----- |
| src/lib.rs            | :heavy_check_mark: | |
| .cargo-ok             | :heavy_check_mark: | |
| .gitignore            | :heavy_check_mark: | |
| .travis.yml           | :heavy_check_mark: | MSRV: stable
| Cargo.toml            | :heavy_check_mark: | Apache-2.0/MIT |
| Cargo.toml.orig       | :heavy_check_mark: | Apache-2.0/MIT |
| README.md             | :heavy_check_mark: | Apache-2.0 OR MIT


| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | :heavy_check_mark: | None
| fs        | :heavy_check_mark: | Opens `%CARGO_MANIFEST_DIR%\Cargo.toml` for read only
| io        | :heavy_check_mark: | Bulk parsing outsourced to toml crate
| docs      | :heavy_check_mark: | Straightforward
| tests     | :heavy_check_mark: | |
