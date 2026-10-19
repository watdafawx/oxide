**Why this matters:** every Java program you will ever ship, from a bank backend to a Discord bot, goes through the same three tools. Know them once and error messages stop being scary.

- **JDK** (Java Development Kit): everything you need to *write and build* Java. Oxide uses **JDK 21**.
- **`javac`**: the compiler. It turns `Hello.java` into `Hello.class`, which is **bytecode**, not machine code.
- **`java`**: starts the **JVM**, which runs bytecode. A **JRE** is just the run-only half of a JDK; you want the full JDK.

```
javac Hello.java     # Hello.java  -> Hello.class
java Hello           # the JVM runs Hello.class
```

You will almost never type those two lines. Real projects use a build tool: **Gradle**. Each quest folder has a `gradlew` file (the *wrapper*) that downloads the one exact Gradle version the project needs, then builds and runs the tests for you.

**Heads up:** Oxide finds JDK 21 on its own. The very first build downloads Gradle (about 130 MB) and the test libraries JUnit and AssertJ (about 8 MB): roughly **90 seconds, once**. After that a run takes a few seconds.

## Your turn

Answer the three questions below.
