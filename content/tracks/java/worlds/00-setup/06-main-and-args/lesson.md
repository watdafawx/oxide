Every program you run from a terminal starts in the same place, and anything you type after its name arrives as a list of text. Command-line tools, servers and bots all read their settings this way.

**Where programs start.** Java starts every program in a method called `main`. It is already written in the starter: it calls `banner` and prints what comes back with `System.out.println(...)`, which means "print this and start a new line".

**Words from the terminal.** When you run `java Banner Ada`, the word `Ada` is handed to `main` inside `String[] args`. The `[]` means a **list** of texts, one for every word typed after the program name.

**Counting starts at zero.** The first word is `args[0]`, the second is `args[1]`. Programmers count from 0, so the number in the brackets is how many words to skip.

**Why a second method.** `main` is hard to check, because it prints to the screen. So the real work sits in `banner(args)`, which the checker can call, and `main` just passes `args` along:

```java
public static String banner(String[] args) {
    return "Hello, " + args[0] + "!";
}
```

**One careful thing.** If nobody typed a word, `args[0]` crashes the program: there is no first word to take. The checker always hands you at least one word here. Handling "no words" comes later, with `if`.

## Your turn

- In `banner`, build the text from `args[0]`.
- In `pairBanner`, use both `args[0]` and `args[1]`: "Hello, Ada and Bo!".
- Save. Both objectives turn green.
