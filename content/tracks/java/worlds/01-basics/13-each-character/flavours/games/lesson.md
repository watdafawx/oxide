A MOTD can carry colour codes, and every one starts with the section sign `§`. To know how many it has, the bot has to look at the text one character at a time.

**One character.** A **`char`** holds a single character, written in single quotes: `'a'`, `'7'`, `' '`. Text, a `String`, is a row of them.

**Text to characters.** `text.toCharArray()` hands back every character of the text, in order, as an array like the ones `args` was.

**For each.** The enhanced `for` visits them one by one: `for (char c : text.toCharArray())` reads "for each character `c` in the text". No counter variable, no `i`.

```java
int length = 0;
for (char c : "hello".toCharArray()) {
    length++;
}   // length is 5
```

**Compare a char.** `c == 'a'` asks whether the character is the letter a. `==` works for single characters just as it does for numbers.

**Only some of them.** Put that comparison in an `if` inside the loop, and the block only runs for the characters you care about. A counter outside the loop adds them up.

## Your turn

- In `countColorCodes`, loop over the characters of `motd` and count the ones equal to `'§'`.
- Save. The objective turns green.
