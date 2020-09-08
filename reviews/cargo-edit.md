---
category:       Command Line Tools
description:    Add/remove/update Cargo.toml dependencies from the CLI.
---

# cargo-edit

Add/remove/update Cargo.toml dependencies from the command line.

Pros:
* Safe, probably works

Cons:
* Lots of code
* Lots of dependencies, some of which I haven't fully audited.
* License ambiguity (is this MIT or Apache-2.0/MIT licensed?)

0.4.0
=====
| crev          |   |
| ------------- |---|
| thoroughness  | low
| understanding | medium
| rating        | neutral

| Diff                                                  | Rating | Notes |
| ----------------------------------------------------- | ------ | ----- |
| Cargo.lock                                            | +1 | Added since 0.3.3, enabling frozen installs.  Approx [200 indirect deps](https://github.com/killercup/cargo-edit/blob/e131c92b8e1d5617fb9dca2e095ed7d7c91776ae/Cargo.lock#L1819).
| Cargo.toml                                            | +1 | Version bumps
| Cargo.toml.orig                                       | +1 | Version bumps
| README.md                                             | +1 | Mentions new `--sort` option
| appveyor.yml                                          | +1 | Disables gnu targets
| src/bin/add/args.rs                                   | +1 | New `--sort` and `--offline` options
| src/bin/add/main.rs                                   | +1 | |
| src/bin/upgrade/main.rs                               | +1 | |
| src/errors.rs                                         | +1 | |
| src/fetch.rs                                          | +1 | |
| src/lib.rs                                            | +1 | |
| src/manifest.rs                                       | +1 | |
| src/registry.rs                                       | +1 | |
| tests/cargo-add.rs                                    | +1 | |
| tests/fixtures/add/Cargo.toml.unsorted                | +1 | |

0.3.3
=====
| crev          |   |
| ------------- |---|
| thoroughness  | low
| understanding | medium
| rating        | neutral

| File                                                  | Rating | Notes |
| ----------------------------------------------------- | ------ | ----- |
| src/bin/add/args.rs                                   | +1 | |
| src/bin/add/main.rs                                   | +1 | |
| src/bin/add/manifest_test.rs                          | +1 | |
| src/bin/rm/main.rs                                    | +1 | |
| src/bin/upgrade/main.rs                               | +1 | |
| src/crate_name.rs                                     | -1 | .contains(url) seems wrong
| src/dependency.rs                                     | -1 | No branch support for dependencies?
| src/errors.rs                                         | +1 | |
| src/fetch.rs                                          | +1 | |
| src/lib.rs                                            | +1 | |
| src/manifest.rs                                       | 0 | find/search duplicate some of `cargo metadata`'s effort I believe
| tests/fixtures/add/local/Cargo.toml.sample            | +1 | |
| tests/fixtures/add/Cargo.toml.sample                  | +1 | |
| tests/fixtures/manifest-invalid/Cargo.toml.sample     | +1 | |
| tests/fixtures/rm/Cargo.toml.sample                   | +1 | |
| tests/fixtures/upgrade/Cargo.toml.invalid             | +1 | |
| tests/fixtures/upgrade/Cargo.toml.source              | +1 | |
| tests/fixtures/upgrade/Cargo.toml.target              | +1 | |
| tests/cargo-add.rs                                    | 0 | 191: Duplicate assert!s for no reason?
| tests/cargo-rm.rs                                     | +1 | Tests are admittedly a bit brittle
| tests/cargo-upgrade.rs                                | +1 | |
| tests/test_manifest.rs                                | +1 | |
| tests/utils.rs                                        | 0 | 17: Pointless clone, beware execute_* passes to exec.
| .cargo_vcs_info.json                                  | +1 | |
| .cargo-ok                                             | +1 | |
| .editorconfig                                         | +1 | |
| .gitignore                                            | +1 | |
| .travis.yml                                           | 0 | rustfmt, clippy, travis-cargo, libcurl4-openssl-dev, libelf-dev, libdw-dev, coveralls
| appveyor.yml                                          | +1 | |
| bors.toml                                             | +1 | |
| Cargo.toml                                            | 0 | Apache-2.0/MIT.  That's a lot of deps.
| Cargo.toml.orig                                       | 0 | ^
| Cargo.lock                                            | -1 | N/A, would nice to be able to --frozen(?) to install fully audited bins
| CONTRIBUTING.md                                       | +1 | |
| LICENSE                                               | 0 | Just MIT listed here, Cargo.toml references Apache-2.0/MIT.
| README.md                                             | 0 | "Apache-2.0/MIT" could be clearer in a Readme.
| rustfmt.toml                                          | 0 | Empty file

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1 | None, warn if introduced
| miri      | N/A | Not bothering with
| fs        | 0 | Manifest related, looks safe?
| io        | 0 | Manifest related
| docs      | +1 | |
| tests     | +1 | |

TIL
---
* crates.io API string format
* Refresher on format! placeholders
```rust
format!(
    "{host}/api/v1/crates/{crate_name}",
    host = REGISTRY_HOST,
    crate_name = crate_name
);
```
