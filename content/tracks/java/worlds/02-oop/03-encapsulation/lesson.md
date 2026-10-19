**Why this matters:** a class is a promise: "use my methods and I keep my data valid". The promise breaks the moment outside code can reach in and change a field directly.

**Encapsulation** = fields `private`, access only through methods. Then the class can check every change:

```java
public class Wallet {
    private int coins;                    // nobody outside can write wallet.coins = -50

    public int coins() { return coins; }

    public void spend(int amount) {
        if (amount > coins) throw new IllegalArgumentException("not enough coins");
        coins -= amount;
    }
}
```

But a getter can still leak. Numbers and strings are safe (copied or immutable). A **mutable** object is not:

```java
private final List<String> tags = new ArrayList<>();

public List<String> tags() { return tags; }            // leak: caller gets THE list
public List<String> tags() { return List.copyOf(tags); } // safe: caller gets a copy
```

With the leak, `obj.tags().clear()` empties the list inside your object and you never ran a line of your own code.

Rule of thumb: private fields, small public methods, return copies (or unmodifiable views) of mutable data.
