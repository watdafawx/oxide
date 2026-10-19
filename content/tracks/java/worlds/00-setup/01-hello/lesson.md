Every Java quest is a small Gradle project: you edit what is under `src/main/java`, the tests in `src/test/java` say what "done" means.

**Why now:** before anything bigger, you need one green tick, so you know the loop works: edit, save, watch the objective turn green.

`Hello.greets` should turn a name into a greeting. Java joins strings with `+`:

```java
String who = "Oxide";
String line = "Hi, " + who + "!";   // Hi, Oxide!
```

The test calls `Hello.greets("Ada")` and expects exactly `Hello, Ada!`. The first run downloads Gradle and the test libraries, so it takes a while; later runs are fast.

## Your turn

- Open `src/main/java/oxide/hello/Hello.java`.
- Replace `return null;` so `greets` returns `"Hello, " + name + "!"`.
- Save. The objective turns green.
