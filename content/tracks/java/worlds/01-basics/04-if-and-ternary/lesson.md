Every real program decides things: is the input valid, is the user allowed, is the list empty. `if` is how Java chooses a path.

```java
if (n < 0) {
    return "negative";
} else if (n == 0) {
    return "zero";
} else {
    return "positive";
}

String kind = n < 0 ? "negative" : "not negative";   // ternary: an if that has a value
```

- Conditions are `boolean`: `<`, `<=`, `==`, `!=`, and `&&` (and), `||` (or), `!` (not).
- `&&` and `||` **short-circuit**: in `list != null && list.length > 0` the right side never runs when the left is false. That is how you avoid a `NullPointerException`.
- Use `==` for numbers, never for text (a later quest explains why).
- Ternary `cond ? a : b` is good for one small choice; use `if` for anything longer.

## Your turn

- Open `src/main/java/oxide/if_and_ternary/Branching.java`.
- `clamp(v, lo, hi)`: return `v` forced into `lo..hi`.
- `signOf(n)`: return `-1`, `0` or `1`.
