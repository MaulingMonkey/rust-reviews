---
category:       I/O
description:    Beautiful cargo-like error reporting
---

# codespan-reporting

Beautiful cargo-like error reporting

Pros:
* Beautifully documented codebase

Neutral:
* Large diffs, constant internal refactoring makes for constant improvements and constant rereviews

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.9.5] | ⚠️ low | medium | positive |
| [0.9.4] | ⚠️ low | medium | positive |
| [0.9.3] | ⚠️ low | medium | positive |
| [0.9.2] | ⚠️ low | medium | positive |
| [0.9.1] | ⚠️ low | medium | positive |
| [0.9.0] | medium  | medium | positive | Major overhaul
| [0.8.0] | medium  | medium | positive |
| [0.7.0] | medium  | medium | positive |
| [0.6.0] | medium  | medium | positive |
| [0.5.0] | medium  | medium | positive |
| [0.4.1] | medium  | medium | positive |
| [0.4.0] | medium  | medium | positive | Massive overhaul
| [0.3.0] | medium  | medium | positive |
| [0.2.1] | medium  | medium | positive |
| [0.2.0] | medium  | medium | positive |
| [0.1.4] | medium  | medium | positive | 
| [0.1.3] | medium  | medium | positive |
| [0.1.2] | medium  | medium | positive |
| [0.1.1] | medium  | medium | positive |
| [0.1.0] | medium  | medium | positive | Full review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative medium positive strong
-->

[0.9.5]: #0.9.5
[0.9.4]: #0.9.4
[0.9.3]: #0.9.3
[0.9.2]: #0.9.2
[0.9.1]: #0.9.1
[0.9.0]: #0.9.0
[0.8.0]: #0.8.0
[0.7.0]: #0.7.0
[0.6.0]: #0.6.0
[0.5.0]: #0.5.0
[0.4.1]: #0.4.1
[0.4.0]: #0.4.0
[0.3.0]: #0.3.0
[0.2.1]: #0.2.1
[0.2.0]: #0.2.0
[0.1.4]: #0.1.4
[0.1.3]: #0.1.3
[0.1.2]: #0.1.2
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

## WTB:
* `Cargo.lock` differ
* `unsafe`/`std`/`io` finder
* mass open?

<h2 name="0.9.5">0.9.5</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| Cargo.lock                        | ✔️ | Version bumps, dirs-* dependency changes
| src\files.rs                      | ✔️
| src\term\config.rs                | ✔️
| src\term\renderer.rs              | ✔️
| src\term\views.rs                 | ✔️
| tests\snapshots\\*.snap           | ✔️
| tests\term.rs                     | ⚠️ | tab tests, existing `unsafe`... in test data only though

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ⚠️ | Only a touch, in test data (a forbid(unsafe_code) would make me happy)
| fs        | ✔️ | None
| io        | ✔️ | via termcolor etc.
| docs      | ✔️ | Decent exterior coverage, excellent internal coverage
| tests     | ✔️ | Excellent

<h2 name="0.9.4">0.9.4</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| Cargo.lock                        | ✔️ | Version bumps
| src\diagnostic.rs                 | ✔️ | + "in bytes" comment
| src\term\config.rs                | ✔️ | + Chars::pointer_left
| src\term\renderer.rs              | ✔️ | note rendering
| src\term\views.rs                 | ✔️
| tests\snapshots\\*.snap           | ✔️ | notes
| tests\term.rs                     | ⚠️ | `unsafe`... in test data only though

<h2 name="0.9.3">0.9.3</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| Cargo.lock                        | ✔️ | Version bumps, +`scopeguard`,`terminal_size`
| Cargo.toml                        | ✔️ | `insta` bump
| Cargo.toml.orig                   | ✔️ | `insta` bump
| examples\term.rs                  | ✔️ | additional note
| src\term\config.rs                | ✔️ | src indent
| src\term\renderer.rs              | ✔️ | tests, minor tweaking
| src\term\views.rs                 | ✔️ | src indent
| tests\snapshots\\*.snap           | ✔️ | reordering, indent
| tests\term.rs                     | ✔️

<h2 name="0.9.2">0.9.2</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| Cargo.lock                        | ✔️
| examples\readme_preview.rs        | ✔️
| examples\reusable_diagnostic.rs   | ✔️ | New
| src\diagnostic.rs                 | ✔️ | `Copy`
| src\term\config.rs                | ✔️
| src\term\renderer.rs              | ✔️
| src\term\views.rs                 | ✔️
| tests\snapshots\\*.snap           | ✔️
| tests\term.rs                     | ✔️

<h2 name="0.9.1">0.9.1</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️
| Cargo.lock                        | ❔
| src\diagnostic.rs                 | ✔️ | `Debug`
| src\term\views.rs                 | ✔️
| tests\snapshots\\*.snap           | ✔️

<h2 name="0.9.0">0.9.0</h2>

Another major overhaul.  Deleted Readme?!?

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| CHANGELOG.md                      | ✔️ | Huzzah!
| Cargo.lock                        | ❔
| Cargo.toml                        | ✔️ | Drop `codespan`, bump `insta`, add `anyhow`/`peg`/`rustyline` (dev-deps).  Also attempt to to a relative Readme.md.
| Cargo.toml.orig                   | ✔️ | Drop `codespan`, bump `insta`, add `anyhow`/`peg`/`rustyline` (dev-deps).  Also attempt to to a relative Readme.md.
| examples\custom_files.rs          | ✔️ | New
| examples\peg_calculator.rs        | ✔️ | New
| examples\readme_preview.rs        | ✔️ | New
| examples\term.rs                  | ✔️
| src\diagnostic.rs                 | ✔️
| src\files.rs                      | ✔️ | New
| src\lib.rs                        | ✔️
| src\term\config.rs                | ✔️
| src\term\renderer.rs              | ✔️ | New - and holy heck those doc comments!
| src\term\views.rs                 | ✔️ | Merged views\\*.rs
| src\term.rs                       | ✔️
| tests\snapshots\\*.snap           | ✔️
| tests\support\mod.rs              | ✔️
| tests\term.rs                     | ✔️

<h2 name="0.8.0">0.8.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.lock                        | ❔
| Cargo.toml                        | ✔️ | `insta` bump
| Cargo.toml.orig                   | ✔️ | `insta` bump
| src\term\views\diagnostic.rs      | ✔️
| src\term\views\locus.rs           | ✔️
| src\term\views\source_snippet.rs  | ✔️
| src\term\views.rs                 | ✔️
| src\term.rs                       | ✔️
| tests\snapshots\\*.snap           | ✔️

<h2 name="0.7.0">0.7.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.lock                        | ❔
| Cargo.toml                        | ✔️ | -`heapsize`
| Cargo.toml.orig                   | ✔️ | -`heapsize`
| src\diagnostic.rs                 | ✔️
| src\term\mod.rs                   | ✔️
| src\term\views\diagnostic.rs      | ✔️
| src\term\views\mod.rs             | ✔️
| src\term\views\source_snippet\mod.rs | ✔️
| tests\support\mod.rs              | ✔️
| tests\term.rs                     | ✔️

<h2 name="0.6.0">0.6.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.lock                        | ❔ | With the loss of the metadata table this is pretty opaque now
| Cargo.toml                        | ✔️ | Bumped insta, structopt
| Cargo.toml.orig                   | ✔️ | Bumped insta, structopt
| examples\term.rs                  | ✔️
| src\term\config.rs                | ✔️
| src\term\mod.rs                   | ✔️
| src\term\views\diagnostic.rs      | ✔️
| src\term\views\header.rs          | ✔️
| src\term\views\locus.rs           | ✔️
| src\term\views\mod.rs             | ✔️
| src\term\views\new_line.rs        | ✔️
| src\term\views\source_snippet\border.rs | ✔️
| src\term\views\source_snippet\gutter.rs | ✔️
| src\term\views\source_snippet\mod.rs | ✔️
| src\term\views\source_snippet\note.rs | ✔️
| src\term\views\source_snippet\underline.rs | ✔️
| tests\snapshots\\*.snap           | ✔️
| tests\term.rs                     | ✔️

<h2 name="0.5.0">0.5.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.lock                        | ✔️ | Various version bumps, version churn, dropped (extra?) dependencies
| Cargo.toml                        | ✔️ | Bumped codespan, insta, lazy_static deps
| Cargo.toml.orig                   | ✔️ | Bumped codespan, insta, lazy_static deps
| src\term\config.rs                | ✔️
| src\term\views\diagnostic.rs      | ✔️
| src\term\views\source_snippet\border.rs | ✔️
| src\term\views\source_snippet\mod.rs | ✔️ | Rewritten a good bit
| tests\snapshots\\*.snap           | ✔️
| tests\term.rs                     | ✔️ | `insta` is pretty neat

<h2 name="0.4.1">0.4.1</h2>

Trivial codespan bump

<h2 name="0.4.0">0.4.0</h2>

Massive overhaul.  Lots of great comments, organization, testing, etc.

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.lock                        | ✔️ | Added, tons of deps, dear god that's a lot of deps
| Cargo.toml                        | ✔️ | dependency / features tweaks
| Cargo.toml.orig                   | ✔️ | dependency / features tweaks
| README.md                         | ✔️ | Whitespace
| examples\term.rs                  | ✔️
| src\diagnostic.rs                 | ✔️ | Significant overhaul
| src\lib.rs                        | ✔️ | Gutted
| src\term\config.rs                | ✔️ | Tons of new styling stuff
| src\term\mod.rs                   | ✔️ | Color choices
| src\term\views\diagnostic.rs      | ✔️ | Diagnostic APIs
| src\term\views\header.rs          | ✔️ | Diagnostic header APIs
| src\term\views\locus.rs           | ✔️ | File/line/column info
| src\term\views\mod.rs             | ✔️
| src\term\views\new_line.rs        | ✔️ | `\n`?!?
| src\term\views\source_snippet\border.rs       | ✔️ | Stub structs for display
| src\term\views\source_snippet\gutter.rs       | ✔️ | Stub structs for display
| src\term\views\source_snippet\mod.rs          | ✔️ | Source snippet core logic - tons of excellent comments
| src\term\views\source_snippet\note.rs         | ✔️ | Stub structs for display
| src\term\views\source_snippet\underline.rs    | ✔️ | Stub structs for display
| tests\snapshots\\*.snap           | ✔️
| tests\support\color_buffer.rs     | ✔️
| tests\support\mod.rs              | ✔️
| tests\term.rs                     | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | None
| io        | ✔️ | tons via termcolor
| docs      | ✔️ | lots
| tests     | ✔️ | pretty good

<h2 name="0.3.0">0.3.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | edition 2018
| Cargo.toml.orig                   | ✔️ | edition 2018
| examples\emit.rs                  | ✔️ | cleanup
| src\diagnostic.rs                 | ✔️ | cleanup
| src\emitter.rs                    | ✔️ | cleanup, tweaks

<h2 name="0.2.1">0.2.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| examples\emit.rs                  | ✔️
| src\emitter.rs                    | ✔️

<h2 name="0.2.0">0.2.0</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | `serde`
| Cargo.toml.orig                   | ✔️ | `serde`
| src\diagnostic.rs                 | ✔️ | `Deserialize`
| src\lib.rs                        | ✔️ | `Deserialize`

<h2 name="0.1.4">0.1.4</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | `heapsize`
| Cargo.toml.orig                   | ✔️ | `heapsize`
| examples\emit.rs                  | ✔️ | `HeapSizeOf`
| src\diagnostic.rs                 | ✔️ | `HeapSizeOf`
| src\emitter.rs                    | ✔️ | `HeapSizeOf`
| src\lib.rs                        | ✔️ | `HeapSizeOf`

<h2 name="0.1.3">0.1.3</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | `structopt` dev-dep, exclude
| Cargo.toml.orig                   | ✔️ | `structopt` dev-dep, exclude
| README.md                         | ✔️ | Fleshed out
| examples\emit.rs                  | ✔️
| src\diagnostic.rs                 | ✔️
| src\emitter.rs                    | ✔️
| src\lib.rs                        | ✔️ | +much docs

<h2 name="0.1.2">0.1.2</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| examples\emit.rs                  | ✔️
| src\emitter.rs                    | ✔️

<h2 name="0.1.1">0.1.1</h2>

| Diff                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| Cargo.toml                        | ✔️ | Metadata
| Cargo.toml.orig                   | ✔️ | Metadata

<h2 name="0.1.0">0.1.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| examples\emit.rs                  | ✔️
| src\diagnostic.rs                 | ✔️
| src\emitter.rs                    | ✔️
| src\lib.rs                        | ✔️
| .cargo-ok                         | ✔️
| Cargo.toml                        | ✔️ | `Apache-2.0`, `termcolor`
| Cargo.toml.orig                   | ✔️ | `Apache-2.0`, `termcolor`
| README.md                         | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | None
| io        | ✔️ | via termcolor
| docs      | ⚠️
| tests     | ⚠️

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
