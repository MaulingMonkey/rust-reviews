---
category:       FFI
description:    foreign-types support crate
---

# foreign-types-shared

# Audit

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.1.1]   | medium | low | negative | Full Review

[0.1.1]: #011

# 0.1.1

Technically sound, but makes me nervous.

- `Opaque`: Looks way too dereferencable, and would have the wrong size - that of `()`.
- `ForeignTypeRef`: Unchecked pointer casts galore.
