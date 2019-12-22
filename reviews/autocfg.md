Preamble
========

Runs `rustc` to test for features / versions.

0.1.7
=====
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | positive

LGTM, starts using RUSTFLAGS

0.1.6
=====
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | positive

Reads env vars and executes the program in them, or whatever program happens to
be called `rustc`... but given that this thing's purpose is to probe rustc
versions, that's kinda inevitable.  Abuseable but certainly not malicious; it's
made for build scripts and it's fine for this purpose.

0.1.5
=====
| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | positive

* No unsafe code
* Minor safe-looking file I/O
