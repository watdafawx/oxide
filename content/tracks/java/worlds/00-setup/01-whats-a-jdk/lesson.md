You are about to write Java, and Java comes with its own toolbox. Three names from it show up in almost every error message, so it pays to meet them once.

**The kit.** The **JDK** (Java Development Kit) is everything you need to write and build Java. Oxide uses **JDK 21** and finds it on its own, so there is nothing for you to install.

**The translator.** You write code in `.java` files. A tool called `javac`, the **compiler**, translates a file into **bytecode**: a `.class` file made for Java's own machine, not for your CPU.

**The runner.** Java's own machine is the **JVM** (Java Virtual Machine). The `java` tool starts it and it runs the bytecode:

```bash
javac Hello.java   # Hello.java -> Hello.class
java Hello         # the JVM runs Hello.class
```

**Skip the typing.** You will almost never type those two lines. A build tool called **Gradle** runs them for you, and every quest has a `gradlew` file (the **wrapper**) that fetches the exact Gradle version the project needs.

**The slow first time.** The first build downloads Gradle and the test libraries, about 140 MB. That takes roughly 90 seconds, once. After that a run takes a few seconds.

**If you are new to coding.** [Programming 101](/quest/basics/01-what-a-program-is/01-instructions-in-order) teaches the ideas every language shares, once and in plain words: variables, choices, loops, functions, lists. It is optional and uses Lua; skip it if you have programmed before.

## Your turn

Answer the three questions below.
