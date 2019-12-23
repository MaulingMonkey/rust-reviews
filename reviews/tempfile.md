---
category:       General Utility
description:    Create/cleanup temporary files and directories.
---

# tempfile

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [3.1.0](#3.1.0) | medium | medium | positive

Solid crate overall.

Concerns:
* Absurdly high default `NUM_RETRIES` means this crate can hang.
* Slightly unnecessarily large unsafe use, used once without clear need.
* TempDir seems like a big footgun.
* Well documented footguns with regards to share tmp dirs on some systems.
* Hazard to reproducable builds thanks to random filename generation.

# 3.1.0

Detail
------

| File                                                  | Rating | Notes |
| ----------------------------------------------------- | ------ | ----- |
| src/file/imp/mod.rs                                   | +1    | |
| src/file/imp/other.rs                                 | +1    | not_supported
| [src/file/imp/unix.rs](#src/file/imp/unix.rs)         | +1    | `unsafe`, but sound.
| [src/file/imp/windows.rs](#src/file/imp/windows.rs)   | +1    | `unsafe`, but sound.
| [src/file/mod.rs](#src/file/mod.rs)                   | +1    | |
| src/dir.rs                                            | +1    | |
| src/error.rs                                          | +1    | |
| [src/lib.rs](#src/lib.rs)                             | +1    | |
| src/spooled.rs                                        | +1    | Could use a `.into_file()`
| [src/util.rs](#src/util.rs)                           | 0     | `unsafe`, but sound.
| tests/namedtempfile.rs                                | +1    | |
| tests/spooled.rs                                      | +1    | |
| tests/tempdir.rs                                      | +1    | |
| tests/tempfile.rs                                     | +1    | |
| .gitignore                                            | +1    | |
| Cargo.toml                                            | +1    | |
| Cargo.toml.orig                                       | +1    | |
| LICENSE-APACHE                                        | +1    | |
| LICENSE-MIT                                           | +1    | |
| NEWS                                                  | +1    | |
| README.&#8203;md                                      | +1    | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | 0     | Minor unnecessary/overlong unsafe blocks
| fs        | 0     | Rationale of this entire crate
| io        | +1    | All looks sane
| docs      | +1    | Tons of doc comments
| tests     | +1    | Lots of 'em

src/file/imp/unix.rs
--------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 15    | cvt_err           | +1, verified error handling is correct vs online man pages for `rename` and `link`.
| 25    | cvt_err           | +1
| 30    | cstr              | +1
| 35    | create_named      | +1
| 44    | create_unlinked   | +1
| 62    | create            | +1, sane flag use.
| 79    | create            | +1
| 83    | create_unix       | +1, minor hazard to reproducable builds due to random filenames
| 93    | reopen            | +1
| 107   | persist           | 0, `unsafe` larger than necessary, but sound.
| 130   | persist           | 0, redox NYI but sane placeholder error
| 135   | keep              | 0, nothing to do on unix

src/file/imp/windows.rs
-----------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 19    | to_utf16          | +1, null terminates
| 23    | create_named      | +1
| 32    | create            | +1, minor hazard to reproducable builds due to random filenames
| 50    | reopen            | +1, `unsafe` but sound.  Verified error handling vs MSDN.
| 67    | keep              | +1, `unsafe` but sound.  Verified error handling vs MSDN.
| 78    | persist           | 0, `unsafe` larger than necessary, but sound.  Verified error handling vs MSDN.

src/file/mod.rs
---------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| ...   | \*                | +1, well reviewed despite my lack of notes.
| 587   | new_in            | +1, doc comment links wrong method (new_in instead of new)
| ...   | \*                | +1, well reviewed despite my lack of notes.
| 859   | into_file         | 0, confusing how to use these correctly as Drop still occurs
| 867   | into_temp_path    | 0, confusing how to use these correctly as Drop still occurs
| 876   | into_parts        | 0, confusing how to use these correctly as Drop still occurs
| ...   | \*                | +1, well reviewed despite my lack of notes.

src/lib.rs
----------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| ...   | \*                | +1, well reviewed despite my lack of notes.
| 131   | NUM_RETRIES       | -1, absurdly large default value 1 &lt;&lt; 31, will hang "forever".
| ...   | \*                | +1, well reviewed despite my lack of notes.

src/util.rs
-----------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 9     | tmpname           | 0, `unsafe` for semi-pointless str::from_utf8_unchecked, but sound.  Reproducable builds hazard.
| 26    | create_helper     | +1, although I'd pick a different error message.

TIL
---
```rust
let tmp;
if cond {
    tmp = asdf;
    &tmp
}
```
```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TempDir")
        .field("path", &self.path())
        .finish()
}
```
* [std::io::Cursor](https://doc.rust-lang.org/std/io/struct.Cursor.html)
