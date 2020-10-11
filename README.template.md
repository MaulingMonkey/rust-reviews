# MaulingMonkey's Rust Reviews

This repository serves a few purpouses:
* To provide a quick overview and human readable versions of all [my cargo crev reviews](https://github.com/MaulingMonkey/crev-proofs).
* To provide a repository that [Dependabot](https://dependabot.com) can create issues against, to remind me to update my crev proofs.
* To provide a visual fallback via deps.rs:  [![deps.rs](https://deps.rs/repo/github/MaulingMonkey/rust-reviews/status.svg)](https://deps.rs/repo/github/MaulingMonkey/rust-reviews)

[crev-author]:      https://img.shields.io/badge/-🐵-green
[crev-none]:        https://img.shields.io/badge/-%3F-lightblue

[audio-rodio]:      https://img.shields.io/badge/🔊-rodio-green

[crev-positive]:    https://img.shields.io/badge/-✓-green
[crev-neutral]:     https://img.shields.io/badge/-%3D-lightgrey
[crev-negative]:    https://img.shields.io/badge/-✗-yellow
[crev-dangerous]:   https://img.shields.io/badge/-✗-red

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Legend&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Description |
| ----------------------------- | ----------- |
| ![crev-author]    Author      | I wrote this!
| ![crev-positive]  Positive    | Seems safe/sound/possibly useful
| ![crev-neutral]   Neutral     | This crate is OK, but might have better alternatives
| ![crev-negative]  Negative    | I have serious concerns, possibly including: too much `unsafe`, `panic!`-prone, history of soundness bugs, general brittleness, or lacking critical functionality.  Might still be a good basis for cleanup / forking.
| ![crev-dangerous] Dangerous   | Unsound, vulnerable, or likely to be (now or in the future based on poor history)
| ![crev-none]      N/A         | Haven't properly reviewed the code yet

# Categories

{{#categories}}
* [{{category}}](#{{anchor}})
{{/categories}}

{{#categories}}
<h2 id="{{anchor}}">{{{category}}}</h2>

| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Review&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Links | Description |
| ------ | ----- | ----------- |
{{#crates}}
| ![crev-{{crev}}] &nbsp; [{{crate}}](reviews/{{crate}}.md) | [docs.rs](https://docs.rs/{{crate}}) <!-- [lib.rs](https://lib.rs/crates/{{crate}}) --> | {{description}}
{{/crates}}

{{/categories}}


# Procedures

## Newfangled Reviews

```sh
# Display versions in VS Code
cargo versions byteorder
```

```sh
# Prefer cmd.exe for keepass purpouses

# Generate template and open secondary vscode window with all versions open
cargo review --all byteorder
cargo open byteorder *

# Diff versions
cls && cargo diff byteorder 0.1.1
cls && cargo diff byteorder 0.2.0
...

# Publish review to github:
# Finish authoring [cratename].md
# Add/commit [cratename].md
git push github master

# Crosspost to crev, linking https://github.com/MaulingMonkey/rust-reviews/blob/master/reviews/ [cratename].md
cargo install cargo-crev
cargo crev crate review -u --advisory            byteorder --vers 0.2.11
cargo crev crate review -u --advisory            byteorder --vers 0.3.8
cargo crev crate review -u --skip-activity-check byteorder --vers 1.3.4
cargo crev repo git diff HEAD~1
cargo crev repo publish
```

```yml
# Combined advisory/review/flags/alternatives template
advisories:
  - ids: []
    severity: medium
    range: major
    comment: ""
review:
  thoroughness: low
  understanding: medium
  rating: positive
flags:
  unmaintained: false
alternatives:
  - source: "https://crates.io"
    name: ""
comment: |-
```
