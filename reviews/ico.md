---
category:       Serialization
description:    Encoders/decoders for .ico and .cur file formats
---

# ico

Encoders/decoders for .ico and .cur file formats.

Pros:
* Well documented and ~complete despite 0.1.0 version
* Pure Rust

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.0]   | medium | high | positive | Full Review

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[0.1.0]: #0.1.0

<h2 name="0.1.0">0.1.0</h2>

| File                              | Rating | Notes |
| --------------------------------- | ------ | ----- |
| .cargo-ok                         | ✔️
| .gitignore                        | ✔️
| Cargo.toml                        | ✔️ | `MIT`
| Cargo.toml.orig                   | ✔️ | `MIT`
| LICENSE                           | ✔️ | `MIT`
| README.md                         | ✔️
| rustfmt.toml                      | ✔️
| examples\icotool.rs               | ✔️
| [src\lib.rs](#0.1.0-src-lib-rs)   | ✔️
| tests\decode.rs                   | ✔️
| tests\encode.rs                   | ✔️
| tests\images\\*                   | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | Examples/tests only, as appropriate
| io        | ✔️ | Safe, reasonable
| docs      | ✔️ | Good
| tests     | ✔️ | Good

<h2 name="0.1.0-src-lib-rs">src\lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
|  432 | `IconImage::from_rgba_data`: would prefer `Result`s instead of `panic`s for overly large images
|  486 | `IconImage::read_png`: TODO: !8bpp
|  529 | `IconImage::read_png`: TODO: Greyscale
|  604 | `IconImage::read_bmp`: Y-flipped BMPs not supported?
