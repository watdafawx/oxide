The boss of this world reads the pack file, one line of text per mod: `sodium,Sodium,fabric,9500,0.5.3`. The update feed has lines too: `sodium 0.5.8 Fixed a crash in chunk loading`. Before any stream can count mods, each line has to be cut into its parts.

**split.** `line.split(",")` cuts the text at every comma and hands back the pieces as a `String[]`, an array like the ones in the Basics world. `"a,b,c".split(",")` is `{"a", "b", "c"}`. The separator itself is dropped.

**Into a list.** `List.of(...)` takes any number of values, and an array counts as "any number", so `List.of(line.split(","))` turns the pieces into a read-only list.

**Keep the rest whole.** The changelog `Fixed a crash in chunk loading` has spaces of its own, so a plain split at spaces cuts it into six words. `line.split(" ", 3)` cuts into **at most 3** pieces: after the second cut it stops, and the third piece is the whole rest of the line, spaces included.

```java
String[] parts = "ada 2024-05-01 Fixed the login page".split(" ", 3);
parts[0];   // "ada"
parts[1];   // "2024-05-01"
parts[2];   // "Fixed the login page"
```

**For the curious.** The separator is really a **regular expression**, a pattern language: `split(",\\s*")` cuts at a comma plus any spaces after it, and a dot has to be written `"\\."`, which matters when you split a version like `0.5.8`. Plain letters, spaces and commas mean themselves.

## Your turn

- In `loaders`, cut the line at its commas and return the loaders as a list.
- In `changelogOf`, cut the update line at spaces into at most 3 parts and return the third one, the changelog.
