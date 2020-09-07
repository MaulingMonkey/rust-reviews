---
category:       Async
description:    Asyncronous streams, sinks, executors, tasks, I/O, etc.
crev:           positive
---

# futures

Asyncronous streams, sinks, executors, tasks, I/O, etc.

* [docs.rs/futures](https://docs.rs/futures/)

I tend to prefer this over tokio - doesn't try to take over the main
thread/loop, no magic ambient contexts, etc.
