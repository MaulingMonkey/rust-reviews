---
category:       Serialization
description:    Fast integer I/O
---

# itoa

# Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [0.4.4]   | low | low | neutral | Pretty thorough review

[0.4.4]: #044



# 0.4.4

I believe this is sound, but has way too much unsafe and too few tests for my tastes.
udivmod_1e19 reduces my thoroughness and understanding to low, otherwise both would be medium or higher.

## Reviewed

```
benches/bench.rs    +1

src/lib.rs          0
    119 mem::uninitialized (POD)
    164 impl_IntegerCommon is potentially unsound, but not public, and guarded by debug_assert!s at least.
        Would vastly prefer static_assertions though.
    196 one giant unsafe block for pointer math?  No bounds checks?  Super gross!
        Relies on $max_len being enough for $t with no bounds checking.
        Relies on d1/d2/n being unable to overflow DEC_DIGITS_LUT.
        A careful reading makes this appear technically sound, but super super sketchy.
    246 Carefully audited for impl_Integer.
    255 Carefully audited for impl_Integer.
    267 Carefully audited for impl_Integer.
    270 Carefully audited for impl_Integer.
    273 Carefully audited for impl_Integer.
    294 Again?  A careful reading makes this appear technically sound, but super super sketchy.
    338 Carefully audited for impl_Integer128.
    339 Shouldn't this technically have the cfg if U128_MAX_LEN is going to have it?
    341 Carefully audited for impl_Integer128.

tests/test.rs       +1
    Not nearly enough tests for my liking given how much unsafe code there is.
    I'd like to see every edge case of iota::write tested, they aren't.
    I'd like to see every edge case of udivmod_1e19 tested, it's not directly tested at all.

.gitignore          +1
.travis.yml         +1
Cargo.toml          +1
Cargo.toml.orig     +1
README.md           +1
```

## Not yet fully reviewed:

```
src/udiv128.rs      0
    34  Correct - since high is nonzero (line 29 bails if it wasn't), leading_zeros < 64.
    35+ My eyes glazed over at this point.  I haven't verified udivmod_1e19(...).1 meets it's invariants!
```

## TIL

- You can use \`\`\`edition2018\` on doc comment code blocks.
- You can have a public trait require a private one to "seal" it.
- You can specify `sudo: false` in .travis.yml
