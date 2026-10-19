**Why now:** the status bot has to prove it is alive before it can check anything. Its first feature is one chat line: "Status bot online!". It is also your first green tick, which proves the edit, save, green loop works.

A Java method that gives back text looks like this:

```java
public static String farewell() {
    return "Bot going offline.";
}
```

- `String` is the **return type**: text.
- `return` hands the value back to whoever called the method.
- Text literals use **double quotes**, and every statement ends with `;`.

The test calls `StatusBot.greeting()` and expects exactly `Status bot online!`. Capital letters and `!` count.

The first save downloads Gradle and the test libraries (about 90 seconds, once). Later saves take a few seconds.

## Your turn

- Open `src/main/java/oxide/hello_oxide/StatusBot.java`.
- Replace `return null;` so `greeting()` returns `"Status bot online!"`.
- Save. The objective turns green.
