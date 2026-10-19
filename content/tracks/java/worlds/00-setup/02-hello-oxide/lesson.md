**Why this matters:** before you build anything bigger, you need proof the loop works: edit, save, watch a check turn green. Every real program is built on that loop, with tests as the checks.

A Java method that gives back text looks like this:

```java
public static String farewell() {
    return "Bye, Oxide!";
}
```

- `String` is the **return type**: text.
- `return` hands the value back to whoever called the method.
- Text literals use **double quotes**, and every statement ends with `;`.

The test calls `Hello.greeting()` and expects exactly `Hello, Oxide!`. Capital letters, comma and `!` all count.

The first save downloads Gradle and the test libraries (about 90 seconds, once). Later saves take a few seconds.

## Your turn

- Open `src/main/java/oxide/hello_oxide/Hello.java`.
- Replace `return null;` so `greeting()` returns `"Hello, Oxide!"`.
- Save. The objective turns green.
