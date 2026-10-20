Text is everywhere: names, messages, file paths, JSON. In Java a `String` is an **immutable** object: methods never change it, they return a *new* string.

```java
String s = "Java";
s.toUpperCase();              // returns "JAVA" ... and s is still "Java"
s = s.toUpperCase();          // keep the result by assigning it
"MiXeD".toLowerCase();        // "mixed" (the lower-case twin)

s.length();                   // 4
s.charAt(0);                  // 'J'
s.substring(1, 3);            // "av"   (start included, end excluded)
"a b".replace(" ", "");       // "ab"
s.equals("Java");               // true: same characters
new StringBuilder("abc").reverse().toString();   // "cba"
```

- `==` on objects asks "is this the *same object*?". Two strings with the same characters can still be different objects, so **compare text with `equals`** (`equalsIgnoreCase` for case-insensitive).
- Building a string in a loop with `+` makes a new object each time; use `StringBuilder` (next quests).

## Your turn

- Open `src/main/java/oxide/strings/Words.java`.
- `reverse(text)`: the characters backwards.
- `isPalindrome(text)`: reads the same both ways, ignoring case and spaces ("Never odd or even").
- `sameText(a, b)`: true when both have the same characters. The starter uses `==`, which is the bug.
