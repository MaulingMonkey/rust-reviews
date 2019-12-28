---
category:       I/O
description:    Unsafe APIs for loading `.so`s, `.dll`s at runtime.
---

Pros:
* Referenced by [ye olde rust FAQ](https://prev.rust-lang.org/en-US/faq.html#how-do-i-do-dynamic-rust-library-loading)
* Popular

Cons:
* Odd license (ISC)
* unconditional `cc` build dependency (for [weak symbol nonsense](https://github.com/nagisa/rust_libloading/blob/master/src/os/unix/global_static.c))
* mostly `unsafe` interface
