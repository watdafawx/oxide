**Why this matters:** programs talk to people through text. A report, a log line, a price tag: all of them need exact spacing and decimals, and `printf` and `String.format` give you that with one **template** instead of a pile of `+`.

```java
System.out.printf("%s has %d items%n", "Ada", 3);   // Ada has 3 items
String price = String.format(Locale.ROOT, "%.2f", 7.5);   // "7.50"
```

- `%s` text, `%d` whole number, `%.2f` decimal with 2 digits (it rounds), `%n` newline.
- `printf` prints; `String.format` returns the text so you can reuse it.
- **`Locale.ROOT`** pins the format: without it, a German computer prints `7,50` with a comma and your tests would go red.
- Tests cannot watch the screen, so the code prints to a `PrintStream out` it is **given**. `System.out` is just one such stream. The test hands in a buffer and reads it back.

## Your turn

- Open `src/main/java/oxide/printing/Printer.java`.
- `printGreeting` prints `Hello, <name>!` plus a newline to `out`, using `printf`.
- `formatPrice` returns the price as `$` plus two decimals, using `String.format`.
