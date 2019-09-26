Pros:
* Cross platform

Cons:
* Soundness issues
* Data races

0.3.1
=====
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | negative

| issue | severity  | desc |
| ----- | --------- | ---- |
| [#3]  | medium    | Getting the cursor pos in ANSI mode can drop stdin data
| [#4]  | high      | Winapi save/restore cursor invokes undefined behavior: `static mut SAVED_CURSOR_POS` not guarded
| [#5]  | high      | Unsound access of possibly invalid `HANDLE`s

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| .github/CODEOWNERS                            | +1 | |
| docs/CONTRIBUTING.md                          | +1 | |
| src/cursor/ansi_cursor.rs                     | +1 | |
| src/cursor/cursor.rs                          | +1 | |
| src/cursor/winapi_cursor.rs                   | +1 | |
| src/sys/unix.rs                               | -1 | \[[#3]\] 45: Getting the cursor pos can drop stdin data
| [src/sys/winapi.rs](src/sys/winapi.rs)        | -1 | \[[#4], [#5]\] Multiple soundness issues
| src/cursor.rs                                 | +1 | |
| src/lib.rs                                    | +1 | |
| src/sys.rs                                    | +1 | |
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | |
| Cargo.toml                                    | +1 | MIT, dep: winapi
| Cargo.toml.orig                               | +1 | MIT, dep: winapi
| CHANGELOG.md                                  | +1 | |
| LICENSE                                       | +1 | MIT
| README.md                                     | +1 | |


| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1 | Soundness issues
| fs        | +1 | None
| io        |  0 | Drops stdin
| docs      | +1 | |
| tests     | +1 | |

src/sys/winapi.rs
-----------------
| Line  | What                          | Notes |
| -----:| ----------------------------- | ----- |
| 26    | unsafe mut SAVED_CURSOR_POS   | -1, \[[#4]\] Access to static mut is unguarded!  Undefined behavior!  Unsound!
| 68    | fn Cursor::goto               | 0,  \[[#5]\] `unsafe { ... }` - would be valid if screen buffer handle was guaranteed valid
| 86    | fn Cursor::set_visibility     | 0,  \[[#5]\] `unsafe { ... }` - would be valid if screen buffer handle was guaranteed valid
| 101   | fn Cursor::restore_cursor_pos | -1, \[[#4]\] Access to static mut is unguarded!  Undefined behavior!  Unsound!
| 114   | fn Cursor::save_cursor_pos    | -1, \[[#4]\] Access to static mut is unguarded!  Undefined behavior!  Unsound!
| 121   | impl From<Handle> for Cusror  | ??, \[[#5]\] Not sure if Handle is guaranteed to be valid
| 129   | impl From<HANDLE> for Cursor  | -1, \[[#5]\] no guarantee HANDLE is valid, unsound!

[#3]:   https://github.com/crossterm-rs/crossterm-cursor/issues/3
[#4]:   https://github.com/crossterm-rs/crossterm-cursor/issues/4
[#5]:   https://github.com/crossterm-rs/crossterm-cursor/issues/5
