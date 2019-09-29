Pros:
* Styling!

Cons:
* Poor win7 support
* Unsound

0.5.1
=====
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | negative

| issue | severity  | desc |
| ----- | --------- | ---- |
| [#245]| high      | Unguarded access of static mut ORIGINAL_CONSOLE_COLOR is unsound
| [#261]| low       | SetBg command instead sets foreground in win7 non-ansi terms.
| [#262]| low       | Windows 7 doesn't handle styling well / at all
| [#263]| low       | Windows 7 grey/white foreground swapped, background... is just a mess.

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| .github/CODEOWNERS                            | +1 | |
| docs/CONTRIBUTING.md                          | +1 | |
| src/enums/attribute.rs                        | +1 | Verified codes vs https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_parameters
| src/enums/color.rs                            |  0 | FromStr for Color doesn't implement RGB parsing despite supporting RGB
| src/enums/colored.rs                          | +1 | |
| src/ansi_color.rs                             |  0 | Verified codes vs https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit .  Could've simplified match logic a bit.
| src/color.rs                                  | -1 | Bugs ([#261], [#263])
| src/enums.rs                                  | +1 | |
| src/lib.rs                                    | +1 | |
| src/macros.rs                                 | +1 | |
| src/objectstyle.rs                            | +1 | |
| src/styledobject.rs                           |  0 | Odd fg/bg naming style.  Also reset seems suboptimal if nesting styles?
| src/traits.rs                                 | +1 | Not sure how wild I am about &str extension methods, but it works.
| [src/winapi_color.rs](src/winapi_color.rs)    | -1 | Unsound static mut ORIGINAL_CONSOLE_COLOR if original_console_color ever called before init_console_color, which appears possible [#245]
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | No MSRV
| Cargo.toml                                    | +1 | MIT, winapi, crossterm_winapi, serde
| Cargo.toml.orig                               | +1 | MIT, winapi, crossterm_winapi, serde
| CHANGELOG.md                                  | +1 | |
| LICENSE                                       | +1 | MIT
| README.md                                     | +1 | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1 | [#245] unsound static mut
| fs        | +1 | None
| io        |  0 | Not sure what to blame for win7 styling failures
| docs      | +1 | |
| tests     | -1 | Few, hard anyways

src/winapi_color.rs
-------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 46    | fn WinApiColor::set_fg            | mask should be 0x00F0 instead of special casing BACKGROUND_INTENSITY
| 78    | fn WinApiColor::set_bg            | mask should be 0x000F instead of special casing FOREGROUND_INTENSITY
| 118   | fn color_value                    | Isn't Color::White and Color::Grey here swapped in terms of colors to be used?
| 119   | fn color_value                    | Isn't Color::White and Color::Grey here swapped in terms of colors to be used?
| 133   | fn color_value                    | 0 seems like a poor choice for fallback fg color, especially when it's also used for bg color
| 153   | fn color_value                    | Isn't Color::White and Color::Grey here swapped in terms of colors to be used?
| 154   | fn color_value                    | Isn't Color::White and Color::Grey here swapped in terms of colors to be used?
| 133   | fn color_value                    | 0 seems like a mediocre choice for fallback bg color, especially when it's also used for fg color
| 172   | fn color_value                    | Wait why the heck are we going to/from strings that makes 0 sense
| 191   | static mut ORIGINAL_CONSOLE_COLOR | Unsound access if reset called before set_??, which appears possible [#245]

[#245]: https://github.com/crossterm-rs/crossterm/issues/245
[#261]: https://github.com/crossterm-rs/crossterm/issues/261
[#262]: https://github.com/crossterm-rs/crossterm/issues/262
[#263]: https://github.com/crossterm-rs/crossterm/issues/263
