A help desk numbers its tickets: 1, 2, 3, never the same number twice. The "next number" is not part of any one ticket. It is a fact about tickets as a whole, and Java has a word for that.

**Try the obvious thing.** The starter keeps `nextId` as a normal field, and the constructor takes its id from it. Fill in the constructor and save: every ticket says 1. Each new ticket has its own fresh `nextId`, starting at 1, so the counting never carries over.

**One for the class.** A field marked **`static`** belongs to the class itself, not to any object. There is exactly one `nextId`, and every ticket's constructor reads and moves the same one:

```java
public class Badge {
    private static int issued = 0;   // one copy, shared by every Badge
    private final int number;

    public Badge() {
        issued++;
        this.number = issued;
    }
}
```

**You met some already.** `Integer.MAX_VALUE` and `Math.PI` are static fields: you read them through the class name and a dot, with no object in sight. Inside its own class a static field is just its name, `nextId`.

**Use it sparingly.** A static field that changes is shared by everything, so one part of a program can surprise another. Counters like this one are the classic good use; most other fields stay normal.

## Your turn

- In the constructor, store `nextId` in `this.id`, then move `nextId` on by one.
- Save, and read the red: every ticket is 1.
- Make `nextId` `static`. Save again. Both objectives turn green.
