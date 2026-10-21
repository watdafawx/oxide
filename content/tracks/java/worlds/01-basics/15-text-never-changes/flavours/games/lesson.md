The MOTD arrives from the server with stray spaces and odd capitals. Before the bot posts it, it should be tidied up. Text has ready-made tools for that, and one rule you have to know.

**Ask the text.** A piece of text knows how to do things. You ask with a dot: `text.trim()` is "text, please trim yourself".

**Four to know.** `trim()` cuts spaces off both ends. `toUpperCase()` and `toLowerCase()` change capitals. `length()` is the number of characters. The brackets are always there, even when empty.

**Text never changes.** A `String` is **immutable**: no method can change it. Every method hands back a **new** text and leaves the original alone.

**So keep the result.** A call whose result nobody keeps does nothing useful:

```java
String s = "  Java ";
String clean = s.trim();             // "Java"
s.toUpperCase();                     // makes "  JAVA ", but nobody keeps it
String loud = clean.toUpperCase();   // "JAVA"
int size = loud.length();            // 4
```

**Chain them.** The result is a text too, so you can ask it straight away: `s.trim().toUpperCase()` trims first, then upper-cases.

## Your turn

- In `tidy`, return the text with the spaces at both ends cut off.
- In `shout`, return the MOTD in capital letters.
- In `motdLength`, return how many characters the MOTD has. Save and all three turn green.
