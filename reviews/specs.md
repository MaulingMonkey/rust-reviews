---
category:       Gamedev
description:    High boilerplate ECS.  Fancy and parallel though.
---

# specs

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.15.1](#0.15.1) | medium | medium | neutral |

Pros:
* Very well documented
* Well tested
* Good attention to detail WRT unsafe code.

Cons:
* Issue [#644] Scary partially uninitialized Vec\<T\>s
* PR [#645] `println!()`s for !is_unconstrained() joins is lame / a perf hazard in and of itself.
* Complicated as heck
* I'm not yet convinced the ParJoin s are sound
* Plenty of unsafe code
    * Some scary lifetime extensions
    * Comments like "// This is horribly unsafe."
* See `-1`s for more details

0.15.1
======

Misc. notes:
* I still don't have a firm grasp on when `world.maintain()` is necessary

| crev          |   |
| ------------- |---|
| thoroughness  | medium
| understanding | medium
| rating        | neutral

| File                                                  | Rating | Notes |
| ----------------------------------------------------- | ------ | ----- |
| .github/ISSUE_TEMPLATE/bug_report.md                  | +1 | |
| .github/ISSUE_TEMPLATE/feature_request.md             | +1 | |
| .github/stale.yml                                     | +1 | |
| benches/benches_main.rs                               | +1 | criterion
| benches/big_or_small.rs                               | +1 | nalgebra, shred
| benches/parallel.rs                                   | +1 | |
| benches/storage_cmp.rs                                | +1 | |
| benches/storage_sparse.rs                             | +1 | |
| benches/world.rs                                      | +1 | rayon
| docs/reference/src/01_system.md                       |  0 | ~empty
| docs/reference/src/intro.md                           | +1 | |
| docs/reference/src/SUMMARY.md                         | +1 | TOC
| docs/reference/book.toml                              | +1 | |
| docs/tutorials/src/images/component-tables.svg        | +1 | \<switch\>?  \<foreignObject\>?  What is this madness...
| docs/tutorials/src/images/entity-component.svg        | +1 | |
| docs/tutorials/src/images/system.svg                  | +1 | |
| docs/tutorials/src/01_intro.md                        | +1 | Java?
| docs/tutorials/src/02_hello_world.md                  |  0 | specs 0.15.0, lots of `rust,ignore`s, poor link names
| docs/tutorials/src/03_dispatcher.md                   | +1 | |
| docs/tutorials/src/04_resources.md                    | +1 | Option\<Read\<...\>\>
| docs/tutorials/src/05_storages.md                     | +1 | |
| docs/tutorials/src/06_system_data.md                  | +1 | |
| docs/tutorials/src/07_setup.md                        | +1 | |
| docs/tutorials/src/08_join.md                         | +1 | |
| docs/tutorials/src/09_parallel_join.md                | +1 | |
| docs/tutorials/src/10_rendering.md                    | +1 | |
| docs/tutorials/src/11_advanced_component.md           | +1 | |
| docs/tutorials/src/12_tracked.md                      | +1 | |
| docs/tutorials/src/13_saveload.md                     | +1 | |
| docs/tutorials/src/14_troubleshooting.md              | +1 | |
| docs/tutorials/src/SUMMARY.md                         | +1 | TOC
| docs/tutorials/book.toml                              | +1 | |
| docs/website/content/pages/about.md                   | +1 | |
| docs/website/content/pages/docs.md                    | +1 | |
| docs/website/content/_index.md                        | +1 | |
| docs/website/content/specs-0.15.md                    | +1 | draft, "Test"
| docs/website/themes/hyde/sass/hyde.scss               | +1 | |
| docs/website/themes/hyde/sass/poole.scss              | +1 | |
| docs/website/themes/hyde/sass/print.scss              | +1 | |
| docs/website/themes/hyde/static/.gitkeep              | +1 | |
| docs/website/themes/hyde/templates/404.html           | +1 | some templating engine used
| docs/website/themes/hyde/templates/index.html         | +1 | |
| docs/website/themes/hyde/templates/page-nodate.html   | +1 | |
| docs/website/themes/hyde/templates/page.html          | +1 | |
| docs/website/themes/hyde/.gitignore                   | +1 | |
| docs/website/themes/hyde/config.toml                  | +1 | |
| docs/website/themes/hyde/LICENSE                      | +1 | MIT
| docs/website/themes/hyde/README.md                    | +1 | |
| docs/website/themes/hyde/theme.toml                   | +1 | |
| docs/website/config.toml                              | +1 | |
| examples/async.rs                                     | +1 | |
| examples/basic.rs                                     | +1 | |
| examples/bitset.rs                                    | +1 | fizzbuzz!  Hah.
| examples/cluster_bomb.rs                              | +1 | |
| examples/full.rs                                      | +1 | `dispatcher.with_barrier()`
| examples/ordered_track.rs                             | +1 | |
| examples/saveload.rs                                  | +1 | ron, structs/impls inside fns
| examples/track.rs                                     | +1 | |
| scripts/build-netlify.sh                              |  0 | Installing things from the internet (zola, mdbook, rustup)
| scripts/kcov.sh                                       | -1 | docker - owns/deletes mykcov1, runs images, disables seccomp
| [src/join/mod.rs](src/join/mod.rs)                    |  0 | I don't fully grok the ParJoin limitations yet, nor the underlying reason behind unsafe fn open/get.
| [src/join/par_join.rs](src/join/par_join.rs)          | -1 | "[...] technically not allowed" (ab)use of UnsafeCell
| src/saveload/de.rs                                    | +1 | |
| src/saveload/marker.rs                                | +1 | |
| src/saveload/mod.rs                                   | +1 | |
| src/saveload/ser.rs                                   | +1 | |
| src/saveload/tests.rs                                 | +1 | |
| src/saveload/uuid.rs                                  | +1 | |
| src/storage/data.rs                                   | +1 | |
| [src/storage/drain.rs](src/storage/drain.rs)          | +1 | |
| [src/storage/entry.rs](src/storage/entry.rs)          | -1 | `Entries::get` seems sketchy
| [src/storage/flagged.rs](src/storage/flagged.rs)      | +1 | |
| src/storage/generic.rs                                | +1 | |
| [src/storage/mod.rs](src/storage/mod.rs)              | -1 | `&mut Storage::get_mut` is sketchy as heck
| [src/storage/restrict.rs](src/storage/restrict.rs)    | -1 | More `ParJoin` I can't wrap my head around, `_unchecked` fns sound unsafe but aren't marked as such.
| [src/storage/storages.rs](src/storage/storages.rs)    | -1 | [#644], `UnprotectedStorage` is a scary API in general for perf reasons.
| [src/storage/tests.rs](src/storage/tests.rs)          | +1 | needs more 
| [src/storage/track.rs](src/storage/track.rs)          |  0 | |
| src/world/comp.rs                                     | +1 | |
| [src/world/entity.rs](src/world/entity.rs)            |  0 | Possibly missing some `self.killed.contains(...)` checks in `Allocator`, ungrokked `ParJoin`
| src/world/lazy.rs                                     | +1 | |
| src/world/mod.rs                                      | +1 | |
| src/world/tests.rs                                    | +1 | |
| [src/world/world_ext.rs](src/world/world_ext.rs)      |  0 | Some concerns about `create_entity_unchecked`, `is_alive`
| [src/bitset.rs](src/bitset.rs)                        | +1 | |
| [src/changeset.rs](src/changeset.rs)                  | -1 | Bypasses lifetime checks
| src/error.rs                                          | +1 | |
| src/lib.rs                                            | +1 | |
| src/prelude.rs                                        | +1 | |
| tests/saveload.rs                                     | +1 | |
| tests/tests.rs                                        | +1 | |
| .cargo_vcs_info.json                                  | +1 | |
| .cargo-ok                                             | +1 | |
| .gitignore                                            | +1 | |
| .rustfmt.toml                                         | +1 | |
| Cargo.lock                                            | +1 | |
| Cargo.toml                                            | +1 | |
| Cargo.toml.orig                                       | +1 | |
| CHANGELOG.md                                          | +1 | I like the link format here
| CODE_OF_CONDUCT.md                                    | +1 | |
| codecov.yml                                           | +1 | |
| CONTRIBUTING.md                                       | +1 | |
| LICENSE-APACHE                                        | +1 | |
| LICENSE-MIT                                           | +1 | |
| netlify.toml                                          | +1 | |
| PULL_REQUEST_TEMPLATE.md                              | +1 | |
| README.md                                             | +1 | Dual MIT/Apache 2.0, could use a deps badge

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1 | ParJoin sounds unsound, scary comments like "// This is horribly unsafe."
| miri      | -1 | N/A (atomics)
| fs        | +1 | N/A
| io        | +1 | N/A
| docs      | +1 | Sets the golden standard for how to document unsafe code in many places... but sadly not all.
| tests     | 0  | Some gaps on a per-module basis, which really seem necessary for some of the scarier nests of unsafe code.

src/join/mod.rs
---------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 227   | unsafe fn Join::open                          | Hmm... will this truly be unsound?  Will need to read more...
| 237   | unsafe fn Join::get                           | "The implementation [...] has no invariants to meet" - I don't grok why this is unsafe yet, but tat least the doc comments seem clear.
| 273   | unsafe fn MaybeJoin::open                     | +1, just forwards, good doc comments about held constraints
| 280   | unsafe fn MaybeJoin::get                      | +1, meets documented constraints, good doc comments about held constraints
| 305   | fn JoinIter::new                              | 0, fix pending: [#645] println! isn't an appropriate mechanism for this and possibly adds another perf issue.  PR switches to log crate.
| 312   | fn JoinIter::new                              | 0, relies on constraint of impl
| 370   | fn JoinIter::get                              | +1, documents and abides by safety constraints
| 386   | fn JoinIter::get_unchecked                    | +1, documents and abides by safety constraints
| 399   | fn JoinIter::next                             | +1, documents and abides by safety constraints
| 422   | unsafe fn (A,B,...)::open                     | +1, documents and abides by safety constraints
| 434   | unsafe fn (A,B,...)::get                      | +1, documents and abides by safety constraints
| 440   |        fn (A,B,...)::is_unconstrained         | +1
| 451   | unsafe impl ParJoin for (A,B,...)             | 0, sounds accurate but I don't fully grok
| 500   | unsafe fn \_Readers\_::open                   | +1, just forwards sanely
| 506   | unsafe fn \_Readers\_::get                    | +1, just forwards sanely
| 511   | unsafe fn \_Readers\_::is_unconstrained       | +1
| 519   | unsafe impl ParJoin for \_Readers\_           | 0, sounds accurate but I don't fully grok
| 542   | unsafe fn \_Writers\_::open                   | +1, just forwards sanely
| 548   | unsafe fn \_Writers\_::get                    | +1, just forwards sanely
| 553   | unsafe fn \_Writers\_::is_unconstrained       | +1
| 561   | unsafe impl ParJoin for \_Writers\_           | 0, sounds accurate but I don't fully grok
| EOF   | | |

src/join/par_join.rs
--------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  21   | fn ParJoin::par_join comment                  | -1, "NOTE: This is currently unspecified behavior." - concerning...
|  30   | fn ParJoin::par_join                          | 0, fix pending: [#645] println! isn't an appropriate mechanism for this and possibly adds another perf issue.  PR switches to log crate.
|  56   | fn JoinParIter::drive_unindexed               | 0, `unsafe { ... }` relies on constraint of impl
|  98   | unsafe impl Send for JoinProducer             | -1, "[...] technically not allowed" abuse of UnsafeCell
| 130   | fn UnindexedProducer::fold_with               | 0, logic makes sense to me here
| EOF   | | |

src/storage/drain.rs
--------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  25   | unsafe fn Drain::open                         | +1, just forwards sanely
|  32   | unsafe fn Drain::get                          | +1, uses remove sanely
| EOF   | | |

src/storage/entry.rs
--------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  41   | fn Storage::entry                             | +1, `unsafe { ... }` documents and abides by safety constraints
| 121   | fn Storage::entries                           | +1
| 140   | unsafe fn Entries::open                       | +1
| 146   | unsafe fn Entries::get                        | -1, `unsafe { ... }` - I can't verify how this lifetime extension is possibly sound either.
| 164   | fn Entries::is_unconstrained                  | +1
| 184   | fn OccupiedEntry::get                         | +1, `unsafe { ... }` - documents and abides by safety constraints
| 197   | fn OccupiedEntry::get_mut                     | +1, `unsafe { ... }` - documents and abides by safety constraints
| 205   | fn OccupiedEntry::into_mut                    | +1, `unsafe { ... }` - documents and abides by safety constraints
| 209   | fn OccupiedEntry::insert                      | +1
| 215   | fn OccupiedEntry::remove                      | +1
| 236   | fn VacantEntry::insert                        | +1, `unsafe { ... }` - documents and abides by safety constraints
| EOF   | | |

src/storage/flagged.rs
----------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 203   | unsafe fn FlaggedStorage::clean               | +1, just forwards sanely
| 210   | unsafe fn FlaggedStorage::get                 | +1, just forwards sanely
| 214   | unsafe fn FlaggedStorage::get_mut             | +1, just forwards sanely
| 221   | unsafe fn FlaggedStorage::insert              | +1, just forwards sanely
| 228   | unsafe fn FlaggedStorage::remove              | +1, just forwards sanely
| EOF   | | |

src/storage/mod.rs
------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  57   | unsafe fn AntiStorage::open                   | +1, safe and sound
|  62   | unsafe fn AntiStorage::get                    | +1, safe and sound
|  66   | unsafe impl DistinctStorage for AntiStorage   | +1, safe and sound
|  70   | unsafe impl ParJoin for AntiStorage           | +1, safe and sound
|  78   | unsafe impl CastFrom for dyn AnyStorage       | 0, don't fully grok why this is unsafe, but looks 100% safe and sound and matches shred examples.
| 122   | unsafe trait DistinctStorage                  | +1, safe and sound
| 155   | fn MaskedStorage::clear                       | +1, `unsafe { ... }` - documents and abides by safety constraints
| 165   | fn MaskedStorage::remove                      | +1, `unsafe { ... }` - documents and abides by safety constraints
| 175   | fn MaskedStorage::drop                        | +1, `unsafe { ... }` - documents and abides by safety constraints
| 231   | fn Storage::get                               | +1, `unsafe { ... }` - documents and abides by safety constraints
| 274   | unsafe fn Storage::unprotected_storage_mut    | +1, documents constraints
| 282   | fn Storage::get_mut                           | +1, `unsafe { ... }` - documents and abides by safety constraints
| 299   | fn Storage::insert                            | +1, `unsafe { ... }` - documents and abides by safety constraints
| 304   | fn Storage::insert                            | +1, `unsafe { ... }` - documents and abides by safety constraints
| 341   | unsafe impl DistinctStorage for Storage       | +1, proper constraints
| 356   | unsafe fn &Storage::open                      | +1, documents constraints
| 362   | unsafe fn &Storage::get                       | +1, just forwards sanely
| 400   | unsafe fn &mut Storage::open                  | +1, documents constraints
| 409   | unsafe fn &mut Storage::get                   | -1, WTF? Lifetime extension? Can't this just be v.get_mut(i)?
| 449   | trait UnprotectedStorage                      | -1, needs to make it clearer that `clear` *must* be called before dropping as currently stands, unless that's just a bug.
| 457   | unsafe fn UnprotectedStorage::clear           | +1, documents constraints
| 472   | unsafe fn UnprotectedStorage::get             | +1, documents constraints
| 485   | unsafe fn UnprotectedStorage::get_mut         | +1, documents constraints
| 496   | unsafe fn UnprotectedStorage::insert          | +1, documents constraints
| 504   | unsafe fn UnprotectedStorage::remove          | +1, documents constraints
| 514   | unsafe fn UnprotectedStorage::drop            | +1, documents constraints, default impl sane/sound
| EOF   | | |

src/storage/restrict.rs
-----------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  83   | unsafe impl ParJoin for &mut RestrictedStorage\<..., MutableParallelRestriction\> | -1, no idea
|  93   | unsafe impl ParJoin for &RestrictedStorage\<..., ImmutableAliasing\>              | -1, no idea
| 113   | unsafe fn &RestrictedStorage::open                                                | 0, could maybe document safety more, but I believe sound?
| 118   | unsafe fn &RestrictedStorage::get                                                 | 0, could maybe document safety more, but I believe sound?
| 144   | unsafe fn &mut RestrictedStorage::open                                            | 0, could maybe document safety more, but I believe sound?
| 149   | unsafe fn &mut RestrictedStorage::get                                             | 0, could maybe document safety more, but I believe sound?
| 239   | fn PairedStorage::get_unchecked                                                   | -1, `unsafe { ... }` - maybe fine, but safety undocumented and get_unchecked sounds like something that should itself be an unsafe fn?
| 252   | fn PariedStorage::get_mut_unchecked                                               | -1, `unsafe { ... }` - maybe fine, but safety undocumented and get_unchecked sounds like something that should itself be an unsafe fn?
| 272   | fn PairedStorage::get                                                             | +1, abides by safety constraints
| 294   | fn PairedStorage::get_mut                                                         | +1, abides by safety constraints
| EOF   | | |

src/storage/storages.rs
-----------------------
Careful auditing gives a good idea for what the actual API requirements of `UnprotectedStorage` are, and it's a doozy.

1. VecStorage makes it unsound to not call `clean`
2. Noop `clean`s on several storages make me think it should consume `self` instead of taking `&mut self` - maybe can't due to API limitations?
3. Need clearer UnprotectedStorage docs

| Line  | What  | Notes |
| -----:| ----- | ----- |
|  20   | unsafe fn BTreeStorage::clean                     | 0, should maybe clear storage?
|  27   | unsafe fn BTreeStorage::get                       | +1
|  31   | unsafe fn BTreeStorage::get_mut                   | +1
|  35   | unsafe fn BTreeStorage::insert                    | +1
|  39   | unsafe fn BTreeStorage::remove                    | +1
|  44   | unsafe impl DistinctStorage for BTreeStorage      | +1 as I grok DistinctStorage
|  54   | unsafe fn HashMapStorage::clean                   | 0, should maybe clear storage?
|  61   | unsafe fn HashMapStorage::get                     | +1
|  65   | unsafe fn HashMapStorage::get_mut                 | +1
|  69   | unsafe fn HashMapStorage::insert                  | +1
|  73   | unsafe fn HashMapStorage::remove                  | +1
|  78   | unsafe impl DistinctStorage for HashMapStorage    | +1
|  95   | unsafe fn DenseVecStorage::clean                  | 0, should maybe clear storage?
| 102   | unsafe fn DenseVecStorage::get                    | 0, unsound on bad index, but fn is unsafe.  Otherwise sound.
| 107   | unsafe fn DenseVecStorage::get_mut                | 0, unsound on bad index, but fn is unsafe.  Otherwise sound.
| 112   | unsafe fn DenseVecStorage::insert                 | 0, `set_len` leaves uninitialized gaps in the data_id Vec.  Maybe sound, but at least violates std docs.  See [#644]
| 124   | unsafe fn DenseVecStorage::remove                 | 0, unsound on bad index, but fn is unsafe.  Otherwise sound.
| 133   | unsafe impl DistinctStorage for DenseVecStorage   | +1
| 143   | unsafe fn NullStorage::clean                      | +1
| 149   | unsafe fn NullStorage::get                        | +1
| 153   | unsafe fn NullStorage::get_mut                    | +1
| 157   | unsafe fn NullStorage::insert                     | +1
| 159   | unsafe fn NullStorage::remove                     | +1
| 178   | unsafe impl DistinctStorage for NullStorage       | +1, precondition enforced by `Default::default()`
| 187   | unsafe fn VecStorage::clean                       | +1
| 200   | unsafe fn VecStorage::get                         | 0, unsound on bad index, but fn is unsafe.  Otherwise sound.
| 204   | unsafe fn VecStorage::get_mut                     | 0, unsound on bad index, but fn is unsafe.  Otherwise sound.
| 208   | unsafe fn VecStorage::insert                      | -1, `set_len` leaves uninitialized gaps in the data_id Vec, making it unsound to drop before calling clean.  Sound if perfectly used (e.g. call clean), but a timebomb as stands IMO.  See [#644]
| 222   | unsafe fn VecStorage::remove                      | 0, unsound on bad index, but fn is unsafe.  Otherwise sound.
| 229   | unsafe impl DistinctStorage for VecStorage        | +1
| EOF   | | |

src/storage/tests.rs
--------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 460   | vec_arc                                       | +1, `unsafe { ... }`
|       |                                               | -1, would like to see more tests involving drop types
|       |                                               | -1, would like to see more tests involving specific storages
| EOF   | | |

src/storage/track.rs
--------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  53   | fn Storage::channel                           | 0, `unsafe { ... }` - I don't grok the safety invariants we need to hold here
|  59   | fn Storage::event_emission                    | 0, `unsafe { ... }` - I don't grok the safety invariants we need to hold here
|  72   | fn Storage::channel_mut                       | 0, `unsafe { ... }` - I don't grok the safety invariants we need to hold here
|  92   | fn Storage::set_event_emission                | 0, `unsafe { ... }` - I don't grok the safety invariants we need to hold here
| EOF   | | |

src/world/entity.rs
-------------------
Well, this is a bit more complicated than simply doing generational indicies.  Atomics... oh boy.

| Line  | What  | Notes |
| -----:| ----- | ----- |
|  51   | struct Allocator                              | 0, interplay of fields is underdocumented
|       | Allocator::alive                              | Lagging bitset derived from aliveness of `generations`, used for `Join` and not much else.  Lags behind when *\_atomic is used.
|       | Allocator::raised, killed                     | Tracks atomic ops specifically for later merge, otherwise ignored/internal.
|       | Allocator::cache                              | LIFO queue of entity IDs to reuse
|       | Allocator::max_id                             | next entity ID if no entity IDs to reuse
|  90   | fn Allocator::kill_atomic                     | 0, leaves generations alone?  Is this an async kill?
| 114   | fn Allocator::is_alive                        | -1, doesn't this need to check !self.killed.contains(id) too?
| 131   | fn Allocator::entity                          | -1, doesn't this need to check !self.killed.contains(id) too?
| 188   | fn Allocator::merge                           | 0, why not push directly into cache?
| 211   | struct CreateIterAtomic                       | +1
| 223   | struct Entity                                 | +1
| 256   | struct EntitiesRes                            | +1
| 321   | unsafe fn EntitiesRes::open                   | +1? don't fully grok preconds but this seems fine/safe
| 321   | unsafe fn EntitiesRes::get                    | +1? don't fully grok preconds but this seems fine/safe
| 336   | unsafe impl ParJoin for EntitiesRes           | -1, don't fully grok preconds
| 341   | struct EntityResBuilder                       | +1
| 376   | struct Generation                             | +1, should maybe have a struct-level comment that negative = dead though?  Although obvious from fn is_alive...
| 420   | struct ZeroableGeneration                     | +1
| 441   | fn ZeroableGeneration::die                    | 0, is debug_assert! enough?  Seems like it'll go super off the rails in release builds, and other fns like raised panic.  Internal method though...
| 451   | fn ZeroableGeneration::raised                 | 0, explicit checked_sub vs implicit for Generation::raised
| 470   | struct EntityCache                            | 0, needs better docs.  Seems to be a semi-concurrent FILO index list for entity realloc.
| EOF   | | |

src/world/world_ext.rs
----------------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 240   | fn WorldExt::create_entity_unchecked          | -1, does this need to be an `unsafe fn`?  What isn't checked?  Just that you have exclusive world access?
| 281   | fn WorldExt::is_alive                         | 0, sketchy edge cases
| 392   | fn World::is_alive                            | 0, should this really be panicing on a dead generation?  Doesn't that just mean the entity is dead?!?
| EOF   | | |

src/bitset.rs
-------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
|  24   | unsafe fn BitSet\*::open                      | +1, sure?
|  29   | unsafe fn BitSet\*::get                       | +1, sure?
|  35   | unsafe impl ParJoin for BitSet\*              | 0, think this is fine?
| EOF   | | |

src/changeset.rs
----------------
| Line  | What  | Notes |
| -----:| ----- | ----- |
| 61    | fn ChangeSet::add                             | +1, `unsafe { ... }` - documents and abides by safety constraints
| 66    | fn ChangeSet::add                             | +1, `unsafe { ... }` - abides by safety constraints, docs seem a little off
| 77    | fn ChangeSet::clear                           | +1, `unsafe { ... }` - documents and abides by safety constraints
| 115   | unsafe fn &mut ChangeSet::open                | +1, sure?
| 123   | unsafe fn &mut ChangeSet::get                 | -1, bypasses lifetime checks!
| 134   | unsafe fn &ChangeSet::open                    | +1, sure?
| 141   | unsafe fn &ChangeSet::get                     | +1, sure?
| 154   | unsafe fn ChangeSet::open                     | +1, sure?
| 161   | unsafe fn ChangeSet::get                      | +1, sure?
| EOF   | | |

TIL
---
* std::num::NonZeroI32
* std::fmt::Formatter .debug_tuple().field().finish()
* `#[must_use]` makes a lot of sense for builder patterns terminating in `.build()`

[#644]:     https://github.com/slide-rs/specs/issues/644
[#645]:     https://github.com/slide-rs/specs/pull/645
