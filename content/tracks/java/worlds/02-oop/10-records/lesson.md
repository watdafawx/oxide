**Why this matters:** last quest you wrote `equals`, `hashCode` and `toString` by hand for a class that only holds data. That is 25 lines of code to get wrong. A **record** is Java's one-line version: you list the fields, Java writes everything else.

```java
// the long way: fields, constructor, accessors, equals, hashCode, toString...
// the record way:
public record Tag(String name, int level) {

    public Tag {                                   // compact constructor: validation only
        if (level < 0) {
            throw new IllegalArgumentException("level must not be negative: " + level);
        }
    }

    public Tag withLevel(int level) {              // "change" = build a copy
        return new Tag(name, level);
    }
}

Tag a = new Tag("fire", 3);
a.name();                                  // accessor named after the field, no "get"
a.equals(new Tag("fire", 3));              // true: compared by value
a.toString();                              // Tag[name=fire, level=3]
a.withLevel(5);                            // new Tag; `a` is unchanged
```

- Fields are `private final`: a record is **immutable**.
- The **compact constructor** has no parentheses. It runs first, so it is the place to validate (and to normalise, e.g. `label = label.trim();`).
- Records can have extra methods, static methods and implement interfaces. They cannot extend a class.

## Your turn

- Open `src/main/java/oxide/records/Parcel.java`. It is the long-hand class.
- Turn it into `record Parcel(String label, int grams)`.
- Add the compact constructor (grams below 0 is an error) and `withGrams`.
