---
category:       Async
description:    Convert JS `Promise`s to/from Rust `Future`s
---

# wasm_bindgen_futures

Convert JS `Promise`s to/from Rust `Future`s

* [wasm_bindgen_futures](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/)
* [docs.rs](https://docs.rs/wasm-bindgen-futures/0.4.17/wasm_bindgen_futures/)

## Alternatives

* For stdweb, use `stdweb::spawn_local` instead (gated behind `features = ["experimental_features_which_may_break_on_minor_version_bumps", "futures-support"]`)
