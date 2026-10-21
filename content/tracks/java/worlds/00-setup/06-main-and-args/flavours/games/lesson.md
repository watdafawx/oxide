The bot is not a test: you run it from a terminal and tell it which server to check. The words you type after the program name arrive in the program as a list of text.

**Where programs start.** Java starts every program in a method called `main`. It is already written in the starter: it calls `header` and prints what comes back with `System.out.println(...)`, which means "print this and start a new line".

**Words from the terminal.** When you run `java StatusCommand play.example.net`, the word `play.example.net` is handed to `main` inside `String[] args`. The `[]` means a **list** of texts, one for every word typed after the program name.

**Counting starts at zero.** The first word is `args[0]`, the second is `args[1]`. Programmers count from 0, so the number in the brackets is how many words to skip.

**Why a second method.** `main` is hard to check, because it prints to the screen. So the real work sits in `header(args)`, which the checker can call, and `main` just passes `args` along:

```java
public static String header(String[] args) {
    return "Status of " + args[0];
}
```

**One careful thing.** If nobody typed a word, `args[0]` crashes the program: there is no first word to take. The checker always hands you at least one word here. Handling "no words" comes later, with `if`.

## Your turn

- In `header`, build the text from `args[0]`.
- In `compareHeader`, use both `args[0]` and `args[1]`: "Status of play.example.net and mc.example.org".
- Save. Both objectives turn green.
