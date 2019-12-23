---
category:       Unsound
crev:           negative
description:    Cross-platform console stuff.  No web support, soundness issues.
---

# Crossterm

Pros:
* Cross platform

Cons:
* Oddly reliant on struct instances / OO style
* No browser support
* Implementation crates have had soundness issues

0.11.1
======
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | positive

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| .github/workflows/crossterm_test.yml          | +1 | hecrj/setup-rust-action@master |
| .github/CODEOWNERS                            | +1 | |
| docs/mdbook/src/command.md                    | +1 | |
| docs/mdbook/src/feature_flags.md              | +1 | |
| docs/mdbook/src/input.md                      | +1 | |
| docs/mdbook/src/screen.md                     | +1 | |
| docs/mdbook/src/styling.md                    | +1 | |
| docs/mdbook/src/SUMMARY.md                    | +1 | |
| docs/mdbook/book.toml                         | +1 | |
| docs/.gitignore                               | +1 | |
| docs/CONTRIBUTING.md                          | +1 | |
| docs/crossterm_c.png                          | +1 | |
| docs/crossterm_full.png                       | +1 | |
| docs/crossterm_full.svg                       | +1 | |
| docs/know-problems.md                         | +1 | |
| docs/UPGRADE.md                               | +1 | |
| src/crossterm.rs                              | +1 | |
| src/lib.rs                                    | +1 | |
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | No MSRV
| Cargo.toml                                    | +1 | MIT
| Cargo.toml.orig                               | +1 | MIT
| CHANGELOG.md                                  | +1 | |
| LICENSE                                       | +1 | MIT
| README.md                                     |  0 | No test coverage of examples

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    |
| miri      |
| fs        |
| io        |
| docs      |
| tests     |
