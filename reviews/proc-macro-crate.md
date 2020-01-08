---
category:       Macros
description:    $crate for proc macros
---

# proc-macro-crate

| version | thoroughness | understanding | rating | Notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.1.0] | medium | medium | positive | Full review

[0.1.0]:    #100

# 0.1.0

| file                  | rating | notes |
| --------------------- | ------ | ----- |
| src/lib.rs            | :heavy_check_mark: | |
| .cargo-ok             | :heavy_check_mark: | |
| .gitignore            | :heavy_check_mark: | |
| .travis.yml           | :heavy_check_mark: | MSRV: stable
| Cargo.toml            | :heavy_check_mark: | Apache-2.0/MIT |
| Cargo.toml.orig       | :heavy_check_mark: | Apache-2.0/MIT |
| README.md             | :heavy_check_mark: | Apache-2.0 OR MIT


| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | :heavy_check_mark: | None
| fs        | :heavy_check_mark: | Opens `%CARGO_MANIFEST_DIR%\Cargo.toml` for read only
| io        | :heavy_check_mark: | Bulk parsing outsourced to toml crate
| docs      | :heavy_check_mark: | Straightforward
| tests     | :heavy_check_mark: | |
