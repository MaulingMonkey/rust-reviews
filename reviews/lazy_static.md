---
category:       General Utility
description:    Static init at runtime.
---

# lazy_static


| version           | thoroughness | understanding | rating | notes |
| ----------------- | ------------ | ------------- | ------ | ----- |
| [1.4.0](#140) | high | high | positive | |
| [1.3.0](#130) | high | high | positive | Full review
| [0.2.11](#0211) | medium | medium | negative | Unsound, race conditions

## 1.4.0

Read diff, looks fine.

## 1.3.0

Read all of src, skimmed all of tests.  core_lazy.rs looks a little odd, but is 100% safe code - any issues would be in it's core dependency, spin.  inline_lazy.rs contains unsafe blocks... look safe, but downgrades rating to merely positive.  lib.rs is just safe macros.  Tests all pass.

## 0.2.11

lazy_static 0.2.11 relied on unsafe muts which would have race conditions during initialization.
I recommend upgrading to a modern lazy_static 1.3.0+!

| file                  | rating | notes |
| --------------------- | ------ | ----- |
| src\core_lazy.rs      | +1 | |
| src\lazy.rs           | -1 | UNSOUND (race conditions in multithreaded init due to static mut / mut refs taken outside of call_once)
| src\lib.rs            | -1 | UNSOUND (uses the static muts without syncronization)
| src\nightly_lazy.rs   | -1 | UNSOUND (race conditions in multithreaded init due to static mut / mut refs taken outside of call_once)
| | | |
| tests\\*              | +1 | (skimmed, seem fine)
| | | |
| .travis.yml           | +1 |
| .appveyor.yml         | +1 |
| Cargo.toml            | +1 |
| Cargo.toml.orig       | +1 |
| README.md             | +1 |
