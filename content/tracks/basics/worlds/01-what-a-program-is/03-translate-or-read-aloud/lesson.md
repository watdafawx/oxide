Your welcome screen ran the moment you pressed Run. A Rust or Java program takes an extra step before it can run, and knowing why explains many of the messages you will meet later.

**The computer's own language.** The chip that does the work, the **processor**, only understands very simple numbered commands called **machine code**. Nobody wants to write that by hand, so people write in a language they can read and let a program translate it.

**Read it aloud.** Lua uses an **interpreter**: a program that reads your code and carries it out right away, step by step, like someone reading a recipe aloud while cooking. Starting is instant, but each line is only looked at closely when the interpreter reaches it.

**Translate first.** Rust and Java use a **compiler**: a program that translates all of the source code before any of it runs, like translating a whole book before printing it. This step is called **compiling** (or building), and it produces a file the computer can run again and again.

**Two kinds of mistakes.** A compiler reads everything first, so it can refuse a broken program before it ever runs: that is a **compile error**. A mistake that only shows up while the program is running is a **runtime error**. Lua finds most mistakes at run time, so a typo on line 40 surfaces only when line 40 runs.

**Why bother compiling.** The compiler's checks catch whole families of bugs before anyone uses the program, and translated programs run faster. The price is waiting for the build and obeying more rules. Rust is the strictest language in this app; its red messages are those checks doing their job.

**In between.** Java compiles to an in-between code, which a Java program then runs. Whichever way a language runs, the ideas in this track carry over unchanged.

## Your turn

Answer the quiz, then start the next quest.
