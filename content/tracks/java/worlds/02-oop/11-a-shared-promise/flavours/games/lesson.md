Arrows, lava and explosions should hurt zombies, armor stands and boats alike. The damage code should not care which one it hit, only that it can call `damage(...)`. You want a way to say "anything that can be hurt does this", and let the compiler check it.

**A promise with no steps.** An **interface** is a list of method lines without bodies: what a thing can do, not how. `Damageable` says every damageable thing has a `name()`, a `health()` and a `damage(int amount)`.

**Keep the promise.** A class that writes `implements Damageable` promises to give every one of those methods a body. Forget one, and the compiler names it:

```text
error: Zombie is not abstract and does not override abstract method damage(int) in Damageable
```

**One type, many classes.** A variable can have the interface as its type: `Damageable target = new Zombie();`. An arrow that takes a `Damageable` hurts a zombie, an armor stand, or a mob your mod adds next year, because all it uses is the promise.

**In code.** Here coal keeps the `Burnable` promise. `@Override` above a method says "this fills in a promised method"; the compiler checks the spelling, the way `@Test` marks a test:

```java
public interface Burnable {
    int fuelTicks();
}

public class Coal implements Burnable {
    @Override
    public int fuelTicks() {
        return 1600;
    }
}
```

## Your turn

- In `Zombie`, fill in `name` (`"Zombie"`) and `damage` (take the whole amount from `health`, never below 0).
- In `ArmorStand`, fill in `name` (`"Armor stand"`) and `damage` (take only half the amount).
- Save. Both objectives turn green.
