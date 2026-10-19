**Why now:** you are going to build a bot that asks a Minecraft server who is online. Your server already runs on Java 21, and so will the bot, so meet the three tools first.

- **JDK** (Java Development Kit): everything you need to *write and build* Java. Oxide uses **JDK 21**, the same version Minecraft 1.21 servers run on.
- **`javac`**: the compiler. It turns `StatusBot.java` into `StatusBot.class`, **bytecode** that any JVM can run.
- **`java`**: starts the **JVM**, which runs bytecode. When your server's start script says `java -jar server.jar`, that is this tool. A **JRE** is just the run-only half; to build a bot you want the full JDK.

```
javac StatusBot.java   # StatusBot.java -> StatusBot.class
java StatusBot         # the JVM runs it
```

You will almost never type those lines. Real projects use a build tool: **Gradle**. Every quest folder has a `gradlew` file (the *wrapper*) that fetches the one exact Gradle version the project needs, then builds and tests for you.

**Heads up:** Oxide finds JDK 21 on its own. The first build downloads Gradle (about 130 MB) and the test libraries JUnit and AssertJ (about 8 MB): roughly **90 seconds, once**. After that a run takes a few seconds.

## Your turn

Answer the three questions below.
