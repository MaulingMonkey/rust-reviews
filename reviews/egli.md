---
category:       Graphics
description:    EGL bindings - provides OpenGL (ES) contexts
---

# egli

Cross compileable **EGL I**nterface crate.

Pros:
* A "Sound" API (not fully audited but looks way better than `egl`s "safe" fns taking raw pointers)
* Raw `egli::ffi` bindings if you want 'em

Cons:
* Hard links against libegl.a

Alternatives:
* <code>[khronos-egl](https://lib.rs/crates/glutin_egl_sys)</code> seems OK, but can't be cross compiled.
* <code>[glutin_egl_sys](https://lib.rs/crates/glutin_egl_sys)</code> uses `gl_generator` to provide low level `unsafe` API structs for soft/dlopen based loading.
