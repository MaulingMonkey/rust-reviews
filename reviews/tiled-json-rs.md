---
category:       Gamedev
description:    [Tiled](https://www.mapeditor.org/) `.json` export file parser. Decent bones, but I'm concerned about path traversal attacks.
---

# tiled

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.2.6](#0.2.6) | medium | medium | negative

# 0.2.6

Do not use on User Generated Content!
* Vulnerable to path traversal attacks if fed bogus .tmx files (see Tileset::new_reference)
* A couple cases where bad input will panic, a potential DoS vector.

For game engines, there's also no great way to inject your own virtual filesystem callbacks (again see Tileset::new_reference)

Pros:
* JSON is lighter weight than XML
* Fuller format support vs tiled

Cons:
* No compression
* API is just as raw in many ways
* Requires exporting.
* Slightly unusual license for rust projects (MPL, instead of MIT/Apache 2)

Detail
======

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| [src/layer.rs](src/layer.rs)                  | 0 | Raw structures
| src/lib.rs                                    | +1 | |
| src/map.rs                                    | +1 | |
| src/object.rs                                 | +1 | |
| [src/parsers.rs](src/parsers.rs)              | 0 | No decompression support, can panic (not suitable for user generated content)
| [src/tile_set.rs](src/tile_set.rs)            | -1 | Not suitable for user generated content!
| src/utils.rs                                  | +1 | |
| src/wangs.rs                                  | +1 | |
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| Cargo.toml                                    | +1 | |
| Cargo.toml.orig                               | +1 | |
| LICENSE                                       | +1 | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1 | None
| fs        | -1 | Path traversal
| io        | +1 | serde
| docs      | +1 | |
| tests     | -1 | Not in crate, maybe in repository

src/layer.rs
------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  17   | TileLayer::chunks | Option seems kinda pointless, also this API is meh
| 129   | DrawOrder | There are at least 4 draw modes now for layers - although there's also Map / RenderOrder.... blehrg (top->down left->right, top->down right->left, ...)

src/parsers.rs
--------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 129   | parse_color blue | Despite earlier padding, no guarantee this is valid / may panic (both for overflowing and for not being a unicode boundary.)

src/tile_set.rs
---------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 121   | Deserialize for TileSet | File::open - path traversal attacks, lack of virtual filesystem support, etc.
