When a Java program misbehaves (slow start, a memory error, "works on my machine"), the cause is usually in how Java runs. One screen of the picture helps a lot.

```
Hello.java --javac--> Hello.class (bytecode) --JVM--> runs on any OS
```

1. **Source** (`.java`): what you write.
2. **`javac`** compiles it to **bytecode** (`.class`). Bytecode is not for one CPU; it is for the JVM.
3. The **JVM** (Java Virtual Machine) loads the classes and runs them. The same `.class` runs on Windows, Linux and macOS: "write once, run anywhere".
4. The **JIT** (just-in-time compiler) watches which methods run a lot and compiles them to real machine code on the fly. That is why Java is slow for the first second and fast afterwards.
5. The **garbage collector** frees objects nothing refers to any more. You never call `free`, but you can still leak memory by keeping references you forgot about.

Gradle runs `javac` and the JVM for you; the tests you see in Oxide are just a JVM run of your compiled code.

Answer the quiz to finish.
