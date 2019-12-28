---
category:       FFI
description:    Generate Rust wrappers around C types
---

# foreign-types

Generates wrapper types / boilerplate around C types for you.
Like all FFI, this is a big footgun, but it might work for your needs.

# Audit

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.5.0] | none   | medium | neutral  | [sfackler/foreign-types/#10]
| [0.4.0] | low    | medium | negative | Unsound
| [0.3.2] | medium | medium | negative | Unsound, Full Review |

# 0.5.0

I need to take another proper look at this crate, but [sfackler/foreign-types/#10] at least addressed my main concern.

# 0.4.0 / 0.3.2

foreign_type! can generate unsound code without using the `unsafe` keyword in client code at all:

- Invokes C FFI which may be unsafe from safe fns.
- More recent versions (0.4.0) can implement unsafe traits (Send, Sync) without the unsafe keyword at all.



[0.5.0]:        #050
[0.4.0]:        #040--032
[0.3.2]:        #040--032

[sfackler/foreign-types/#10]:   https://github.com/sfackler/foreign-types/issues/10
