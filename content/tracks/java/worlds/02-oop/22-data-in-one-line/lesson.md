In the last three quests you wrote `equals`, `hashCode` and `toString` by hand for `Money`, on top of fields, a constructor and accessors. That is thirty lines for a class that only holds two values, and every one of them can be got wrong. For a class that is just data, Java has a one-line version.

**List the fields.** A **record** is declared with its fields, called **components**, in brackets: `public record Tag(String name, int level) {}`. That single line is the whole class.

**What Java writes.** From that line Java writes a private final field for each component, a constructor taking them in order, an accessor for each, and `equals`, `hashCode` and `toString` that use all the components. Everything you did by hand, done the same way every time.

**Accessors, no get.** The accessor has the component's name: `tag.name()`, `tag.level()`. That is why your classes in this world used `owner()`, not `getOwner()`.

**It cannot change.** The fields are final and there are no setters, so a record is **immutable**: once built, it holds its values forever. Its `toString` reads like this:

```java
Tag fire = new Tag("fire", 3);
fire.toString();                  // Tag[name=fire, level=3]
fire.equals(new Tag("fire", 3));  // true
```

**When to use one.** A record fits values: amounts, positions, ids, settings. A thing that changes over time, like an account with a balance, stays a class.

## Your turn

- Replace the whole `Parcel` class with a record that has the components `String label` and `int grams`, in that order.
- Save. Both objectives turn green.
