You are going to build a bot that asks a Minecraft server who is online. Your server already runs on Java 21, and so will the bot, so meet its toolbox first.

**The kit.** The **JDK** (Java Development Kit) is everything you need to write and build Java. Oxide uses **JDK 21** and finds it on its own, so there is nothing for you to install.

**The translator.** You write code in `.java` files. A tool called `javac`, the **compiler**, translates a file into **bytecode**: a `.class` file made for Java's own machine, not for your CPU.

**The runner.** Java's own machine is the **JVM** (Java Virtual Machine). The `java` tool starts it and it runs the bytecode. When your server's start script says `java -jar server.jar`, that is this tool:

```bash
javac StatusBot.java   # StatusBot.java -> StatusBot.class
java StatusBot         # the JVM runs StatusBot.class
```

**Skip the typing.** You will almost never type those two lines. A build tool called **Gradle** runs them for you, and every quest has a `gradlew` file (the **wrapper**) that fetches the exact Gradle version the project needs.

**The slow first time.** The first build downloads Gradle and the test libraries, about 140 MB. That takes roughly 90 seconds, once. After that a run takes a few seconds.

## Your turn

Answer the three questions below.
