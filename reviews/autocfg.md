---
category:       Build Utility
description:    Runs `rustc` to test for features / versions.
---

# autocfg

| version | thoroughness | understanding | rating | Notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.1.7](#0.1.7) | medium | medium | positive | |
| [0.1.6](#0.1.6) | medium | medium | positive | |
| [0.1.5](#0.1.5) | medium | medium | positive | Full review

0.1.7
=====

LGTM, starts using RUSTFLAGS

0.1.6
=====

Reads env vars and executes the program in them, or whatever program happens to
be called `rustc`... but given that this thing's purpose is to probe rustc
versions, that's kinda inevitable.  Abuseable but certainly not malicious; it's
made for build scripts and it's fine for this purpose.

0.1.5
=====

* No unsafe code
* Minor safe-looking file I/O
