Real programs print reports, build JSON, write log lines. Three tools cover almost all of it.

```java
StringBuilder sb = new StringBuilder();
sb.append("one\n").append("two\n");        // fast: no new String per append
String all = sb.toString();

String row = String.format("%-6s|%4d\n", "Ada", 95);   // "Ada   |  95\n"

String json = """
        {
          "name": "%s",
          "age": %d
        }
        """.formatted("Ada", 36);
```

- `%s` text, `%d` whole number, `%.2f` two decimals, `%-6s` left-aligned in 6 columns, `%4d` right-aligned in 4.
- A **text block** starts with `"""` + newline and keeps line breaks; quotes inside need no escape. The incidental indentation is removed up to the closing `"""` (or the least-indented line), but *relative* indentation stays.
- A closing `"""` on its own line makes the string end with `\n`.
- Write a newline as `\n` here, not `%n`. `%n` is the platform's newline (`\r\n` on Windows), which looks identical in a failure but is not the `\n` this report format uses.
- `"...".formatted(args)` is `String.format` as a method.

## Your turn

- Open `src/main/java/oxide/text_blocks_and_format/Report.java`.
- `table(names, scores)`: one row per name, `"%-6s|%4d"` plus `\n`, built with a `StringBuilder`.
- `jsonBlock(status, count)`: return the JSON from the comment as a text block, filled with `formatted`.
