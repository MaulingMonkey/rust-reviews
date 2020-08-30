---
category:       Build Utility
description:    Embed resources (icons, versions, ...) into your executables.
---

# winres

Embed resources (icons, manifests, version strings, ...) into your executables.

Pros:
* Excellent!
* Easy `build.rs` integration

Cons:
* Awkward to use outside of a `build.rs` context.

## Issues

| issue     | severity  | broke     | fix       | desc |
| --------- | --------- | --------- | --------- | ---- |
| [#11]     | ⚠️ medium | [0.1.0]   | [0.1.7]   | Quotes not properly escaped
| #NA1      | ⚠️ medium | [0.1.0]   | [0.1.8]   | Non-quotes not properly escaped?
| [#25] 🐵 | ⚠️ medium | [0.1.0]   |           | VERSIONINFO strings not null terminated?

I'd normally consider escaping issues to be high severity bugs - however, these
are all dev controlled, and should typically only lead to build errors.
[#25] is the most concerning - however, I'm not even sure I can construct a sane
demo of unsoundness, and I'm not even sure if it's a bug.  Getting a raw
C-string seems like a PITA.

[#11]:  https://github.com/mxre/winres/issues/11
[#25]:  https://github.com/mxre/winres/issues/25

## Audit

| version   | thoroughness | understanding  | rating    | notes |
| --------- | ------------ | -------------- | --------- | ----- |
| [0.1.11]  | low          | medium         | positive  | More explicit string escaping
| [0.1.10]  | low          | medium         | positive  |
| [0.1.9]   | low          | medium         | positive  |
| [0.1.8]   | low          | medium         | positive  | Fixed: ~~[#NA1]~~
| [0.1.7]   | low          | medium         | neutral   | Fixed: ~~[#11]~~
| [0.1.6]   | low          | medium         | neutral   |
| [0.1.5]   | low          | medium         | neutral   |
| [0.1.4]   | low          | medium         | neutral   |
| [0.1.3]   | low          | medium         | neutral   |
| [0.1.1]   | low          | medium         | neutral   |
| [0.1.0]   | low          | medium         | neutral   | Full review.  **[#11]** **#NA1** **[#25]**

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[0.1.11]: #0.1.11
[0.1.10]: #0.1.10
[0.1.9]: #0.1.9
[0.1.8]: #0.1.8
[0.1.7]: #0.1.7
[0.1.6]: #0.1.6
[0.1.5]: #0.1.5
[0.1.4]: #0.1.4
[0.1.3]: #0.1.3
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.1.11">0.1.11</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| lib.rs                            | ✔️ | More explicit string escaping

<h2 name="0.1.10">0.1.10</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| lib.rs                            | ✔️ | + path configuration options

<h2 name="0.1.9">0.1.9</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| lib.rs                            | ✔️ | +`set_icon_with_id`, `reg ... /reg:32`

<h2 name="0.1.8">0.1.8</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| lib.rs                            | ⚠️ | Fix non-quote escaping

<h2 name="0.1.7">0.1.7</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo_vcs_info.json              | ✔️
| .gitignore                        | ✔️ | workspaceless example?
| Cargo.toml                        | ✔️ | docs.rs link
| Cargo.toml.orig                   | ✔️ | docs.rs link
| lib.rs                            | ⚠️ | Fix quote escaping

<h2 name="0.1.6">0.1.6</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| README.md                         | ✔️
| lib.rs                            | ✔️

<h2 name="0.1.5">0.1.5</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .gitignore                        | ✔️
| Cargo.toml                        | ✔️ | `winapi 0.2` -> `0.3`
| Cargo.toml.orig                   | ✔️ | `winapi 0.2` -> `0.3`
| lib.rs                            | ✔️

<h2 name="0.1.4">0.1.4</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | `toml 0.1` -> `0.4`
| lib.rs                            | ✔️

<h2 name="0.1.3">0.1.3</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .editorconfig                     | ✔️
| README.md                         | ✔️
| lib.rs                            | ✔️ | `try!` -> `?`, docs, log rc.exe output, more paths

<h2 name="0.1.2">0.1.2</h2>

Nonexistant/skipped

<h2 name="0.1.1">0.1.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | + `winapi 0.2` (dev)
| README.md                         | ✔️
| lib.rs                            | ✔️ | Docs, more kit paths

<h2 name="0.1.0">0.1.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| .gitignore                        | ✔️
| Cargo.toml                        | ✔️ | `MIT`, `toml`
| LICENSE                           | ✔️ | `MIT`?
| README.md                         | ✔️
| [lib.rs](#0.0.1/lib.rs)           | ⚠️ | Strings aren't `\0` terminated... not sure if that's problematic?
| test.ico                          | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | Reasonable, relies on some reg keys / cargo env paths
| io        | ✔️ | Reasonable
| docs      | ✔️
| tests     | ✔️ | Via doc comments


<h2 name="0.0.1/lib.rs">lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 348 | ⚠️ Should `v` be `\0` terminated like https://docs.microsoft.com/en-us/windows/win32/menurc/versioninfo-resource shows?
| 357 | ⚠️ What about icons?
| 364 | ⚠️ Or manifest paths?
| 397 | ✔️ Command inputs reasonable
| 408 | ✔️ Command inputs reasonable
| 434 | ✔️ I/O paths reasonable
| 461 | ✔️ Command inputs reasonable
| 481 | ✔️ Command inputs reasonable
| 502 | ✔️ I/O paths reasonable

Minor notes:
* Non-UTF8 paths will panic
* Will attempt to use x86 RC on ARM windows

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
