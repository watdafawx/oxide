The status line says ONLINE or OFFLINE, and "1 server" or "2 servers". Each is a tiny choice between two pieces of text. A full `if` is a lot of ceremony for that.

**An if with a value.** `condition ? a : b` is a choice that gives a value back. If the condition is true you get `a`, otherwise `b`. It is called the **ternary** operator.

**Read it in words.** "If crowded, then busy, otherwise quiet." The `?` is the "then" and the `:` is the "otherwise".

**One line.** The whole choice fits on the right side of a variable, or after a `return`:

```java
String size = crowded ? "busy" : "quiet";
```

**Keep it small.** A ternary is good for one little choice between two values. For longer logic, with more paths or more lines, use the full `if` from the last quest.

**Whole-number test.** To ask "is it exactly 1?" use `count == 1`: two equals signs, as before.

## Your turn

- In `statusWord`, return `ONLINE` when `online` is true, otherwise `OFFLINE`.
- In `serverWord`, return `server` when `count` is exactly 1, otherwise `servers`.
- Save. Both objectives turn green.
