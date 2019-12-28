---
category:       General Utility
description:    Decentralized static registration
msrv:           1.31.0
---

# inventory

Excellent crate that allows you to decentralize static registration.  Two caveats:

* It's currently unsound \[[1],[2]\].
* It doesn't work with WASM targets.

The author has shown interest in adding WASM support, and has attempted to find a way,
but rust/WASM simply don't have the link_section stuff to abuse (yet?)

# Audit

| version | thoroughness | understanding | rating | notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.1.5](#015) :heavy_check_mark: | high | high | positive | [dtolnay/inventory#15]
| [0.1.4](#014) :exclamation: | high | high | negative | Unsound! \[[1],[2]\]

# 0.1.5

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| .cargo_vcs_info.json  | :heavy_check_mark: | |
| Cargo.lock            | :heavy_check_mark: | version bumps for ctor, ghost, inventory, inventory-impl, proc-macro2, quote, syn, unicode-xid
| Cargo.toml            | :heavy_check_mark: | Version bumps, badges
| Cargo.toml.orig       | :heavy_check_mark: | Version bumps, badges
| src/lib.rs            | :heavy_check_mark: | Merges soundness fixes [dtolnay/inventory#15]

# 0.1.4

| file                  | rating | notes |
| --------------------- | ------ | ----- |
| examples/flags.rs     | :heavy_check_mark: | |
| src/lib.rs            | :exclamation: | Unsound! \[[1],[2]\]
| .cargo_vcs_info.json  | :heavy_check_mark: | |
| .cargo-ok             | :heavy_check_mark: | |
| .gitignore            | :heavy_check_mark: | |
| .travis.yml           | :heavy_check_mark: | Tests 1.31.0, stable, beta, nightly
| Cargo.lock            | :heavy_check_mark: | Depends on, ctor, ghost, inventory-impl, 2x proc-macro2, 2x quote, 2x syn, 2x unicode-xid
| Cargo.toml            | :heavy_check_mark: | `license = "MIT OR Apache-2.0"`
| Cargo.toml.orig       | :heavy_check_mark: | `license = "MIT OR Apache-2.0"`
| LICENSE-APACHE        | :heavy_check_mark: | Skimmed
| LICENSE-MIT           | :heavy_check_mark: | |
| README.md             | :heavy_check_mark: | MSRV 1.31.0, MIT OR Apache-2.0


| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | :exclamation: | Unsound! \[[1],[2]\]
| fs        | :heavy_check_mark: | None |
| io        | :heavy_check_mark: | None |
| docs      | :heavy_check_mark: | Excellent |
| tests     | :heavy_check_mark: | Pretty good, although I'd add one test to ensure all values are found. |

#### :exclamation:  \[1\] Unsound [fn Registry\<T\>::submit](https://github.com/dtolnay/inventory/blob/31b0974e6ab749967ee3506d166302c5a138221c/src/lib.rs#L154-L160) race conditions

`new.next` is set *after* `new` has been inserted into the registry.
* Every `unsafe { prev.as_ref() }` possibly returns a `&Node<T>` to our `&mut Node<T>` use, which is Undefined Behavior.
* Even if it weren't UB, this would temporarilly truncate the registry.

This should be fixable ([dtolnay/inventory#15]):
* Set `new.next = unsafe { head.as_ref() };` unconditionally, in the loop, *before* the CAS, to fix the race condition.
* ~~Downgrade `&mut Node` to `&Node` before the CAS~~ Use a `ptr::NonNull` and only construct `&mut Node` when not visible.
* Make `Registry::submit` do the box leaking to make the API locally sound.

Undefined Behavior is also hard to trigger in practice - requiring submitting or iterating over the registry from a new (pre-main?) thread, mid-registration.

#### :exclamation:  \[2\] Unsound `unsafe { prev.as_ref() }` blocks

* [fn Registry\<T\>::submit](https://github.com/dtolnay/inventory/blob/0.1.4/src/lib.rs#L159-L160)
* [fn \<iter as IntoIterator\>::into_iter](https://github.com/dtolnay/inventory/blob/0.1.4/src/lib.rs#L222-L223)

Per the race condition \[[1]\], these are unsound.
Aside from that, these would otherwise be sound, as the comments are correct.
Head will always be null (maps to `None`) or a valid instance (`Some(&Node<T>)`).



[1]: #exclamation--1-unsound-fn-registrytsubmit-race-conditions
[2]: #exclamation--2-unsound-unsafe--prevas_ref--blocks
[dtolnay/inventory#15]: https://github.com/dtolnay/inventory/pull/15
