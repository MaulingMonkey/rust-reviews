---
category:       Unsound
crev:           dangerous
description:    AVOID.  Unsound as fuck, abandoned.  See khronos-egl for a sounder, maintained fork.
---

# egl

<span style="color: red; font-weight: bold;">AVOID.</span>  This crate is unsound as fuck, and abandoned.

Alternatives:
* <code>[khronos-egl](https://lib.rs/crates/khronos-egl)</code> is a sounder, maintained fork.
* <code>[egli](https://lib.rs/crates/egli)</code> is an alternative, supposedly sound API with low and high level APIs.
* <code>[glutin_egl_sys](https://lib.rs/crates/glutin_egl_sys)</code> uses `gl_generator` to provide low level `unsafe` API structs.

Unsoundness:
* Tons of "safe" functions taking raw pointers that will get used.
* Tons of "safe" functions taking arrays that are expected to be `NONE`-terminated, but are unchecked.
* `choose_config` as a concrete example has no less than 3 soundness bugs and a mismapped API shape to boot.  From [src/lib.rs](https://github.com/seankerr/rust-egl/blob/36d88bb45c8005d3424de971b0c10cce87dea157/src/lib.rs#L292-L315):
    ```rust
    pub fn choose_config(display: EGLDisplay, attrib_list: &[EGLint], // UNSOUND: `eglChooseConfig` expects a valid display pointer or various magic constants, but `display` isn't validated.
                        config_size: EGLint) -> Option<EGLConfig> {
        unsafe {
            let mut config: EGLConfig = ptr::null_mut(); // UNSOUND: `eglChooseConfig` expects `[EGLConfig; config_size]`, not `[EGLConfig; 1]`.
            let mut count:  EGLint = 0;

            let attribs = if attrib_list.len() > 0 {
                attrib_list.as_ptr() // UNSOUND: `eglChooseConfig` expects this to be terminated with `EGL_NONE`, but this isn't enforced.
            } else {
                ptr::null()
            };

            if ffi::eglChooseConfig(display, attribs, &mut config, config_size,
                                    &mut count) == EGL_TRUE {
                if count > 0 {
                    Some(config)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
    ```
