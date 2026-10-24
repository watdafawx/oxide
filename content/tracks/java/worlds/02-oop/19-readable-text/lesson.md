A test fails and says `expected: 12.50 EUR but was: Money@1b2c3d`. That second part is useless: it is the class name and a number, all `Object` knows how to say about your object. The third method from `Object` fixes it.

**What Object says.** `toString()` turns an object into text. The version every class gets from `Object` gives the class name, `@`, and a number. Override it, and your object describes itself.

**Used everywhere.** Java calls `toString()` for you whenever an object meets text: `"Total: " + price`, `System.out.println(price)`, and the checker's red messages. Write it once, and every log line and failed test reads clearly.

**Two-digit cents.** 5 cents must print as `0.05`, not `0.5`. In `String.format`, **`%02d`** is a `%d` slot that pads with zeros to two digits: 5 becomes `05`, 50 stays `50`.

**In code.** Here a clock time with padded minutes:

```java
@Override
public String toString() {
    return String.format("%d:%02d", hours, minutes);   // 9:05
}
```

## Your turn

- In `toString`, hand back whole euros, a dot, the cents as two digits, a space, then the currency.
- Save. Both objectives turn green.
