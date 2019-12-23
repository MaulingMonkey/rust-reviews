---
category:       Gamedev
description:    Pure rust MIDI device I/O. Good start, but probably unsound.
---

# midir

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.5.0](#0.5.0) | low | low | negative

Strongly suspect some unsoundness in the backends, a few strange API decisions, but good bones and pure Rust.

# 0.5.0

Good starting point, and my favorite Rust MIDI API so far, but likely unsound as stands.

Pros:
- Pure rust, no mucking with building building C/C++ libs like with portmidi
- To be WASM/Browser compatible (PR made).

Cons:
- Likely unsound as currently stands (transmute_copy, type punning, sketchy memalloc crate use, haven't vetted thread safety properly, ...)
- API clunky in spots (mix of known issues and easily fixed surface level stuff)
- Needs more unit test coverage if possible (are there perhaps virtual MIDI devices for windows that could be added to CI...?)

TODO:
- rating: netural: Eliminate as much sketchy unsafe as possible.
- rating: positive: Clean up API design a bit (async, saner member functions, deal with Send inconsistency, maybe make connecting not consume MidiInput s?)

Detail
------

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| examples/test_forward.rs                      | +1 | |
| examples/test_play.rs                         | +1 | |
| examples/test_reuse.rs                        | +1 | |
| examples/test_sysex.rs                        | +1 | |
| [src/backend/asla/mod.rs](src/backend/asla/mod.rs) | 0 | Some extra `unsafe`, use of uninit data possibly UB
| src/backend/coremidi/mod.rs                   | 0 | Is Core MIDI thread safe?  (`unsafe impl Send for MidiOutputConnection`)
| src/jack/mod.rs                               | -1 | Use of transmute_copy on Box is skeeeeeeetchy, uninitialized too :(
| src/jack/wrappers.rs                          | -1 | Is JACK thread safe?  Ringbuffer::read is unsound!  Lots of unsafe but mostly for sane FFI.
| src/winmm/handler.rs                          | 0 | unsafe for FFI, some pointer casts I haven't thoroughly vetted |
| src/winmm/mod.rs                              | 0 | unsafe for FFI, uninitialized :(, sketchy deallocate API.  Is WinMM thread safe?  333: UB &mut violates aliasing?
| src/backend/mod.rs                            | +1 | |
| src/os/mod.rs                                 | +1 | |
| src/os/unix.rs                                | +1 | |
| src/common.rs                                 | 0 | Consumption of the MidiInput/MidiOutput clients/factories on connect seems a bit strange, especially after the ports refactor.
| src/errors.rs                                 | +1 | |
| src/lib.rs                                    | +1 | unsafe, but sound |
| tests/virtual.rs                              | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | Downloads/installs jack from the internet |
| appveyor.yml                                  | +1 | |
| Cargo.toml                                    | +1 | |
| Cargo.toml.orig                               | +1 | |
| LICENSE                                       | +1 | |
| README.md                                     | +1 | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1    | 99% sure something in here is unsound.  transmute_copy, type punning, use of memalloc crate.
| fs        | +1    | N/A |
| io        | 0     | libc... maybe safe? |
| docs      | +1    | |
| tests     | 0     | Good examples, but needs more automated unit/integration tests.  Admittedly hard given the lack of MIDI devices on CI servers... |

src/backend/asla/mod.rs
-----------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  21 | fn poll                             | +1
|  81 | unsafe impl Send for EventEncoder   | Is ASLA thread safe?
| 578 | unsafe in handle_input              | Construction of uninitialized vec, UB?
