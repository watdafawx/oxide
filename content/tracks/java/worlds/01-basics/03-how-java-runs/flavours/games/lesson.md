**Why now:** your server status bot will run for weeks next to a Minecraft server, on whatever machine you host it on. Knowing what the JVM does with your code explains the slow first minute, the memory the bot uses, and why the same jar runs on your PC and on a Linux box.

```
StatusBot.java --javac--> StatusBot.class (bytecode) --JVM--> runs anywhere
```

1. **Source** (`.java`): the `StatusLine` class you write.
2. **`javac`** compiles it to **bytecode** (`.class`), made for the JVM, not for one CPU.
3. The **JVM** loads the classes and runs them. The same `.class` works on Windows, Linux and macOS.
4. The **JIT** compiles methods that run often (like building the status line every ten seconds) to real machine code while the program runs. Slow first minute, fast after.
5. The **garbage collector** frees objects nothing refers to. Every old `StatusLine` goes away on its own; you only leak memory by holding on to things (say, a list of every status ever built).

Gradle runs `javac` and the JVM for you. Oxide's tests are a JVM run of your compiled quest.

Answer the quiz to finish.
