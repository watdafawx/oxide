**Why this matters:** support tickets, orders, invoices: each needs a number nobody else has. The "next number" is not stored in any one ticket. It belongs to the **class**, and that is what `static` means.

```java
public class Badge {
    private static int issued = 0;        // one copy, shared by every Badge
    private final int number;

    private Badge() {                     // private: outsiders can't skip the numbering
        number = ++issued;
    }

    public static Badge issue() {         // static factory: called on the class
        return new Badge();
    }

    public int number() { return number; }
}

Badge a = Badge.issue();    // number 1
Badge b = Badge.issue();    // number 2
```

- `static` field: one copy for the whole class. `static` method: called as `Badge.issue()`, no object needed.
- `nextId++` hands out the **current** value and then adds one (`id = nextId++` with nextId at 1 gives 1). `++nextId` adds first and hands out the new value.
- A **static factory** (`of`, `create`, `issue`) is a named way to build objects. It can validate, cache, or number them.
- Use `static` for things that really are shared. Static state that changes is global state: it makes tests depend on each other, so keep it small.

## Your turn

- Open `src/main/java/oxide/static_members/Ticket.java`.
- Write `Ticket.of(title)`.
- Make the constructor take the next id from `nextId` (and move `nextId` forward).
