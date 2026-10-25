The status bot has to prove it is alive before it can check anything. Its first feature is one chat line, and getting it right is also your first green tick: edit, save, watch a check turn green.

**Where you start.** Oxide gives you a method called `greeting` and a checker that calls it. The checker lives in `StatusBotTest.java`: that file is what the checker reads, and you never change it. It wants the exact text `Status bot online!`, and your method does not return that yet.

**Read the red.** Save once without changing anything. The objective turns red and the checker explains why:

```text
expected: "Status bot online!"
 but was: null
```

**Two lines, one meaning.** `expected` is what the checker wants. `but was` is what the bot gave back. `null` is Java's word for "nothing yet". Red is not a failure, it is a message telling you what to fix.

**The shape of a method.** `public static String greeting()` is the shape every method has for now: copy it as it is and do not worry about the words yet. Two parts matter today. `String` means the method hands back text, and `greeting` is the name the checker calls.

**Hand it back.** `return` hands a value back to whoever called the method. Text goes in double quotes, and a line ends with `;`:

```java
public static String greeting() {
    return "Status bot online!";
}
```

**Exact means exact.** The checker compares letter by letter, capitals and punctuation included. One wrong character and `but was` shows you which. The first save also downloads Gradle, about 90 seconds, once.

**If this is new.** Programming 101 explains tests and functions from zero: [What a test is](/quest/basics/01-what-a-program-is/06-what-a-test-is) and [Define and call](/quest/basics/05-functions/01-define-and-call).

## Your turn

- Save once without changing anything and read the red message.
- In `greeting`, replace `null` with the text `"Status bot online!"`.
- Save again. The objective turns green.
