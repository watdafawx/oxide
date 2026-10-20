**Why now:** your bot polls the server every ten seconds and only posts to Discord when the **MOTD changed**. Each poll builds a fresh string, so comparing with `==` would say "changed!" every time and spam the channel. Strings need `equals`. While you are in there, the bot also gets an April-Fools MOTD flipper and a palindrome easter egg.

In Java a `String` is **immutable**: methods never change it, they return a *new* string.

```java
String motd = "Welcome";
motd.toUpperCase();           // returns "WELCOME" ... and motd is still "Welcome"
motd = motd.toUpperCase();    // keep the result by assigning it
"MiXeD".toLowerCase();       // "mixed" (the lower-case twin)

motd.length();                // 7
motd.charAt(0);               // 'W'
motd.substring(1, 3);         // "el"   (start included, end excluded)
"a b".replace(" ", "");       // "ab"
motd.equals("Welcome");       // true: same characters
new StringBuilder("abc").reverse().toString();   // "cba"
```

- `==` on objects asks "is this the *same object*?". Two MOTDs with identical text can be different objects, so **compare text with `equals`**.

## Your turn

- Open `src/main/java/oxide/strings/Motd.java`.
- `flip(motd)`: the characters backwards.
- `isPalindrome(motd)`: reads the same both ways, ignoring case and spaces ("Was it a car or a cat I saw").
- `sameMotd(a, b)`: true when both have the same characters. The starter uses `==`, which is the bug.
