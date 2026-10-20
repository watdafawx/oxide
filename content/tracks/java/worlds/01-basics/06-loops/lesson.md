Anything that handles "all of them" (every line in a file, every item in an order) is a loop. Java has three, and you will use all of them.

```java
int total = 0;
for (int i = 1; i <= 5; i++) {      // counted loop: start; keep going while; step
    total += i;                     // total = total + i
}

int n = 100;
while (n > 1) {                     // loop while the condition holds
    n = n / 2;
}

for (char c : "hello".toCharArray()) {   // enhanced for: each element, no index
    System.out.println(c);
}
```

- Off-by-one is the classic bug: `i < n` stops before `n`, `i <= n` includes it.
- Use enhanced `for` when you do not need the index; use counted `for` when you do.
- `text.toCharArray()` gives the characters; `Character.toLowerCase(c)` normalises case.
- **`indexOf`** finds a character inside text: `"aeiou".indexOf('e')` is `1`, and it is `-1` when the character is not there. So `"aeiou".indexOf(c) >= 0` asks "is `c` a vowel?". A `char` is one character in single quotes.
- `total += i` is shorthand for `total = total + i`.

## Your turn

- Open `src/main/java/oxide/loops/Loops.java`.
- `sumTo(n)`: add `1 + 2 + ... + n`.
- `countVowels(text)`: count `a e i o u`, upper or lower case.
