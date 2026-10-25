The server has to tell mobs apart, even two zombies with the same name, so every entity gets a number: 1, 2, 3, never the same twice. The "next number" is not part of any one mob. It is a fact about mobs as a whole, and Java has a word for that.

**Try the obvious thing.** The starter keeps `nextId` as a normal field, and the constructor takes its id from it. Fill in the constructor and save: every mob is entity 1. Each new mob has its own fresh `nextId`, starting at 1, so the counting never carries over.

**One for the class.** A field marked **`static`** belongs to the class itself, not to any object. There is exactly one `nextId`, and every mob's constructor reads and moves the same one:

```java
public class Arrow {
    private static int fired = 0;    // one copy, shared by every Arrow
    private final int number;

    public Arrow() {
        fired++;
        this.number = fired;
    }
}
```

**You met some already.** `Integer.MAX_VALUE` and `Math.PI` are static fields: you read them through the class name and a dot, with no object in sight. Inside its own class a static field is just its name, `nextId`.

**Use it sparingly.** A static field that changes is shared by everything, so one part of a mod can surprise another. Counters like this one are the classic good use; most other fields stay normal.

## Your turn

- In the constructor, store `nextId` in `this.id`, then move `nextId` on by one.
- Save, and read the red: every mob is entity 1.
- Make `nextId` `static`. Save again. Both objectives turn green.
