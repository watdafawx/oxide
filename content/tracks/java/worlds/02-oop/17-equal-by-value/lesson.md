A shop compares prices: is this 12.50 EUR the same as that 12.50 EUR? Two separate `Money` objects with the same amount answer "no" to `a.equals(b)`. You met this with text in world 01: `==` asks "same object?", and `equals` was the fix. For your own class, you have to teach `equals` what "same" means.

**Same object or same value.** Out of the box, `equals` on your class only says true for the very same object. That is **identity**. For a value like money, coordinates or an id, you want **equality by value**: same fields, same thing.

**Where equals comes from.** Every class quietly extends **`Object`**, the root of all classes, even when it says no `extends`. `Object` gives every object `equals`, `hashCode` and `toString`. You override `equals` to replace the identity check.

**Anything may be passed in.** The parameter is an `Object`, because any object can be compared to a Money, even a String. So check the kind first: `o instanceof Money other` is true when `o` is a Money, and then names it `other`. A `null` fails the check too. This is the one place where asking an object's kind is right.

**Then compare fields.** Numbers with `==`, text and objects with `equals`. Inside `Money`, you may read `other.cents` even though it is private: private means "only this class", and this is the class:

```java
@Override
public boolean equals(Object o) {
    if (!(o instanceof Point other)) {
        return false;
    }
    return x == other.x && y == other.y;
}
```

## Your turn

- In `equals`, hand back `false` when `o` is not a `Money`.
- Otherwise hand back whether both `cents` and `currency` match.
- Save. Both objectives turn green.
