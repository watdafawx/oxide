The launcher shows "37 of 200 jars hashed". The simplest idea is one shared number that every worker adds 1 to after each jar. On one thread that is fine. On several it silently loses jars, and seeing why is the key to everything else in this world.

**+= is three steps.** `done += 1` looks like one action, but the processor does three: read `done` from memory, add 1, write the result back. Another thread can run between any two of those steps.

**A lost update.** Say `done` is 5 and two workers finish a jar at the same moment:

```text
worker A: read 5
worker B: read 5
worker A: write 6
worker B: write 6      two jars finished, the count went up by one
```

**The word for it.** This is a **data race**: two threads use the same memory at the same time, at least one of them writes, and nothing makes them take turns. Think of two inserters grabbing from the same chest slot in the same tick. A lost count is the mild outcome. If one thread pushes onto a `Vec` while another reads it, the push may move the items to a bigger heap area and free the old one, and the reader is left pointing at freed memory: garbage, or a crash. The worst part is timing: the bug may show up once in a thousand launches.

**Rust's promise.** In C or C++ the compiler accepts all of this. In Rust, code with a data race **does not compile**. The rest of this world shows the tools that make sharing legal: handing values over (channels, which you already used), locks, and atomic numbers.

**Data race vs race condition.** Rust stops data races, not every timing bug. A **race condition** is any bug where the result depends on which thread gets there first: two workers both check "is this jar hashed yet?", both see no, and both hash it. Or two threads each wait for the other forever, a **deadlock**, which you will meet with locks. These are bugs in your logic, and the compiler cannot see them; your design has to.

## Your turn

Answer the questions, then start the next quest.
