---
category:       I/O
description:    Terminal I/O Settings
msrv:           1.13
---

# termios

Terminal I/O Settings

Pros:
* Portable as heck
* Probably sound

Cons:
* Limited tests
* No verification against native C headers

## Audit

⚠️ Didn't verify many OS-specific structs/constants, and termios structure might not guarantee ABI stability?

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.3.3] | low | high | ✔️ positive | + Illumos
| [0.3.2] | low | high | ✔️ positive | + NetBSD / Solaris
| [0.3.1] | low | high | ✔️ positive | + Android
| [0.3.0] | low | high | ✔️ positive | + DragonFly BSD, MSRV 1.13
| [0.2.2] | low | high | ✔️ positive | + OpenBSD
| [0.2.1] | low | high | ✔️ positive |
| [0.2.0] | low | high | ✔️ positive | + FreeBSD
| [0.1.0] | low | high | ✔️ positive | Mass file org refactor / documentation pass.
| [0.0.5] | low | high | ✔️ positive |
| [0.0.4] | low | high | ✔️ positive |
| [0.0.3] | low | high | ✔️ positive |
| [0.0.2] | low | high | ✔️ positive |
| [0.0.1] | low | high | ✔️ positive |

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         ❌ dangerous ⚠️❗️ negative ❔ neutral ✔️ positive ✔️ strong
-->

[0.3.3]: #0.3.3
[0.3.2]: #0.3.2
[0.3.1]: #0.3.1
[0.3.0]: #0.3.0
[0.2.2]: #0.2.2
[0.2.1]: #0.2.1
[0.2.0]: #0.2.0
[0.1.0]: #0.1.0
[0.0.5]: #0.0.5
[0.0.4]: #0.0.4
[0.0.3]: #0.0.3
[0.0.2]: #0.0.2
[0.0.1]: #0.0.1

<h2 name="0.3.3">0.3.3</h2>

+ illumos

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| Cargo<span>.</span>toml                                   | ✔️
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️
| src\ffi<span>.</span>rs                                   | ✔️
| src\lib<span>.</span>rs                                   | ✔️
| src\os\illumos<span>.</span>rs                            | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?
| src\os\mod<span>.</span>rs                                | ✔️

<h2 name="0.3.2">0.3.2</h2>

+ NetBSD, Solaris/illumos

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| CHANGELOG<span>.</span>md                                 | ✔️
| CONTRIBUTING<span>.</span>md                              | ✔️
| CONTRIBUTORS                                              | ✔️
| Cargo<span>.</span>toml                                   | ✔️
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️
| README<span>.</span>md                                    | ✔️
| src\ffi<span>.</span>rs                                   | ✔️
| src\lib<span>.</span>rs                                   | ✔️
| src\os\mod<span>.</span>rs                                | ✔️
| src\os\netbsd<span>.</span>rs                             | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?
| src\os\solaris<span>.</span>rs                            | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?

<h2 name="0.3.1">0.3.1</h2>

+ Android

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | Bump rustc 1.0.0 -> 1.13.0
| Cargo<span>.</span>toml                                   | ✔️
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️
| README<span>.</span>md                                    | ✔️
| src\os\android<span>.</span>rs                            | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?
| src\os\mod<span>.</span>rs                                | ✔️

<h2 name="0.3.0">0.3.0</h2>

+ DragonFly BSD

Removed `extern crate libc;`, MSRV 1.13.0

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️
| README<span>.</span>md                                    | ✔️
| src\ffi<span>.</span>rs                                   | ✔️
| src\lib<span>.</span>rs                                   | ✔️
| src\os\dragonfly<span>.</span>rs                          | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?
| src\os\freebsd<span>.</span>rs                            | ✔️⚠️
| src\os\linux<span>.</span>rs                              | ✔️⚠️
| src\os\macos<span>.</span>rs                              | ✔️⚠️
| src\os\mod<span>.</span>rs                                | ✔️
| src\os\openbsd<span>.</span>rs                            | ✔️⚠️

<h2 name="0.2.2">0.2.2</h2>

+ OpenBSD

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️
| README<span>.</span>md                                    | ✔️
| src\lib<span>.</span>rs                                   | ✔️
| src\os\mod<span>.</span>rs                                | ✔️
| src\os\openbsd<span>.</span>rs                            | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?

<h2 name="0.2.1">0.2.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | Fixed repository link
| README<span>.</span>md                                    | ✔️

<h2 name="0.2.0">0.2.0</h2>

+ FreeBSD

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | Rust 1.0.0
| Cargo<span>.</span>toml                                   | ✔️
| README<span>.</span>md                                    | ✔️
| src\lib<span>.</span>rs                                   | ✔️
| src\os\freebsd<span>.</span>rs                            | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?
| src\os\mod<span>.</span>rs                                | ✔️

<h2 name="0.1.0">0.1.0</h2>

Mass file org refactor / documentation pass.

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️
| README<span>.</span>md                                    | ✔️
| src\ffi<span>.</span>rs                                   | ✔️
| src\lib<span>.</span>rs                                   | ✔️
| src\os\linux<span>.</span>rs                              | ⚠️ | Didn't verify constants, and termios structure might not guarantee ABI stability?
| src\os\macos<span>.</span>rs                              | ⚠️ | Didn't verify structs/constants, and termios structure might not guarantee ABI stability?
| src\os\mod<span>.</span>rs                                | ✔️

<h2 name="0.0.5">0.0.5</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️
| Cargo<span>.</span>toml                                   | ✔️
| README<span>.</span>md                                    | ✔️
| src\ffi<span>.</span>rs                                   | ✔️⚠️

<h2 name="0.0.4">0.0.4</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️
| README<span>.</span>md                                    | ✔️
| src\ffi<span>.</span>rs                                   | ✔️⚠️

<h2 name="0.0.3">0.0.3</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️
| Cargo<span>.</span>toml                                   | ✔️
| README<span>.</span>md                                    | ✔️
| src\ffi<span>.</span>rs                                   | ✔️⚠️
| src\lib<span>.</span>rs                                   | ✔️

<h2 name="0.0.2">0.0.2</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️
| README<span>.</span>md                                    | ✔️
| src\ffi<span>.</span>rs                                   | ✔️⚠️
| src\lib<span>.</span>rs                                   | ✔️

<h2 name="0.0.1">0.0.1</h2>

Initial version

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>gitignore                                   | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | MIT
| LICENSE                                                   | ✔️ | MIT
| src\ffi<span>.</span>rs                                   | ⚠️ | Didn't verify many OS-specific structs/constants, and termios structure might not guarantee ABI stability?
| src\lib<span>.</span>rs                                   | ✔️

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ⚠️ | As necessary.  Biggest concern is termios struct ABI, which might not be stable on some OSes.
| fs        | ✔️
| io        | ✔️
| docs      | ✔️
| tests     | ⚠️ | Limited tests.
