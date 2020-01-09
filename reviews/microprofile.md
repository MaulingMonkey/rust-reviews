---
category:       Unsound
crev:           dangerous
description:    Bindings for a C++ flamegraph profiler
---

# microprofile

Pros:
* Probably works?
* XB1 and PS4 port allusions

Cons:
* **98% networked exposed C++**
    * **that also string parses**
    * **and disassembles**
    * **and rewrites x86 code on the fly**
    * **I don't see fuzz tests upstream either**
* If this isn't unsound somewhere, I'll eat my hat.
* Don't use in production, only development, IMO

# Audit

| version | thoroughness | understanding | rating | Notes |
| ------- | ------------ | ------------- | ------ | ----- |
| [0.2.1]   | low | low | negative | Much skimming

[0.2.1]:    #021

# 0.2.1

| file                                          | rating | notes |
| --------------------------------------------- | ------ | ----- |
| src/microprofile/distorm/include/distorm.h    | :grey_question:       | Skimmed, BSD
| src/microprofile/distorm/include/mnemonics.h  | :grey_question:       | Skimmed
| src/microprofile/distorm/src/config.h         | :grey_question:       | Skimmed
| src/microprofile/distorm/src/decoder.c        | :grey_question:       | Skimmed
| src/microprofile/distorm/src/decoder.h        | :grey_question:       | Skimmed
| src/microprofile/distorm/src/distorm.c        | :grey_question:       | Skimmed
| src/microprofile/distorm/src/instructions.c   | :grey_question:       | Skimmed
| src/microprofile/distorm/src/instructions.h   | :grey_question:       | Skimmed
| src/microprofile/distorm/src/insts.c          | :grey_question:       | Skimmed, magic constants table
| src/microprofile/distorm/src/insts.h          | :grey_question:       | Skimmed
| src/microprofile/distorm/src/mnemonics.c      | :grey_question:       | Skimmed, magic constants table
| src/microprofile/distorm/src/operands.c       | :grey_question:       | Skimmed
| src/microprofile/distorm/src/operands.h       | :grey_question:       | Skimmed
| src/microprofile/distorm/src/prefix.c         | :grey_question:       | Skimmed
| src/microprofile/distorm/src/prefix.h         | :grey_question:       | Skimmed
| src/microprofile/distorm/src/textdefs.c       | :grey_question:       | Skimmed
| src/microprofile/distorm/src/textdefs.h       | :grey_question:       | Skimmed
| src/microprofile/distorm/src/wstring.c        | :grey_question:       | Skimmed
| src/microprofile/distorm/src/wstring.h        | :grey_question:       | Skimmed
| src/microprofile/distorm/src/x86defs.h        | :grey_question:       | Skimmed
| src/microprofile/distorm/COPYING              | :x:                   | Upstream file missing, legally required for redistribution
| src/microprofile/microprofile_html.h          | :grey_question:       | Skimmed... 15 KLOC of C++ HTML/JS strings, oof
| src/microprofile/microprofile.cpp             | :x:                   | Skimmed... 13 KLOC of network exposed, string parsing, instruction rewriting C++11.
| src/microprofile/microprofile.h               | :grey_question:       | Skimmed
| src/microprofile/patch_osx.s                  | :grey_question:       | Skimmed
| src/microprofile/patch_win32.asm              | :grey_question:       | Skimmed
| src/build.rs                                  | :heavy_check_mark:    | cc
| src/lib.rs                                    | :grey_question:       | No docs, haven't verified C ABI, uses static mut, but not obviously unsound.
| .cargo_fcs_info.json                          | :heavy_check_mark:    | |
| .cargo-ok                                     | :heavy_check_mark:    | |
| Cargo.toml                                    | :x:                   | Says MIT, includes BSD code.
| Cargo.toml.orig                               | :x:                   | Says MIT, includes BSD code.

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | :x:                   | 98% C/C++ code - including string parsing, network stuff
| fs        | :grey_question:       | Yes
| io        | :x:                   | C/C++ networking and string parsing code
| docs      | :x:                   | distorm is well documented, microprofile (both the C++ and Rust) aren't so much
| tests     | :x:                   | Not a fuzz test in sight
