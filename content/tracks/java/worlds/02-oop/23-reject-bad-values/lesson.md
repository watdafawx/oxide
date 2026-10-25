A parcel of -1 grams is nonsense, and so is the label `"  books "` with stray spaces. A record writes its constructor for you, which is great until you need it to check something. You want to add rules to that constructor without writing it all out again.

**A compact constructor.** Inside the record, `public Parcel { ... }` with no brackets after the name is a **compact constructor**. It runs first, with the components as parameters, and Java stores them in the fields after it finishes. So it is the place for rules only.

**Tidy on the way in.** Assigning to a parameter there changes what gets stored: `label = label.trim();` means the record keeps the trimmed text. No `this.`, because the fields are set after your code.

**Refuse with throw.** For a value that cannot be fixed, stop. **`throw`** ends the constructor at once with an error, an **exception**, instead of a finished object. Nobody ever holds a broken parcel:

```java
public Tag {
    if (level < 0) {
        throw new IllegalArgumentException("level must not be negative: " + level);
    }
    name = name.trim();
}
```

**Read the line.** `new IllegalArgumentException("...")` builds the error object, the way `new` builds any object, with a message for the red text. `IllegalArgumentException` is Java's ready-made exception for "you passed me a bad value". It is also what the starter's TODO lines have done all along, with `UnsupportedOperationException`, "not written yet". World 04 teaches catching them.

## Your turn

- Add a compact constructor to `Parcel`.
- In it, throw an `IllegalArgumentException` when `grams` is below 0.
- Then trim `label`. Save. Both objectives turn green.
