Preamble
========

General review concerns:
* [XML Bombs](https://en.wikipedia.org/wiki/Billion_laughs_attack) - DTDs not yet supported
* Anything that might panic or exhibit UB on untrusted user input - none found so far
* Incorrect escaping leading to misparsing or misencoding

0.8.0
=====

Fairly full review.  Looks solid.
* My eyes glazed over a bit when going through the decode state machine.
* Some of the namespace stuff too.
* Caught [netvl/xml-rs#179](https://github.com/netvl/xml-rs/issues/179) at least

Pros:
* Safe code!

Cons:
* Probably slower than quick-xml
* Encoding XML not 100% bug free yet
* No DTD support (yet?)

| File                                                  | Rating | Notes |
| ----------------------------------------------------- | ------ | ----- |
| src/reader/parser/inside_cdata.rs                     | +1 | Going through tokenizer at all unnerves me slightly
| src/reader/parser/inside_closing_tag_name.rs          | +1 | |
| src/reader/parser/inside_comment.rs                   | +1 | |
| src/reader/parser/inside_declaration.rs               | +1 | |
| src/reader/parser/inside_doctype.rs                   | +1 | |
| src/reader/parser/inside_opening_tag.rs               | +1 | |
| src/reader/parser/inside_processing_instruction.rs    | +1 | |
| [src/reader/parser/inside_reference.rs](#src/reader/parser/inside_reference.rs) | +1 | |
| src/reader/parser/mod.rs                              | +1 | |
| src/reader/parser/outside_tag.rs                      | +1 | |
| src/reader/config.rs                                  | +1 | |
| src/reader/error.rs                                   | +1 | |
| src/reader/events.rs                                  | +1 | |
| src/reader/lexer.rs                                   | +1 | |
| src/reader/mod.rs                                     | +1 | |
| src/writer/config.rs                                  | +1 | Two space indents by default is heresy but whatever.
| src/writer/emitter.rs                                 | 0 | Encodings not escaped, namespace URIs not escaped.  General attributes *are* escaped though.  CDATA containing ]]> not fixed.
| src/writer/events.rs                                  | +1 | |
| src/writer/mod.rs                                     | +1 | |
| src/analyze.rs                                        | +1 | Should really be moved to bins or examples or something.
| src/attribute.rs                                      | +1 | |
| src/common.rs                                         | 0 | Caught [netvl/xml-rs#179](https://github.com/netvl/xml-rs/issues/179)
| src/escape.rs                                         | +1 | |
| src/lib.rs                                            | +1 | |
| src/macros.rs                                         | +1 | |
| src/name.rs                                           | +1 | |
| src/namespace.rs                                      | +1 | |
| src/util.rs                                           | +1 | |
| tests/documents/sample_1_full.txt                     | +1 | skimmed
| tests/documents/sample_1_short.txt                    | +1 | skimmed
| tests/documents/sample_1.xml                          | +1 | skimmed
| tests/documents/sample_2_full.txt                     | +1 | skimmed
| tests/documents/sample_2_short.txt                    | +1 | skimmed
| tests/documents/sample_2.xml                          | +1 | skimmed
| tests/documents/sample_3_full.txt                     | +1 | skimmed
| tests/documents/sample_3_short.txt                    | +1 | skimmed
| tests/documents/sample_3.xml                          | +1 | skimmed
| tests/documents/sample_4_full.txt                     | +1 | skimmed
| tests/documents/sample_4_short.txt                    | +1 | skimmed
| tests/documents/sample_4.xml                          | +1 | skimmed
| tests/documents/sample_5_short.txt                    | +1 | skimmed
| tests/documents/sample_5.xml                          | +1 | skimmed
| tests/event_reader.rs                                 | +1 | |
| tests/event_writer.rs                                 | +1 | |
| tests/streaming.rs                                    | +1 | |
| .cargo-ok                                             | +1 | |
| .gitignore                                            | +1 | |
| .travis.yml                                           | +1 | Installs pip travis-cargo
| Cargo.toml                                            | +1 | |
| Cargo.toml.orig                                       | +1 | |
| Changelog.md                                          | +1 | |
| design.md                                             | +1 | TODO list
| LICENSE                                               | +1 | MIT, matching Cargo.toml
| Readme.md                                             | +1 | MIT Licensed

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | +1    | One small use in test case, PR to remove upstream and apply deny(unsafe_code) lint.
| fs        | +1    | Only in analyze (and maybe tests?), and sanely
| io        | +1    | |
| docs      | +1    | |
| tests     | 0     | Needs more fuzz tests

src/reader/parser/inside_reference.rs
-------------------------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 23    | predefined XML entities   | Apparently these 5 are the only predefined entities in XML.  Don't have to worry about the hundreds HTML supports.
| 52    | custom XML entities       | Not recursive, no XML bomb here unless DTD constructed a huge entry for `extra_entities` already.

TIL
===
* `&impl ?Sized+AsRef<str>`
