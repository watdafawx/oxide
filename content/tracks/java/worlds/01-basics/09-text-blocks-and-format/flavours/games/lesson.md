**Why now:** the status bot has two outputs besides the one-line status: a **table** of all your servers for `!servers`, and a small **JSON** body for a web hook. Both are "build text from values", and this is the quick-win quest before the arrays.

```java
StringBuilder sb = new StringBuilder();
sb.append("survival\n").append("creative\n");   // fast: no new String per append
String all = sb.toString();

String row = String.format("%-8s|%3d\n", "survival", 12);   // "survival| 12\n"

String json = """
        {
          "server": "%s",
          "players": %d
        }
        """.formatted("survival", 12);
```

- `%s` text, `%d` whole number, `%.2f` two decimals, `%-8s` left-aligned in 8 columns, `%3d` right-aligned in 3.
- A **text block** starts with `"""` + newline and keeps line breaks; quotes inside need no escape. Incidental indentation is removed up to the closing `"""` (or the least-indented line), but *relative* indentation stays.
- A closing `"""` on its own line makes the string end with a newline (`\n`).
- Write a newline as `\n` here, not `%n`. `%n` is the platform's newline (`\r\n` on Windows), which looks identical in a failure but is not the `\n` this report format uses.
- `"...".formatted(args)` is `String.format` as a method.

## Your turn

- Open `src/main/java/oxide/text_blocks_and_format/StatusReport.java`.
- `table(servers, players)`: one row per server, `"%-8s|%3d"` plus a newline, built with a `StringBuilder`.
- `statusJson(status, playerCount)`: return the JSON from the comment as a text block, filled with `formatted`.
