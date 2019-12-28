---
category:       FFI
description:    Reduced capability-based POSIX subset/alternative.
---

# cloudabi

# Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.0.3]   | low | medium | neutral | |

[0.0.3]: #003

# 0.0.3

Interesting idea.  Lots of unsafe!  Supposedly auto-generated.
I have at least read the code enough (including explicitly looking at each use of unsafe)
to believe there are no obvious funny business nor proper soundness issues.  That said:

Concerns (rating: neutral):
- Lots of concerning use of `unsafe { ... }` + `core::mem::uninitialized()`... but only in `#[test]` code.
- `#[allow(improper_ctypes)]` for `extern "C"` block.  Sounds like UB-bait.
- Punts all safety concerns to the user.

Concerns (thoroughness: low):
- I have not properly vetted fn/struct/type signatures against a proper reference.
