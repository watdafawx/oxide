The bot answers the status command with a little JSON document: several lines, with quote marks everywhere. Escaping every quote and newline by hand is miserable, and Java has a nicer way to write it.

**Many lines, no escapes.** A **text block** starts with three double quotes `"""` and ends with three. Between them, line breaks stay, and quote marks need no backslash.

**How to open it.** The opening `"""` must be the last thing on its line. The text starts on the next line.

**Indentation.** Java removes the indentation up to the closing `"""` (or the least-indented line, whichever is smaller). Extra indentation deeper than that stays, so you can lay JSON out as it should look.

**Fill the slots.** `"...".formatted(args)` is `String.format` as a method on the text: `%s` and `%d` slots work as before.

```java
String note = """
        Hello, %s!
        Welcome back.
        """.formatted("Ada");
```

**The last newline.** When the closing `"""` sits on its own line, the text ends with a `\n`. If it follows the last word directly, there is no final newline.

## Your turn

- In `statusJson`, return the JSON from the comment as a text block.
- Fill the `%s` and `%d` slots with `.formatted(status, playerCount)`. Save and the objective turns green.
