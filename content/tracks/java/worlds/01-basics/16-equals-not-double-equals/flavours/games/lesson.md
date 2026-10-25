The bot should only announce the MOTD when it has really changed. That means comparing the old text with the new one, and `==` is the wrong tool for it.

**Same letters.** Two texts can hold exactly the same letters and still be two separate pieces of text in memory, for example when one was read from the network and the other typed in the code.

**`==` asks the wrong thing.** For texts, `==` asks "are these the very same piece of text?" and can answer `false` for two texts with identical letters. The checker builds exactly that case on purpose.

**`equals` asks the right thing.** `a.equals(b)` asks "do these have the same characters?". That is almost always what you mean:

```java
a.equals(b)               // true when every letter matches
a.equalsIgnoreCase(b)     // true even when only the capitals differ
```

**Ignoring capitals.** `equalsIgnoreCase` does the same but treats `A` and `a` as equal.

**The rule.** Numbers and chars: `==`. Texts: `equals`. When your comparison of text misbehaves, check this first.

**If this is new.** Programming 101 explains the same thing versus an equal-looking one from zero: [Same or alike](/quest/basics/07-how-the-computer-holds-things/03-same-or-alike).

## Your turn

- In `sameMotd`, replace `==` so it compares the characters of the two texts.
- In `sameIgnoringCase`, compare the characters but ignore capitals.
- Save. Both objectives turn green.
