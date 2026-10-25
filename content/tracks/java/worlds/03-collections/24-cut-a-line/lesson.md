The boss of this world reads a server log, one line of text per event: `10:15:30 ERROR db Connection refused`. Before any stream can count those lines, each one has to be cut into its parts: time, level, source and message.

**split.** `line.split(" ")` cuts the text at every space and hands back the pieces as a `String[]`, an array like the ones in the Basics world. `"a b c".split(" ")` is `{"a", "b", "c"}`. The separator itself is dropped.

**Into a list.** `List.of(...)` takes any number of values, and an array counts as "any number", so `List.of(text.split(" "))` turns the pieces into a read-only list.

**Keep the rest whole.** The message `Connection refused` has a space of its own, so a plain split gives five pieces and cuts the message in two. `line.split(" ", 4)` cuts into **at most 4** pieces: after the third cut it stops, and the fourth piece is the whole rest of the line, spaces included.

```java
String[] parts = "ada 2024-05-01 Fixed the login page".split(" ", 3);
parts[0];   // "ada"
parts[1];   // "2024-05-01"
parts[2];   // "Fixed the login page"
```

**For the curious.** The separator is really a **regular expression**, a pattern language: `split(",\\s*")` cuts at a comma plus any spaces after it, and a dot has to be written `"\\."`. Plain letters, spaces and commas mean themselves.

## Your turn

- In `words`, cut the sentence at its spaces and return the words as a list.
- In `messageOf`, cut the log line into at most 4 parts and return the fourth one, the message.
