**Why now:** the bot's whole job is one line of text: `play.example.net: 3/20 online`, plus the server's TPS (ticks per second, ideally 20.00) in the same tidy format. `printf` and `String.format` build those lines from a **template** instead of a pile of `+`.

```java
System.out.printf("%s has %d items%n", "chest", 3);   // chest has 3 items
String tps = String.format(Locale.ROOT, "%.2f", 19.9766);   // "19.98"
```

- `%s` text, `%d` whole number, `%.2f` decimal with 2 digits (it rounds), `%n` newline.
- `printf` prints; `String.format` returns the text so you can reuse it.
- **`Locale.ROOT`** pins the format: without it, a German computer prints `19,98` with a comma and your tests would go red.
- Tests cannot watch the screen, so the code prints to a `PrintStream out` it is **given**. `System.out` is just one such stream. The test hands in a buffer and reads it back.

## Your turn

- Open `src/main/java/oxide/printing/StatusPrinter.java`.
- `printStatus` prints `<server>: <online>/<max> online` plus a newline to `out`, using `printf`.
- `formatTps` returns the TPS with two decimals, using `String.format`.
