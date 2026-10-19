**Why now:** your `Inventory` must never hold 70 items in a 64-stack, or negative durability. The only way to promise that is to make sure nobody outside can write the fields directly.

**Encapsulation** = fields `private`, access only through methods. Then the class can check every change:

```java
public class Furnace {
    private int fuel;                     // nobody outside can write furnace.fuel = -50

    public int fuel() { return fuel; }

    public void burn(int ticks) {
        if (ticks > fuel) throw new IllegalArgumentException("out of fuel");
        fuel -= ticks;
    }
}
```

But a getter can still leak. Numbers and strings are safe (copied or immutable). A **mutable** object is not:

```java
private final List<ItemStack> slots = new ArrayList<>();

public List<ItemStack> slots() { return slots; }              // leak: caller gets THE list
public List<ItemStack> slots() { return List.copyOf(slots); } // safe: caller gets a copy
```

With the leak, `inv.slots().clear()` empties the inventory without a single line of your own code running (and a mod that does that by accident is a dupe or a wipe bug).

Rule of thumb: private fields, small public methods, return copies (or unmodifiable views) of mutable data.
