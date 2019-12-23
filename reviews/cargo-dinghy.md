---
category:       Android
description:    `cargo` subcommand for building Android/iOS
---

# cargo-dinghy

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
|  0.4.15           | low | medium | positive | Trivial version bumps.
|  0.4.14           | low | medium | positive | Trivial version bumps, mass reformatting.
|  0.4.13           | low | medium | positive | Trivial version bumps.
|  0.4.12           | low | medium | positive | Trivial version bumps.
| [0.4.11](#0.4.11) | low | medium | positive | Full review

# 0.4.11

Nice and solid looking code.  100% safe code.

```
Reviewed:
    src\cli.rs:         +1
    src\main.rs:        +1
        151 run_lldb
            Nonterminating loops are problematic, but I think the side effects here should make this work OK?
            (see https://github.com/rust-lang/rust/issues/28728 )
    Cargo.toml:         +1
    Cargo.toml.orig:    +1
        13  readme path references unpackaged readme

TIL:
    Using traits to extend clap::App so flags can be reused per-subcommand is interesting.
```
