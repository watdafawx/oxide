This world builds a mod folder hasher: point it at `mods/` and it computes a short fingerprint, a **hash**, for every jar, so a launcher can tell which mods changed since the last sync. Hashing 200 jars one after another is slow, and most of the PC sits idle while it happens. The fix is to hash several jars at once, and that starts with one word: thread.

**One line at a time.** Every program you have written so far runs one line, then the next, then the next. That single path through the code is a **thread**, which means one sequence of steps that the computer follows in order. A running program, a **process**, starts with one thread: the **main thread**, the one that runs `main` or your test.

**Several cooks.** A process may start more threads. Picture one kitchen with several cooks: they share the same fridge and the same pans (the program's memory), but each cook follows their own recipe at their own pace. One thread hashes `sodium.jar` while another hashes `iris.jar`.

**Cores do the work.** A **core** is the part of the processor that actually carries out steps. A PC with 8 cores can run 8 threads at the very same moment. With more threads than cores, the operating system takes turns: it runs a thread for a few thousandths of a second, pauses it, and runs another. That switching is so fast that all of them seem to run at once, which is how a game keeps drawing frames while it loads chunks.

**The order is not fixed.** You do not decide when the system pauses a thread or wakes it up. So two threads that each print a line can print in either order, and the order can change from one run to the next:

```text
run 1:              run 2:
hashing sodium.jar  hashing iris.jar
hashing iris.jar    hashing sodium.jar
```

**Why it matters.** Code that is correct on one thread can break on two, because "first this, then that" no longer holds between threads. Rust's compiler checks much of this for you, and this world shows how: you start threads, hand them jars, get hashes back, and share a progress counter between them without losing a single update.

**When threads pay off.** A thread costs something to start, roughly the time to hash a small jar. Splitting is worth it when there is a lot of independent work, like 200 jars that do not depend on each other. Adding two numbers on a second thread is only slower.

## Your turn

Answer the questions, then start the next quest.
