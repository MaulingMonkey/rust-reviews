---
category:       I/O
description:    Some extra filesystem utilities
---

# fs2

# Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.4.3]   | medium | medium | positive | Full Review

[0.4.3]: #043

# 0.4.3

I'm assuming GetVolumePathNameW will leave buffer null terminated if successful.  Unsafe only used for sane-looking FFI, and zeroing structs.
