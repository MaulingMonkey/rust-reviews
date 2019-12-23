---
category:       Gamedev
description:    A low-boilerplate, high performance archetype based ECS. Lots of unsafe, possibly unsound, overflow concerns, etc.
---

# legion

| version | thoroughness | understanding | rating |
| ------- | ------------ | ------------- | ------ |
| [0.1.1](#0.1.1) | medium | low (thanks to use of unsafe) | negative

A low-boilerplate, high performance [archetype] based ECS.

Pros:
* Archetypes
* Low boilerplate
* MIT Licensed

Cons:
* Widespread use of 16-bit values is begging for overflows
* Some O(N) patterns I dislike, fortunately with small C.
* Not MIRI friendly in even the most trivial of examples.
* Difficult-to-vet unsafe code, strongly suspect some unsound.
* Omnipresent low-value logging
* Tons of dependencies (89)

0.1.1
=====

| thoroughness | understanding | rating |
| ------------ | ------------- | ------ |
| medium | low (thanks to use of unsafe) | negative

| File                                          | Rating | Notes |
| --------------------------------------------- | ------ | ----- |
| benches/allocation_stress.rs                  | +1 | No global side effects, no black box use...?
| benches/pos_vel.rs                            | +1 | No global side effects, no black box use...?
| examples/hello_world.rs                       | +1 | |
| [src/borrows.rs](src/borrows.rs)              | +1 | API lock design concerns me... and I suspect this relies on [stable_drop_order] to avoid aliasing violations.
| [src/lib.rs](src/lib.rs)                      | -1 | Difficult to vet unsafe.  Some O(N) and overflow concerns.
| [src/query.rs](src/query.rs)                  | -1 | Difficult to vet unsafe.
| [src/storage.rs](src/storage.rs)              | -1 | Tons of difficult-to-vet UnsafeCell use that leaks some of it's unsafety out to other files.
| tests/query_api.rs                            | +1 | |
| tests/world_api.rs                            | +1 | |
| .cargo_vcs_info.json                          | +1 | |
| .cargo-ok                                     | +1 | |
| .gitignore                                    | +1 | |
| .travis.yml                                   | +1 | caching cargo might be a bad idea IME
| bench.png                                     | +1 | |
| Cargo.toml                                    | +1 | MIT
| Cargo.toml.orig                               | +1 | MIT
| LICENSE                                       | +1 | MIT
| readme.md                                     | +1 | |

| Other     | Rating | Notes |
| --------- | ------ | ----- |
| unsafe    | -1 | Lots of hard to reason about unsafe.
| miri      | -1 | Trivial use chokes up miri.
| fs        | +1 | |
| io        | +1 | |
| docs      | +1 | |
| tests     | +1 | |

src/borrows.rs
--------------
| Line  | What                      | Notes |
| -----:| ------------------------- | ----- |
| 17    | Borrow::aquire_read       | Possible race condition source.  Attempts to increment if >= 0.  Theoretically could livelock if sufficiently contested, shouldn't in practice.
| 33    | Borrow::aquire_write      | Possible race condition source.  Attempts to go from 0 => -1.
| 43    | Drop for Borrow           | +1
| 57    | Borrow*ed*                | Safe from aliasing violations by virtue of [stable_drop_order] ?  Cannot drop `Borrow` before `value` lifetime has ended.
| 92    | PartialEq for Borrowed    | 'b lifetime unused...?
| 98    | Eq for Borrowed           | 'b lifetime unused...?
| 128   | Borrow*ed*Mut             | Safe from aliasing violations by virtue of [stable_drop_order] ?  Cannot drop `Borrow` before `value` lifetime has ended.
| 177   | BorrowedSlice             | Safe from aliasing violations by virtue of [stable_drop_order] ?  Cannot drop `Borrow` before `value` lifetime has ended.
| 220   | BorrowedMutSlice          | Safe from aliasing violations by virtue of [stable_drop_order] ?  Cannot drop `Borrow` before `value` lifetime has ended.
| 269   | BorrowedIter              | Safe from aliasing violations by virtue of [stable_drop_order] ?  Cannot drop `Borrow` before `value` lifetime has ended.
| EOF   | | |

src/lib.rs
----------
| Line  | What                              | Notes |
| -----:| --------------------------------- | ----- |
| 1     | Lib doc comments                  | +1
| 254   | WorldId                           | Only 16-bit world IDs? I could see this overflowing in practice, especially if using the advertized ability to stream stuff in.
| 264   | ArchetypeId                       | Only 16-bit chunk IDs? Also, poor alignment.
| 281   | Entity                            | Standard dual generation index, I approve... although hot entity IDs could overflow in practice?
| 310   | Universe::logger                  | Not entirely sure I'm a fan of this.
| 365   | ComponentIndex & friends          | These Indexes should be used earlier when defining WorldId etc....
| 409   | impl EntityBlock                  | +1
| 431   | EntityBlock::in_range             | Slightly bogus u32 cast - EntityBlock::new should enforce u32 size if you want a u32 len...
| 498   | impl EntityAllocator              | Full of O(N/1024) operations that could be O(1).  Chunking is fine, but this really should abuse the fact that BLOCK_SIZE is constant more instead of looping.
| 517   | EntityAllocator::create_entity    | A better allocation strategy (IMO) would be to always start with the block we last allocated from.  Will have degenerate behavior if we're constantly freeing/allocating from the first block.
| 625   | World::merge doc comment          | Concerning, implies API is unsound
| 638   | World::merge                      | `unsafe { ... }` - I *think* this might be sound thanks to `&mut self` implying we have exclusive access to self.archetypes[?].chunks()[?].entities()
| 719   | World::insert                     | `unsafe { ... }` - I *think* this might be sound thanks to `&mut self` implying we have exclusive access to self.archetypes[?].chunks()[?].entities()
| 720   | World::insert                     | `unsafe { ... }` - I *think* this might be sound thanks to `&mut self` implying we have exclusive access to self.archetypes[?].chunks()[?].entities()
| 805   | World::entity_data_mut            | `unsafe { ... }` - I *think* this might be sound thanks to `&mut self` implying we have exclusive access to self.archetypes[?].chunks()[?].entities()
| 841   | World::prep_archetype             | `unsafe { ... }` - Probably sound - index should be valid - but why not just `.map(|(_, a)| a)` instead of using an index?
| 1016  | EntitySource for IterEntitySource | `unsafe { ... }` - I *think* this might be sound thanks to `&mut Chunk` implying we have exclusive access to chunk.entities()
| EOF   | | |

src/query.rs
------------
| Line  | What                                  | Notes |
| -----:| ------------------------------------- | ----- |
| *     | PhantomData<T>                        | Frequently used when T is already used in the struct, I'm concerned I'm missing something.
| 196   | impl View for (...) validate()        | `unsafe { ... }` - `0 <= i < j < types.len()`, so both calls to get_unchecked should be sound.
| 276   | std::ops::Not for Passthrough         | Oooh, operator overloading abuse.  Probably fine?
| 1065  | impl Iterator for ZipEntities next    | `unsafe { ... }` - Sketchy, assumes data.len() <= entities.len() and that's not a particularly trivial assertion to prove.
| 1088  | ChunkView::entities                   | `unsafe { ... }` - NFI if this is sound or not.
| EOF   | | |

src/storage.rs
--------------
| Line  | What                                      | Notes |
| -----:| ----------------------------------------- | ----- |
| 19    | StorageVec<T>                             | UnsafeCell!  Oh no.
| 39    | unsafe fn StorageVec::data_mut(&self)     | Because this doesn't take `&mut self`, this forces the caller to enforce borrow checking manually.
| 47    | ComponentStorage::remove for StorageVec   | `unsafe { ... }` - I *think* this might be sound thanks to `&mut self`
| 52    | ComponentStorage::len for StorageVec      | `unsafe { ... }` - Unsound?  It's not clear what, if anything, ensures len() isn't being mutated by another borrower.
| 104   | unsafe fn Chunk::entities_unchecked       | Because this doesn't take `&mut self`, this forces the caller to enforce borrow checking manually.
| 126   | unsafe fn Chunk::entity_data_unchecked    | +1?
| 141   | unsafe fn Chunk::entity_data_mut_unchecked| Ditto.
| 157   | Chunk::entity_data                        | `unsafe { ... }` - Unsound?  "Locks" via borrow types *after* constructing `&T`, which is too late.
| 174   | Chunk::entity_data_mut                    | `unsafe { ... }` - Unsound?  "Locks" via borrow types *after* constructing `&mut T`, which is too late.
| 199   | Chunk::shared_data                        | `unsafe { ... }` - NFI if this is sound or not.  Haven't groked why SharedComponentStore is UnsafeCell or what protects it, if anything.
| 212   | Chunk::remove                             | `unsafe { ... }` - I *think* this might be sound thanks to `&mut self`
| EOF   | | |

Miri Example
------------
```rust
use legion::prelude::*;

#[test]
fn test() {
    let universe = Universe::new(None);
    let mut world = universe.create_world();
    world.insert_from((), vec![(1i32,)]);
}
```

```cmd
rustup toolchain install nightly-2019-09-11
rustup component add miri --toolchain=nightly-2019-09-11
cargo +nightly-2019-09-11 miri test
```

```
error[E0080]: Miri evaluation error: trying to reborrow for SharedReadWrite, but parent tag <126591> does not have an appropriate item in the borrow stack
     |
     = note: inside call to `<legion::IterEntitySource<std::vec::IntoIter<(i32,)>, (i32,)> as legion::EntitySource>::write` at C:\Users\Mike\.cargo\registry\src\github.com-1ecc6299db9ec823\legion-0.1.1\src\lib.rs:716:29
     = note: inside call to `legion::World::insert::<(), legion::IterEntitySource<std::vec::IntoIter<(i32,)>, (i32,)>>` at C:\Users\Mike\.cargo\registry\src\github.com-1ecc6299db9ec823\legion-0.1.1\src\lib.rs:689:9
note: inside call to `legion::World::insert_from::<(), std::vec::Vec<(i32,)>>` at src\lib.rs:7:5
    --> src\lib.rs:7:5
     |
7    |     world.insert_from((), vec![(1i32,)]);
     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside call to `test` at src\lib.rs:4:1
    --> src\lib.rs:4:1
     |
4    | / fn test() {
5    | |     let universe = Universe::new(None);
6    | |     let mut world = universe.create_world();
7    | |     world.insert_from((), vec![(1i32,)]);
8    | | }
     | |_^
```

TIL
---
* `TypeId::of::<T>()`



[stable_drop_order]:    https://github.com/rust-lang/rfcs/blob/master/text/1857-stabilize-drop-order.md
[Unity]:                https://unity.com
[archetype]:            https://github.com/SanderMertens/ecs-faq#what-is-an-archetype-based-ecs
