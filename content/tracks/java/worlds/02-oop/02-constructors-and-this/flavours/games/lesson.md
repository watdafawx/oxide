**Why now:** the item registry of your mod builds items from the short form (`new Item("Stick")`) and the long form (`new Item("ender_pearl", 16)`). If every constructor repeated the rules ("lower-case ids, stack size 1 to 64"), one copy would drift and you would get two kinds of `stick`.

A class can have several constructors with different parameters (**overloading**). `this(...)` as the first line calls another constructor of the same class:

```java
public class Potion {
    private final String effect;
    private final int seconds;

    public Potion(String effect, int seconds) {   // the one real constructor
        this.effect = effect.toUpperCase();
        this.seconds = Math.max(0, seconds);
    }

    public Potion(String effect) {
        this(effect, 180);                         // reuse it: effect is upper-cased too
    }

    public Potion() {
        this("WATER");                             // chains again
    }
}
```

- `this.effect` is the field; plain `effect` is the parameter. `this.` tells them apart.
- `this(...)` must be the **first** statement of the constructor.
- `final` fields are set once, in a constructor.

## Your turn

- Open `src/main/java/oxide/constructors_and_this/Item.java`.
- In the two-argument constructor: trim and lower-case the name, keep `maxStack` between 1 and 64.
- Make `Item(String)` and `Item()` call another constructor with `this(...)`: default stack size `64`, default name `"air"`.
