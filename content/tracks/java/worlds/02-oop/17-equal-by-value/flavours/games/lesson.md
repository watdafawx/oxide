Your mod remembers where the player put a chest: (10, 64, -3). A tick later it builds a new `BlockPos` for the block being clicked, also (10, 64, -3), and asks `clicked.equals(chest)`. The answer is "no", and the chest does not open. You met this with text in world 01: `==` asks "same object?", and `equals` was the fix. For your own class, you have to teach `equals` what "same" means.

**Same object or same value.** Out of the box, `equals` on your class only says true for the very same object. That is **identity**. For a value like a position, an id or an amount, you want **equality by value**: same fields, same thing.

**Where equals comes from.** Every class quietly extends **`Object`**, the root of all classes, even when it says no `extends`. `Object` gives every object `equals`, `hashCode` and `toString`. You override `equals` to replace the identity check.

**Anything may be passed in.** The parameter is an `Object`, because any object can be compared to a BlockPos, even a String. So check the kind first: `o instanceof BlockPos other` is true when `o` is a BlockPos, and then names it `other`. A `null` fails the check too. This is the one place where asking an object's kind is right.

**Then compare fields.** Numbers with `==`, text and objects with `equals`. Inside `BlockPos`, you may read `other.x` even though it is private: private means "only this class", and this is the class:

```java
@Override
public boolean equals(Object o) {
    if (!(o instanceof ChunkPos other)) {
        return false;
    }
    return x == other.x && z == other.z;
}
```

## Your turn

- In `equals`, hand back `false` when `o` is not a `BlockPos`.
- Otherwise hand back whether `x`, `y` and `z` all match.
- Save. Both objectives turn green.
