---
category:       Serialization
description:    Generate .ico / .icns
---

# icon_baker

Generate .ico / .icns

Pros:
* Rescaling via `image`
* SVG rendering via `resvg`
* Pretty nifty and relatively comprehensive

Cons:
* Unsound

## Issues

| issue                     | severity  | broke     | fix      | desc |
| ------------------------- | --------- | --------- | -------- | ---- |
| [#11]🐵                   | ❗️ high    | <= [3.2.0]🐵 |       | `unsafe impl Send/Sync for Image {}` is unsound (can clone Rc from multiple threads)

[#11]:                  https://github.com/GarkGarcia/icon_baker/issues/11

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [3.2.0] | medium | medium | dangerous | unsound

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[3.2.0]: #3.2.0

<h2 name="3.2.0">3.2.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| .cargo_vcs_info.json              | ✔️
| .gitignore                        | ✔️
| Cargo.toml                        | ✔️ | `LICENSE`
| Cargo.toml.orig                   | ✔️ | `LICENSE`
| LICENSE                           | ✔️ | `MIT`
| README.md                         | ✔️ | Excellent
| src\encode.rs                     | ✔️
| src\favicon.rs                    | ⚠️ | String-based JSON construction is vaguely annoying to audit for user-controlled data
| src\icns.rs                       | ✔️
| src\ico.rs                        | ✔️
| src\lib.rs                        | ❗️ | [#11] `unsafe impl Send/Sync for Image {}` is unsound (can clone Rc from multiple threads)
| src\resample.rs                   | ✔️
| src\test.rs                       | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗️      | [#11] `unsafe impl Send/Sync for Image {}` is unsound (can clone Rc from multiple threads)
| fs        | ✔️    | Reasonable user provided paths, and the occasional path-based extension / index
| io        | ✔️    | Reasonable
| docs      | ✔️    | Reasonable
| tests     | ❔     | Partially excluded from .crate
