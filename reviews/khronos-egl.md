---
category:       Graphics
description:    EGL bindings - provides OpenGL (ES) contexts
---

# khronos-egl

A much safer looking, maintained fork of the `egl` crate.

Pros:
* A "Sound" API (not fully audited but looks way better than `egl`s "safe" fns taking raw pointers)
* API Fixes

Cons:
* `build.rs` doesn't like cross compiling as of `2.0.0`

Alternatives:
* <code>[egli](https://lib.rs/crates/egli)</code> is what I'm using (another supposedly sound crate, with low and high level APIs - cross compiles OK as well.)
* <code>[glutin_egl_sys](https://lib.rs/crates/glutin_egl_sys)</code> uses `gl_generator` to provide low level `unsafe` API structs.
