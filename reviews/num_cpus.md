---
category:       General Utility
description:    Queries the OS for the number of CPU cores you have
---

# num_cpus

Queries the OS for the number of CPU cores you have.

Pros:
*   You didn't have to write it.
*   Handles all that hideously platform specific nonsense for you.

Cons:
*   Lots of (necessary) `unsafe`
*   Linux cgroups support seems wildly overcomplicated.
*   [Lies](https://github.com/seanmonstar/num_cpus/issues/95) and
    [lies](https://github.com/seanmonstar/num_cpus/issues/69) and
    [lies](https://github.com/seanmonstar/num_cpus/issues/34) and
    [lies](https://github.com/seanmonstar/num_cpus/issues/81).
    Not exactly this crate's fault - the system APIs are brittle and full of edge cases.
    Multi-processor **architectures** are full of edge cases.

Alternatives:
*   Just hardcode a reasonable number of threads for your workload!
    Spinning up threads for 64 CPU cores to all false-share a single cacheline
    because that's how many logical cores were detected isn't the right choice!
    And all your threads are probably blocked on I/O anyways!

## Issues


| issue                     | severity  | broke     | fix      | desc |
| ------------------------- | --------- | --------- | -------- | ---- |
| N/A                       | ❗️ high    | [0.2.0]   | [0.2.1]  | Incorrect hardcoded unix-wide sysconf constant
| [#83]                     | ⚠️ medium | [0.2.0]   | [1.11.1] | [std::mem::uninitialized] was probably technically UB

[#83]:  https://github.com/seanmonstar/num_cpus/pull/83

## Audit

| version   | thoroughness | understanding | rating | notes |
| --------- | ------------ | ------------- | ------ | ----- |
| [1.13.0]  | low       | low       |✔️ positive| cgroups stuff
| [1.12.0]  | medium    | medium    |✔️ positive|
| [1.11.1]  | medium    | medium    |✔️ positive| ~~[#83]~~: [std::mem::uninitialized] -> zeroed
| [1.11.0]  | medium    | medium    |⚠️ negative|
| [1.10.1]  | medium    | medium    |⚠️ negative|
| [1.10.0]  | medium    | medium    |⚠️ negative|
| [1.9.0]   | medium    | medium    |⚠️ negative|
| [1.8.0]   | medium    | medium    |⚠️ negative|
| [1.7.0]   | medium    | medium    |⚠️ negative|
| [1.6.2]   | medium    | medium    |⚠️ negative|
| [1.6.0]   | medium    | medium    |⚠️ negative|
| [1.5.1]   | medium    | medium    |⚠️ negative|
| [1.5.0]   | medium    | medium    |⚠️ negative|
| [1.4.0]   | medium    | medium    |⚠️ negative|
| [1.3.0]   | medium    | medium    |⚠️ negative|
| [1.2.1]   | medium    | medium    |⚠️ negative|
| [1.2.0]   | medium    | medium    |⚠️ negative|
| [1.1.0]   | medium    | medium    |⚠️ negative|
| [1.0.0]   | medium    | medium    |⚠️ negative|
| [0.2.13]  | medium    | medium    |⚠️ negative|
| [0.2.12]  | medium    | medium    |⚠️ negative|
| [0.2.11]  | medium    | medium    |⚠️ negative|
| [0.2.10]  | medium    | medium    |⚠️ negative|
| [0.2.9]   | medium    | medium    |⚠️ negative|
| [0.2.8]   | medium    | medium    |⚠️ negative|
| [0.2.7]   | medium    | medium    |⚠️ negative|
| [0.2.6]   | medium    | medium    |⚠️ negative|
| [0.2.5]   | medium    | medium    |⚠️ negative|
| [0.2.4]   | medium    | medium    |⚠️ negative|
| [0.2.3]   | medium    | medium    |⚠️ negative|
| [0.2.2]   | medium    | medium    |⚠️ negative|
| [0.2.1]   | medium    | medium    |⚠️ negative| Stopped using bogus "unix"-wide syscalls
| [0.2.0]   | medium    | medium    |❗️ negative | Rust rewrite.<br>Syscall constants incorrect on some platforms.<br>[#83]: uses [std::mem::uninitialized]
| [0.1.0]   | medium    | medium    |⚠️ negative| Full review.  GCC, C code

<!--
    thoroughness:   none low medium high
    understanding:  none low medium high
    rating:         dangerous negative neutral positive strong
-->

[1.13.0]: #1.13.0
[1.12.0]: #1.12.0
[1.11.1]: #1.11.1
[1.11.0]: #1.11.0
[1.10.1]: #1.10.1
[1.10.0]: #1.10.0
[1.9.0]: #1.9.0
[1.8.0]: #1.8.0
[1.7.0]: #1.7.0
[1.6.2]: #1.6.2
[1.6.0]: #1.6.0
[1.5.1]: #1.5.1
[1.5.0]: #1.5.0
[1.4.0]: #1.4.0
[1.3.0]: #1.3.0
[1.2.1]: #1.2.1
[1.2.0]: #1.2.0
[1.1.0]: #1.1.0
[1.0.0]: #1.0.0
[0.2.13]: #0.2.13
[0.2.12]: #0.2.12
[0.2.11]: #0.2.11
[0.2.10]: #0.2.10
[0.2.9]: #0.2.9
[0.2.8]: #0.2.8
[0.2.7]: #0.2.7
[0.2.6]: #0.2.6
[0.2.5]: #0.2.5
[0.2.4]: #0.2.4
[0.2.3]: #0.2.3
[0.2.2]: #0.2.2
[0.2.1]: #0.2.1
[0.2.0]: #0.2.0
[0.1.0]: #0.1.0

<h2 name="1.13.0">1.13.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | MSRV 1.13.0, more docker, xenial dist
| CHANGELOG<span>.</span>md                                 | ✔️
| Cargo<span>.</span>lock                                   | ✔️ | -`doc-comment`
| Cargo<span>.</span>toml                                   | ✔️ | -`doc-comment`
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | -`doc-comment`
| ci\cgroups\Dockerfile                                     | ✔️
| fixtures\cgroups\cgroups\ceil\cpu<span>.</span>cfs_period_us          | ✔️
| fixtures\cgroups\cgroups\ceil\cpu<span>.</span>cfs_quota_us           | ✔️
| fixtures\cgroups\cgroups\good\cpu<span>.</span>cfs_period_us          | ✔️
| fixtures\cgroups\cgroups\good\cpu<span>.</span>cfs_quota_us           | ✔️
| fixtures\cgroups\cgroups\zero-period\cpu<span>.</span>cfs_period_us   | ✔️
| fixtures\cgroups\cgroups\zero-period\cpu<span>.</span>cfs_quota_us    | ✔️
| fixtures\cgroups\proc\cgroups\cgroup                      | ❔ | I don't know crgoup stuff well enough to audit this
| fixtures\cgroups\proc\cgroups\mountinfo                   | ❔ | I don't know crgoup stuff well enough to audit this
| src\lib<span>.</span>rs                                   | ✔️ | Docs, seperated out linux support
| src\linux<span>.</span>rs                                 | ❔

<h2 name="1.13.0/src/linux.rs">src/linux.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 40    | ✔️ `unsafe { std::mem::zeroed::<libc::cpu_set_t>() }` appears sound
| 41    | ✔️ `unsafe { sched_getaffinity(...) }` appears sound
| 44    | ❔ `unsafe { CPU_ISSET(0..CPU_SETSIZE, &set) }` probably sound?
| 50    | ✔️ `unsafe { sysconf(_SC_NPROCESSORS_ONLN) }` appears sound
| ...   | Skimmed... seems OK?

<h2 name="1.12.0">1.12.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️

<h2 name="1.11.1">1.11.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | ~~[#83]~~ Stopped using [std::mem::uninitialized]

<h2 name="1.11.0">1.11.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| Cargo<span>.</span>lock                                   | ✔️ | New
| Cargo<span>.</span>toml                                   | ✔️ | `hermit-abi`
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | `hermit-abi`
| num_cpus-*<span>.</span>events                            | ⚠️ | Should not be part of the package, but harmless
| num_cpus-*<span>.</span>string_data                       | ⚠️ | Should not be part of the package, but harmless
| num_cpus-*<span>.</span>string_index                      | ⚠️ | Should not be part of the package, but harmless
| src\lib<span>.</span>rs                                   | ✔️ | -bitrig, +hermit, revamp /proc/cpuinfo parsing

<h2 name="1.10.1">1.10.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | +`doc-comment` (dev)
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | +`doc-comment` (dev)
| examples\values<span>.</span>rs                           | ✔️ | Trivial example
| src\lib<span>.</span>rs                                   | ❗️ | Whitespace, doctest, haiku support, [std::mem::uninitialized]

<h2 name="1.10.1/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 394   | ⚠️ ABI appears to match current [system_info](https://github.com/haiku/haiku/blob/c253ef85dfc61516df506c35842ea918a572f39c/headers/os/kernel/OS.h#L429-L466) at least.  Differs from [The Be Book](https://www.haiku-os.org/legacy-docs/bebook/TheKernelKit_SystemInfo.html) which has a confusing URL.
| 423   | ✔️ ABI appears to match [get_system_info](https://github.com/haiku/haiku/blob/c253ef85dfc61516df506c35842ea918a572f39c/headers/os/kernel/OS.h#L532)
| 426   | ❗️ [std::mem::uninitialized] isn't kosher (prefer [MaybeUninit])

* [NAME_MAX](https://github.com/haiku/haiku/blob/4bfc0072e3fedc0d5773151b5ba88172af7e5693/headers/posix/limits.h#L38) == 256
* [B_FILE_NAME_LENGTH](https://github.com/haiku/haiku/blob/abb59d7351c7ddb50c63c40430a82d94fa61917a/headers/os/storage/StorageDefs.h#L16) == NAME_MAX == 256
* [B_OS_NAME_LENGTH](https://github.com/haiku/haiku/blob/c253ef85dfc61516df506c35842ea918a572f39c/headers/os/kernel/OS.h#L23) == 32
* [bigtime_t](https://github.com/haiku/haiku/blob/c253ef85dfc61516df506c35842ea918a572f39c/headers/os/support/SupportDefs.h#L54) == i64

<h2 name="1.10.0">1.10.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | fuchsia broken CI workaround
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | +`illumos`, generalize cores=1 fallback

<h2 name="1.9.0">1.9.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo_vcs_info<span>.</span>json            | ✔️
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | sgx, wasm32 (emscripten) cores = 1

<h2 name="1.8.0">1.8.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| CHANGELOG<span>.</span>md                                 | ✔️
| src\lib<span>.</span>rs                                   | ✔️ | Improved docs, wasm32 (!emscripten) support

<h2 name="1.7.0">1.7.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | Multiplatform rewrite
| CHANGELOG<span>.</span>md                                 | ✔️
| CONTRIBUTING<span>.</span>md                              | ✔️ | Restates existing `Apache-2.0 OR MIT` license
| Cargo<span>.</span>toml                                   | ✔️ | Readme, email change, libc "0.2.6" -> "0.2.26"
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | Readme, email change, libc "0.2.6" -> "0.2.26"
| README<span>.</span>md                                    | ✔️ | Changelog link, remove license/contribution sections
| src\lib<span>.</span>rs                                   | ⚠️ | Hardcoded html_root_url for some reason, macos `hw.physicalcpu` read, handle offline ARM cores

<h2 name="1.7.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
|  13   | ⚠️ Hardcoded html_root_url to 1.7.0
| 261   | ✔️ `unsafe { sysctlbyname(b"hw.physicalcpu\0", ...) }` - appears sound, sane
| 301   | ✔️ Handle offline ARM cores

<h2 name="1.6.2">1.6.2</h2>

Trivial (revert 1.6.1 yank?)

<h2 name="1.6.0">1.6.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | get_physical for windows

<h2 name="1.6.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 65    | ✔️ Appears ABI compatible with Win32 [SYSTEM_LOGICAL_PROCESSOR_INFORMATION](https://docs.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-system_logical_processor_information)
| 72    | ✔️ Appears ABI compatible with Win32 [GetLogicalProcessorInformation](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getlogicalprocessorinformation)
| 83    | ✔️ `unsafe { GetLogicalProcessorInformation(...) }` - appears sound, although no dynamic loading minspecs XP SP3
| 102   | ✔️ `unsafe { GetLogicalProcessorInformation(...) }` - appears sound (buffer should be big enough), although I'd prefer a buf-related size calc to make that more obvious
| 113   | ✔️ `unsafe { buf.set_len(...) }` - appears sound, count rounded down on partial reads which should be "impossible" anyways
| 290   | ✔️ mod tests, env-provided expected values

<h2 name="1.5.1">1.5.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | cpus.max(1)

<h2 name="1.5.0">1.5.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>appveyor<span>.</span>yml                   | ✔️ | New: more CI coverage!
| Cargo<span>.</span>toml                                   | ✔️ | New: Autogen
| Cargo<span>.</span>toml<span>.</span>orig                 | ✔️ | Autogen, libc "0.2" -> "0.2.6"
| README<span>.</span>md                                    | ✔️ | CI links, docs link
| src\lib<span>.</span>rs                                   | ✔️ | Better doc comments, make linux try `sched_getaffinity` first

<h2 name="1.4.0">1.4.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| README<span>.</span>md                                    | ✔️ | Description change, fix Cargo.toml syntax highlighting
| src\lib<span>.</span>rs                                   | ✔️ | haiku fallback

<h2 name="1.3.0">1.3.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | category
| src\lib<span>.</span>rs                                   | ✔️ | +redox fallback

<h2 name="1.2.1">1.2.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>travis<span>.</span>yml                     | ✔️ | New: CI coverage!
| Cargo<span>.</span>toml                                   | ✔️ | docs link, keywords
| src\lib<span>.</span>rs                                   | ✔️ | also parse physical id

<h2 name="1.2.0">1.2.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | Indentation, faster get_physical fallback on !linux, test coverage, emscripten fallback

<h2 name="1.1.0">1.1.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| README<span>.</span>md                                    | ✔️ | +crates.io link, num_cpus dependency docs fix
| src\lib<span>.</span>rs                                   | ✔️ | get_physical (parses "/proc/cpuinfo")

<h2 name="1.0.0">1.0.0</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | keywords

<h2 name="0.2.13">0.2.13</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | +`solaris`

<h2 name="0.2.12">0.2.12</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | `MIT/Apache-2.0`
| LICENSE-APACHE                                            | ✔️ | `Apache-2.0`
| LICENSE-MIT                                               | ✔️ | `MIT`
| README<span>.</span>md                                    | ✔️ | `Apache-2.0 OR MIT`

* \[[#19](https://github.com/seanmonstar/num_cpus/issues/19)\] Relicensed from MIT -> "MIT/Apache-2.0".  Appears to have been done correctly!

<h2 name="0.2.11">0.2.11</h2>

More uplifting

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | -`winapi`, -`kernel32-sys`
| src\lib<span>.</span>rs                                   | ✔️

<h2 name="0.2.11/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 18    | ✔️ Layout should be compatible with Win32 [SYSTEM_INFO](https://docs.microsoft.com/en-us/windows/win32/api/sysinfoapi/ns-sysinfoapi-system_info)
| 33    | ✔️ extern ABI / signature appears correct
| 36+   | ✔️ `unsafe { ... }` changes: use local fn, "uplifted" libc constants, add defensive coding

<h2 name="0.2.10">0.2.10</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | libc::funcs::bsd44::sysctl -> libc::sysctl

<h2 name="0.2.9">0.2.9</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | winapi 0.2, kernel32-sys 0.2
| src\lib<span>.</span>rs                                   | ✔️ | winapi 0.2, kernel32-sys 0.2

<h2 name="0.2.8">0.2.8</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | libc "0.1.7" -> "0.2"
| README<span>.</span>md                                    | ✔️ | num_cpus "*" -> "0.2"

<h2 name="0.2.7">0.2.7</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ✔️ | +netbsd support

<h2 name="0.2.6">0.2.6</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ⚠️ | Extra ptr casting nonsense?

<h2 name="0.2.5">0.2.5</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | libc "*" -> "0.1.7"
| src\lib<span>.</span>rs                                   | ⚠️ | Offloaded some constants to libc

<h2 name="0.2.4">0.2.4</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ⚠️ | +android support (unverified constant)

<h2 name="0.2.3">0.2.3</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ⚠️ | "Fixed" iOS, unix still bogus

<h2 name="0.2.2">0.2.2</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ⚠️ | +nacl support (unverified constant)

<h2 name="0.2.1">0.2.1</h2>

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| src\lib<span>.</span>rs                                   | ⚠️ | "Fixed" apple, iOS - per-platform syscalls (unverified constant)

<h2 name="0.2.0">0.2.0</h2>

Dropped GCC for pure rust

| Diff                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| Cargo<span>.</span>toml                                   | ✔️ | -`gcc`
| src\lib<span>.</span>rs                                   | ❗️ | bogus syscalls, [std::mem::uninitialized]

<h2 name="0.2.0/src/lib.rs">src/lib.rs</h2>

| Line  | Notes |
| -----:| ----- |
| 16    | ❗️ [std::mem::uninitialized] isn't kosher (should be [MaybeUninit] but that might've not yet existed)
| 17    | ✔️ `unsafe { ... }` - GetSystemInfo appears infalliable and shouldn't read sysinfo, so this appears sound
| 46    | ❔ `mib` could be len-4
| 48    | ⚠️ `unsafe { sysctl(...) }` - sane params, although I haven't verified HW_AVAILCPU's value.  At least libc agrees?
| 55    | ✔️ `unsafe { sysctl(...) }` - sane params
| 67    | ❗️ sound but wrong - these vary too much by OS

Ref:
* [sysctl](https://www.freebsd.org/cgi/man.cgi?sysctl(3))
* [CTL_HW](https://github.com/freebsd/freebsd/blob/master/sys/sys/sysctl.h#L927)
* [HW_NCPU](https://github.com/freebsd/freebsd/blob/master/sys/sys/sysctl.h#L1038)
* [_SC_NPROCESSORS_ONLN = 58](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.3/src/libc/unix/bsd/apple/mod.rs.html#1089) on apple
* [_SC_THREAD_CPUTIME = 84](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.3/src/libc/unix/bsd/apple/mod.rs.html#1109) on apple

<h2 name="0.1.0">0.1.0</h2>

Full Review

| File                                                      | Rating | Notes |
| --------------------------------------------------------- | ------ | ----- |
| <span>.</span>cargo-ok                                    | ✔️
| <span>.</span>gitignore                                   | ✔️
| Cargo<span>.</span>toml                                   | ✔️ | `MIT`
| LICENSE                                                   | ✔️ | `MIT`
| README<span>.</span>md                                    | ✔️ | `MIT`
| build<span>.</span>rs                                     | ⚠️ | gcc
| extern\num_cpus<span>.</span>c                            | ❗️  | **Might** be sound, but this is full of uninitialized vars, skipped error handling, and poor indentation that doesn't make it very obvious.
| src\lib<span>.</span>rs                                   | ⚠️ | `unsafe` - sound, fn signatures match

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | ❗️ | C code
| fs        | ✔️ | None
| io        | ✔️ | Appropriate syscalls only
| docs      | ✔️
| tests     | ✔️

[std::mem::uninitialized]:  https://doc.rust-lang.org/std/mem/fn.uninitialized.html
[MaybeUninit]:              https://doc.rust-lang.org/std/mem/union.MaybeUninit.html

<!-- Templates

✔️❔⚠️❗️

#### :exclamation:  \[1\] Unsound ...
#### \[1\] Note ...
[1]: #exclamation--1-unsound-...
[2]: #1-note-...
[user/repository#1]: https://github.com/user/repository/issues/1
[user/repository#1]: https://github.com/user/repository/pull/1



# DiffVersionTemplate

| diff                  | rating | notes |
| --------------------- | ------ | ----- |
| 

# Full File Version Template

| Line  | Notes |
| -----:| ----- |
| 

-->
