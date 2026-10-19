**Why now:** arrows, lava and explosions should hurt zombies, armor stands and boats alike, and the damage code should not need to know which one it hit. A **Damageable** interface is the promise: "anything that is Damageable has a name, health, and takes damage".

```java
public interface Burnable {
    String name();
    int fuelTicks();

    default String label() {                    // has a body: implementations inherit it
        return name() + " burns for " + fuelTicks() + " ticks";
    }
}

public class Coal implements Burnable {
    public String name() { return "Coal"; }
    public int fuelTicks() { return 1600; }
}

Burnable b = new Coal();
b.label();         // "Coal burns for 1600 ticks"
```

- `interface` lists methods without bodies. Classes `implements` it and fill every one in.
- A **default method** has a body, so you write shared behaviour once. It can call the interface's other methods.
- A variable can have the interface as its type: `Burnable b = new Coal();`.

## Your turn

- `Damageable.java` has `name()`, `health()`, `damage(int)` and an empty `status()`. Write the default body of `status()`.
- Fill in `Zombie` and `ArmorStand` (name and `damage`). The armor stand only takes half of every hit.
