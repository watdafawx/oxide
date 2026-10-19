**Why this matters:** by default Java compares objects by **identity**: two `Money` objects holding 12.50 EUR are "different" because they are two objects. Put them in a `HashSet` or call `list.contains(...)` and the answer is "not found". For anything that is a *value* (money, coordinates, ids) you want to compare by content.

Three methods from `Object` do that, and they go together:

```java
public class Point {
    private final int x, y;
    public Point(int x, int y) { this.x = x; this.y = y; }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Point other)) return false;   // also false for null
        return x == other.x && y == other.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);                        // same fields as equals
    }

    @Override
    public String toString() {
        return "(" + x + ", " + y + ")";                  // shown in logs, errors, debugger
    }
}
```

- **The contract:** if `a.equals(b)` then `a.hashCode() == b.hashCode()`. `HashSet` and `HashMap` jump to a bucket by hash first, then use `equals`. Break the contract and equal objects land in different buckets: "not found".
- `instanceof Point other` checks the type and gives you the cast variable in one go.
- `toString()` is what `println`, string concatenation and AssertJ failure messages show. A readable one saves debugging time.

## Your turn

- Open `src/main/java/oxide/equals_hashcode_tostring/Money.java`.
- Write `equals` (same cents and currency), `hashCode` (same fields) and `toString` (`12.50 EUR`).
