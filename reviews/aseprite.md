---
category:       General Utility
description:    Parse JSON aseprite exports
---

# aseprite

Parses JSON [aseprite](https://www.aseprite.org/) exports

[ [github](https://github.com/ggez/aseprite)
• [docs.rs](https://docs.rs/aseprite/)
• [lib.rs](https://lib.rs/crates/aseprite)
]

Pros:
* Looks fairly exhaustive, self contained
* No direct disk I/O

Cons:
* Exahustively matchable structs make most version bumps technically breaking changes
* Requires non-default export options (fixed in an unmerged PR)
* Enum deserialization is brittle when it comes to new options
* Limited maintainence

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.1.3] | medium | high | ✔️ positive | ⚠️ Technically breaking change
| [0.1.2] | medium | high | ✔️ positive | Made fields public
| [0.1.1] | medium | high | ❗️ negative | All fields private, making it unusuable.
| [0.1.0] | medium | high | ❗️ negative | All fields private, making it unusuable.

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         ❌ dangerous ⚠️❗️ negative ❔ neutral ✔️ positive ✔️ strong
-->

[0.1.3]: #0.1.3
[0.1.2]: #0.1.2
[0.1.1]: #0.1.1
[0.1.0]: #0.1.0

<h2 name="0.1.3">0.1.3</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| README<span>.</span>md                                    | ✔️ | Link old docs, tweak
| examples\boonga<span>.</span>json                         | ✔️ | Re-export?
| src\lib<span>.</span>rs                                   | ⚠️ | Added "image" to metadata - technically a breaking change

<h2 name="0.1.2">0.1.2</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | Made fields public, expanded enums, documented necessity of `--format json-array`

<h2 name="0.1.1">0.1.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| README<span>.</span>md                                    | ✔️ | Documented export CLI
| examples\boonga<span>.</span>ase                          | ✔️
| examples\boonga<span>.</span>json                         | ✔️
| examples\boonga<span>.</span>png                          | ✔️
| examples\loading<span>.</span>rs                          | ✔️
| src\lib<span>.</span>rs                                   | ❌ | All fields still private, replaced some strings with enums, more tests

<h2 name="0.1.0">0.1.0</h2>

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>gitignore                                   | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | MIT
| LICENSE                                                   | ✔️ | MIT
| README<span>.</span>md                                    | ✔️
| src\lib<span>.</span>rs                                   | ❌ | All fields private, unusable - seems OK otherwise

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ✔️ | None
| fs        | ✔️ | Examples only
| io        | ✔️ | Examples/serde only
| docs      | ⚠️ | Basically none, but relatively self-describing
| tests     | ✔️ | Basic



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
