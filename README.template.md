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

| Legend | Description |
| ------ | ----------- |
| ![crev-author]    &nbsp; Author      | I wrote this!
| ![crev-positive]  &nbsp; Positive    | Seems safe/sound/possibly useful
| ![crev-neutral]   &nbsp; Neutral     | This crate is OK, but might have better alternatives
| ![crev-negative]  &nbsp; Negative    | I have serious concerns, possibly including: too much `unsafe`, `panic!`-prone, history of soundness bugs, general brittleness, or lacking critical functionality.  Might still be a good basis for cleanup / forking.
| ![crev-dangerous] &nbsp; Dangerous   | Unsound, vulnerable, or likely to be (now or in the future based on poor history)
| ![crev-none]      &nbsp; N/A         | Haven't properly reviewed the code yet

# Categories

{{#categories}}
* [{{category}}](#{{anchor}})
{{/categories}}

{{#categories}}
<h2 id="{{anchor}}">{{{category}}}</h2>

| Review | Links | Description |
| ------ | ----- | ----------- |
{{#crates}}
| ![crev-{{crev}}] &nbsp; [{{crate}}](reviews/{{crate}}.md) | [docs.rs](https://docs.rs/{{crate}}) <!-- [lib.rs](https://lib.rs/crates/{{crate}}) --> | {{description}}
{{/crates}}

{{/categories}}


# Procedures

## Adding a new review

* Add `cratename = "=0.0.1" to Cargo.toml
* Review via crev
    ```cmd
    cargo update
    cargo crev query review  cratename
    cargo crev open          cratename
    cargo crev review        cratename
    cargo crev push
    ```
* Create equivalent notes in new reviews/cratename.md
* Link in README.md
* git commit
    ```
    Add cratename review for v0.0.1
    ```
* git push

## Updating a review

* Bump version once in Cargo.toml
* Review via crev
    ```cmd
    cargo update
    cargo crev query review  cratename
    cargo crev open          cratename
    cargo crev diff          cratename --src=0.0.1 --dst=0.0.2  -u --color
    cargo crev review        cratename --diff
    cargo crev push
    ```
* Append equivalent notes to reviews/cratename.md
* Bump version in README.md
* git commit
    ```
    Update cratename review to v0.0.2

    Any extra notes

    Closes MaulingMonkey/rust-reviews#000
    ```
* git push

## Initial Setup

* Install and configure cargo crev
    ```cmd
    cargo install cargo-crev
    :: ...?
    ```
* Add the following to your `cargo crev edit config`:
    ```yml
    ---
    version: -1
    ...
    open-cmd: "start \"\" \"C:\\Users\\UserName\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe\" -n --disable-extension rust-lang.rust --disable-extension matklad.ra-lsp --disable-extension kalitaalexey.vscode-rust"
     
    ```
    This disables these extensions, which all provide Rust intellisense, which has a tendency to pollute `%USERPROFILE%\.cargo\registry\src` with `Cargo.lock` files, `target` directories, or worse, which makes cargo crev angry:
    * rust-lang.rust
    * matklad.ra-lsp
    * kalitaalexey.vscode-rust
