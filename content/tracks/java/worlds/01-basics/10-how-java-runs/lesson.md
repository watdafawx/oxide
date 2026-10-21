A program can run for weeks. It helps to know what Java is doing while your code runs, because two helpers work in the background and explain most of the odd behaviour you will see.

**Two helpers.** You already know the JVM runs your bytecode. While it does, it has two jobs of its own: making hot code faster, and cleaning up memory.

**The JIT.** The **JIT** (just-in-time compiler) watches which methods run a lot and compiles them to real machine code on the fly. Code that runs a million times ends up as fast as hand-tuned code.

**The slow start.** The JIT needs time to watch. That is why a Java program feels slow for the first second and fast afterwards. It is normal, not a bug.

**The garbage collector.** Every object your program makes takes memory. The **garbage collector** frees any object that nothing refers to any more. You never call `free`, as you would in some other languages.

**Still a leak.** The collector only frees what nothing points at. If you keep a reference to something you forgot about, such as an old list of every player ever seen, that memory stays used. That is a **memory leak**.

## Your turn

- Answer the three questions below.
