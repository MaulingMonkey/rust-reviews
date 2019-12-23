---
category:       FFI
description:    Generate C/C++ headers for Rust code
---

# cbindgen

Pros:
* Lets you generate C/C++ headers for whatever C style ABI you expose from rust.
* Can be intergrated into your build.rs scripts with relative ease.

Cons:
* C style ABIs only, with many unhandled edge cases (generates unusable headers
  for tuples, won't export constants using type aliases, etc.), but usable.
* You'll likely need to hand-write some interop types for strings, references, slices, etc.
