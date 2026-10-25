The shop keeps a set of prices it has already shown, so it never shows one twice. With `equals` from last quest, two 12.50 EUR objects are equal. Yet the set says it has never seen the second one, and holds both. `equals` has a partner you have not written yet.

**Sets look by number.** World 03 teaches Java's ready-made collections. One of them, `HashSet`, holds each value once and finds things fast. To be fast, it first sorts objects into buckets by a number from **`hashCode()`**, then calls `equals` only on the objects in that one bucket.

**The contract.** If `a.equals(b)`, then `a.hashCode()` must equal `b.hashCode()`. The `hashCode` from `Object` is different for every object, so two equal amounts land in different buckets, and `equals` is never even asked. The set answers "not found".

**Same fields as equals.** The fix is a `hashCode` built from exactly the fields `equals` compares. Two different amounts may share a number now and then; that only costs a little speed.

**Let Java mix it.** `Objects.hash(...)` takes the fields and mixes them into one number. `Objects` is a helper class from `java.util`, so the file needs `import java.util.Objects;` at the top; the starter already has that line:

```java
@Override
public int hashCode() {
    return Objects.hash(x, y);      // the fields equals compares
}
```

**The rule to keep.** Override `equals`, override `hashCode` in the same breath. Most "my map lost my object" bugs are one without the other.

## Your turn

- In `hashCode`, hand back `Objects.hash(...)` of the two fields `equals` compares.
- Save. Both objectives turn green.
