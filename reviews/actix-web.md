---
category:       Unsound
crev:           dangerous
description:    AVOID.  Closes soundness bugs unfixed.  Deletes external soundness bugs.
---

Cons:
* This project has a history of unsound and unsafe code, as previously covered by
  ["Why we need alternatives to Actix"](https://64.github.io/actix)
* Original repository has been moved and may be taken down - details @ https://gist.github.com/MaulingMonkey/dfc9299fb51b862e4c7fea87f1b57505
* While much of the old unsafe code has been cleaned up and removed, the project has also taken to deleting
  [soundness bugs](http://web.archive.org/web/20200116231317/https://github.com/actix/actix-net/issues/83),
  which are still present despite much work put into cleaning them up by others.
  While I sympathise with the crate author for having gotten on the wrong side of the internet's ire,
  on account of having made the "mistake" of writing something useful enough to get worked up over -
  I share in the harsh conclusion of others:  I don't trust the dude with the `unsafe` keyword.  At all.<br>
  *Especially now that **I can't even trust the issue tracker to list previous soundness issues raised by the community.***
* This kind of crate defeats half the point of using Rust.

Web servers are exposed to the internet, and are thus exposed to malicious users bots and data.
C and C++ have a long enough history of buffer overflow RCEs that I'm convinced they're unsuitable tools for the job.
`unsafe` Rust has similar problems, and especially for networked code, should be avoided without very strong justification.
When very strong justification is provided, thorough code reviews, unit tests, fuzzing, etc. should be carried out to ensure it's correct.
I don't always meet my ideals on this matter either, but I at least strive to.

Actix is written to a much lesser standard: "Ehh, it's fine."

If that's an acceptable standard for you, prefer a mature server framework written in another language - even a C or C++ one.
They'll likely scale better, have better stability, a proper CVE process, lots of scrutiny, and large scale customers who won't accept "ehh, it's fine" as an answer.
