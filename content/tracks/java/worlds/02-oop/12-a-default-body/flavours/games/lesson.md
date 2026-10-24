The debug overlay wants a status line above every damageable thing: `Zombie: 20 hp`, or `Zombie: dead`. You could write that method in `Zombie`, again in `ArmorStand`, and again in every mob to come. It is the same code each time, and it only needs things every Damageable already promises.

**A body in the promise.** A method in an interface marked **`default`** has a body. Every class that implements the interface gets it for free, without writing a line.

**It can use the promise.** Inside a default method you may call the interface's other methods, like `name()` and `health()`. You do not know which class you are in, but you know it kept the promise, so those calls always work:

```java
public interface Burnable {
    String name();
    int fuelTicks();

    default String label() {
        return name() + " burns for " + fuelTicks() + " ticks";
    }
}
```

**Two cases.** The status has two shapes, alive and dead, so the body needs an `if` on `health()` before it builds the text.

**Still yours to change.** A class may write its own `status()` with `@Override`, and then its version wins. Most do not need to, and that is the point.

## Your turn

- In `Damageable.status()`, hand back `"<name>: dead"` when health is 0 or less.
- Otherwise hand back `"<name>: <health> hp"`.
- Save. Both objectives turn green.
