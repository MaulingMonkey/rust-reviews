---
category:       General Utility
description:    std::num::NonZero___ equivalents
msrv:           1.46.0
---

# nonmax

std::num::NonZero___ equivalents

[ [github](https://github.com/lpghatguy/nonmax)
• [docs.rs](https://docs.rs/nonmax/)
• [lib.rs](https://lib.rs/crates/nonmax)
]

Pros:
* Works
* Can represent 0
* Clever cheap xors for implementation

Cons:
* Missing [a lot of traits](https://doc.rust-lang.org/std/num/struct.NonZeroI32.html#trait-implementations) vs equivalent `NonZero*`s
* Extra pointless unsafe in `new` impl

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.4.0] | high | high | ✔️ positive | `const` spam, MSRV `1.46.0`
| [0.3.0] | high | high | ✔️ positive | Fix NonMax\[UI\]size casing (breaking change)
| [0.2.0] | high | high | ✔️ positive | `MIT OR Apache-2.0`, MSRV `1.34.1`

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         ❌ dangerous ⚠️❗️ negative ❔ neutral ✔️ positive ✔️ strong
-->

[0.4.0]: #0.4.0
[0.3.0]: #0.3.0
[0.2.0]: #0.2.0

<h2 name="0.4.0">0.4.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| <span>.</span>github\workflows\ci<span>.</span>yml        | ✔️ | MSRV `1.46.0`
| CHANGELOG<span>.</span>md                                 | ✔️
| README<span>.</span>md                                    | ✔️
| src\lib<span>.</span>rs                                   | ✔️

<h2 name="0.3.0">0.3.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| CHANGELOG<span>.</span>md                                 | ✔️
| README<span>.</span>md                                    | ✔️ | Fix NonMax\[UI\]size casing (breaking change)
| src\lib<span>.</span>rs                                   | ✔️ | Fix NonMax\[UI\]size casing (breaking change)

<h2 name="0.2.0">0.2.0</h2>

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| <span>.</span>github\workflows\ci<span>.</span>yml        | ✔️ | MSRV `1.34.1`
| <span>.</span>gitignore                                   | ✔️
| CHANGELOG<span>.</span>md                                 | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | `MIT OR Apache-2.0`
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | `MIT OR Apache-2.0`
| LICENSE-APACHE                                            | ✔️
| LICENSE-MIT                                               | ✔️
| README<span>.</span>md                                    | ✔️ | MSRV `1.34.1`, `MIT OR Apache-2.0`
| README<span>.</span>tpl                                   | ✔️ | ? `MIT OR Apache-2.0`
| src\lib<span>.</span>rs                                   | ⚠️ | LGTM, although contains some unsafe

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ⚠️ | `new` could be implemented safely & sanely in terms of nonzero new.  `new_unchecked`, wrapping nonzero `new_unchecked` appropriately
| fs        | ✔️ | None
| io        | ✔️ | None
| docs      | ⚠️ | `new_unchecked` should document safety preconditions
| tests     | ⚠️ | `new` should test failure conditions, `new_unchecked` untested

<!-- Templates

✔️❔⚠️❗️❌

#### :exclamation:  \[1\] Unsound ...
#### \[1\] Note ...
[1]: #exclamation--1-unsound-...
[2]: #1-note-...
[user/repository#1]: https://github.com/user/repository/issues/1
[user/repository#1]: https://github.com/user/repository/pull/1



# DiffVersionTemplate

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| 

# Full File Version Template

| Line  | Notes |
| -----:| ----- |
| 

-->
