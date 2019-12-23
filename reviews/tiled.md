---
category:       Gamedev
description:    [Tiled](https://www.mapeditor.org/) `.tmx` file parser. Decent bones, but I'm concerned about path traversal attacks.
---

# tiled

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.8.0](#0.8.0) | medium | medium | negative

Do not use on User Generated Content!
    * Vulnerable to path traversal attacks if fed bogus .tmx files (see Tileset::new_reference)
    * No obvious protection against zipbombs
    * A couple cases where bad input will panic, a potential DoS vector.

For game engines, there's also no great way to inject your own virtual filesystem callbacks (again see Tileset::new_reference)

There's also a few missing features:
    * Wang Tiles
    * Terrains
    * "file" Custom Properties

# 0.8.0

Detail
------

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| assets/tiled_base64_external.tmx              | +1 | |
| assets/tiled_base64_gzip.tmx                  | +1 | |
| assets/tiled_base64_zlib.tmx                  | +1 | |
| assets/tiled_base64.tmx                       | +1 | |
| assets/tiled_csv.tmx                          | +1 | |
| assets/tiled_image_layers.tmx                 | +1 | |
| assets/tiled_xml.tmx                          | +1 | |
| tilesheet.png                                 | +1 | Neat looking modern tileset... wonder what the source is!
| tilesheet.tsx                                 | +1 | |
| examples/main.rs                              | +1 | |
| [src/lib.rs](src/lib.rs)                      | +1 | |
| tests/lib.rs                                  | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | Sparse... no MSRV, beta/nightly, etc. |
| Cargo.toml                                    | +1 | |
| Cargo.toml.orig                               | +1 | |
| CONTRIBUTORS.md                               | +1 | |
| README.md                                     | +1 | Dead link to tileset source

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1 | No unsafe
| fs        | -1 | See Tileset::new_reference notes
| io        | 0  | Brittle XML parsing, but OK for limited inputs.
| docs      | -1 | Barely any.
| tests     | +1 | |

src/lib.rs
----------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  28 | get_attrs!              | Eep
|  53 | parse_tag!              | Mishandles nested tags... fortunately that's probably not necessary.
|  97 | Colour::from_str        | British... and a possible source of panics.
| 161 | PropertyValue::new      | No "file" support (see https://doc.mapeditor.org/en/stable/manual/custom-properties/#adding-properties)
| 238 | Map::new                | My kingdom for some variable names!
| 256 | Map::new                | Still using try!
| 385 | Tileset::new_reference  | Possible path traversal attacks, limits ability to inject your own virtual filesystem.
| 860 | decode_csv              | I've heard decoding arbitrary CSV is way more complicated than this... but this probably works for tile data as used in tmx files.
| 883 | convert_to_u32          | Kinda want this to be based on iterators to avoid an extra alloc...
