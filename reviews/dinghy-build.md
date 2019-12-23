---
category:       Android
description:    
---

# dinghy-build

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
|  0.4.15           | medium | low | positive | Trivial version bumps.
|  0.4.14           | medium | low | positive | Trivial version bumps, style changes.
|  0.4.13           | medium | low | positive | Trivial version bumps.
|  0.4.12           | medium | low | positive | Trivial version bumps.
| [0.4.11](#0.4.11) | medium | low | positive | Full review

# 0.4.11

Some of the build/path stuff seems a little off... but might be correct?
All safe code, no security problems.

Reviewed:
```
src\build_env.rs    +1
src\build.rs        +1
src\lib.rs          0
    102 Is this really correct for specifying the *host* environment?
    105 ..
src\utils.rs        0
    14  Isn't this generating /../../../ ?  Doesn't seem right...

Cargo.toml          +1
Cargo.toml.orig     +1
```
