Pros:
* Handles console input

Cons:
* Soundness issues
* Not browser compatible

0.4.1
=====
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | negative

| issue | severity  | desc |
| ----- | --------- | ---- |
| [#245]| high      | Unguarded access of static mut ORIG_MODE is unsound

| File                                                      | Rating | Notes |
| ---------------------------------------------             | ------ | ----- |
| .github/CODEOWNERS                                        | +1 | |
| docs/CONTRIBUTING.md                                      | +1 | |
| src/input/input.rs                                        | +1 | |
| [src/input/unix_input.rs](src/input/unix_input.rs)        | -1 | Parsing looks off, panicy internals
| [src/input/windows_input.rs](src/input/windows_input.rs)  | -1 | Unsound \[[#245]\], very strange keyboard handling.
| [src/sys/unix.rs](src/sys/unix.rs)                        |  0 | Mishandles `read == 0`? |
| src/input.rs                                              | +1 | |
| src/lib.rs                                                | +1 | |
| src/sys.rs                                                | +1 | |
| .cargo_vcs_info.json                                      | +1 | |
| .cargo-ok                                                 | +1 | |
| .gitignore                                                | +1 | |
| travis.yml                                                | +1 | |
| Cargo.toml                                                | +1 | MIT, winapi, libc, optional serde
| Cargo.toml.orig                                           | +1 | MIT, winapi, libc, optional serde
| CHANGELOG.md                                              | +1 | |
| LICENSE                                                   | +1 | MIT
| README.md                                                 | +1 | MIT

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1 | Unsound
| fs        | +1 | `/dev/tty` access, but that's expected
| io        | +1 | Sound... probably
| docs      | +1 | |
| tests     |  0 | Admittedly hard to unit test

src/input/unix_input.rs
-----------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 198   | fn SyncReader::next   | -1, Disambiguating ESC like this seems super sketchy/brittle.
| 261   | fn parse_event        | -1, This is more like what ESC parsing *should* look like...?
| 269   | fn parse_event        | -1, \\r\\n -> Enter Enter?  Seems wrong...
| 312   | fn parse_csi          | -1, `.unwrap` Panic city
| EOF   | | |

src/input/windows_input.rs
--------------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 42    | static mut ORIG_MODE                  | -1, More unsound access \[[#2]\]
| 47    | WindowsInput::read_char               | +1, `unsafe { ... }` - willing to assume `_getwche` is sound.
| 110   | fn WindowsInput::enable_mouse_mode    | -1, `unsafe { ... }` - unsound access of ORIG_MODE! \[[#245]\]
| 116   | fn WindowsInput::disable_mouse_mode   | -1, `unsafe { ... }` - unsound access of ORIG_MODE! \[[#245]\]
| 225   | fn read_single_event                  | 0, `unsafe { ... }` - `KeyEventRecord::from(*input.event.KeyEvent())` is *probably* sound/safe?
| 228   | fn read_single_event                  | 0, `unsafe { ... }` - `MouseEvent::from(*input.event.MouseEvent())` is *probably* sound/safe?
| 249   | fn read_input_events                  | 0, `unsafe { ... }` - `KeyEventRecord::from(*input.event.KeyEvent())` is *probably* sound/safe?
| 256   | fn read_input_events                  | 0, `unsafe { ... }` - `MouseEvent::from(*input.event.MouseEvent())` is *probably* sound/safe?
| 291   | fn parse_key_event_record             | 0, Several keys are dead, apparently
| 303   | fn parse_key_event_record             | 0, Strange enumeration values for KeyEvent
| 345   | fn parse_key_event_record             | -1, either VK_PRIOR \| VK_NEXT can be sanely simplified a lot or something is super fucky.
| 354   | fn parse_key_event_record             | -1, either VK_END \| VK_HOME can be sanely simplified a lot or something is super fucky.
| 367   | fn parse_key_event_record             | 0, `unsafe { ... }` assumes UnicodeChar is valid.  Private fn only called on win32 results... technically unsound of winapi only populated AsciiChar, but that would be super dumb.
| EOF   | | |

src/sys/unix.rs
---------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 19    | fn get_tty_fd                         | +1, `unsafe { ... }` looks sound
| 45    | fn read_char_raw                      | 0, `read == 0` can probably happen when pipe broken? generates extra ' '?
| EOF   | | |

[#245]: https://github.com/crossterm-rs/crossterm/issues/245
