A palindrome reads the same backwards, like "Never odd or even". To find out whether a text is one, the program first needs to flip text round.

**Text cannot flip itself.** A `String` has no `reverse()`. A `StringBuilder` is a text you can work on, and it does.

**Wrap, reverse, unwrap.** Learn it as one recipe: `new StringBuilder(text).reverse().toString()` wraps the text in a builder, flips it, and turns it back into a normal `String`.

```java
String back = new StringBuilder("abc").reverse().toString();   // "cba"
String tight = "a b c".replace(" ", "");                       // "abc"
```

**Cut out spaces.** `text.replace(" ", "")` hands back a copy with every space replaced by nothing. The second line above shows it.

**Clean, then compare.** A palindrome ignores capitals and spaces. So tidy the text first, with `replace` and `toLowerCase()`, then check whether it `equals` its own flipped copy.

**Use your own method.** `isPalindrome` can call `reverse` that you wrote in the same file. Methods are there to be reused.

## Your turn

- In `reverse`, return the characters of the text in the opposite order, using the recipe.
- In `isPalindrome`, clean the text (no spaces, lower case) and compare it with its own `reverse`.
- Save. Both objectives turn green.
