---
category:       General Utility
description:    Generate .ico / .icns
---

# icon-pie

Generate .ico / .icns

Pros:
* Rescaling via `image`
* SVG rendering via `resvg`
* Pretty nifty and relatively comprehensive
* `icon-pie`'s lack of multithreading makes `icon-pie` sound despite `icon_baker`'s bogus Send+Sync impls

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.4-beta] | low | medium | positive |

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[0.1.4-beta]: #0.1.4-beta

<h2 name="0.1.4-beta">0.1.4-beta</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| .cargo_vcs_info.json              | ✔️
| .gitignore                        | ✔️
| Cargo.lock                        | ❔ | Lots of deps
| Cargo.toml                        | ✔️ | `LICENSE`, `icon_baker`, `crossterm`
| Cargo.toml.orig                   | ✔️ | `LICENSE`, `icon_baker`, `crossterm`
| LICENSE                           | ✔️ | `MIT`
| README.md                         | ✔️
| assets\logo.afphoto               | ❔ | I lack a viewer, could probably be excluded too
| assets\logo.png                   | ✔️
| assets\logo.svg                   | ✔️
| distribution\icon-baker.tar       | ⚠️ | Should be excluded from crate
| distribution\icon-baker.zip       | ⚠️ | Should be excluded from crate
| examples\\*.afphoto               | ❔ | I lack a viewer, could probably be excluded too
| examples\big.svg                  | ✔️
| examples\borders.png              | ✔️
| examples\concepts.png             | ✔️
| examples\default_resample.png     | ✔️
| examples\dhfakfljsafs.png         | ✔️
| examples\example_1.png            | ✔️
| examples\example_2.png            | ✔️
| examples\example_3.png            | ✔️
| examples\small.png                | ✔️
| examples\sources.png              | ✔️
| src\error\mod.rs                  | ✔️
| src\error\show.rs                 | ✔️
| src\eval\mod.rs                   | ✔️
| src\main.rs                       | ✔️ | `VERSION` should use `env!(...)`
| src\parse\mod.rs                  | ✔️
| src\parse\token.rs                | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | Reasonable
| io        | ✔️ | Reasonable
| docs      | ✔️ | Via help/usage docs, github page
| tests     | ❔ | Not in crate

<!-- Templates

✔️❔⚠️❗️

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
