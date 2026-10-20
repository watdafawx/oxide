**Why now:** the server has to tell mobs apart, even two zombies with the same name. Every entity gets a number nobody else has. The "next number" is not stored in any one mob. It belongs to the **class**, and that is what `static` means.

```java
public class Arrow {
    private static int fired = 0;         // one copy, shared by every Arrow
    private final int number;

    private Arrow() {                     // private: outsiders can't skip the numbering
        number = ++fired;
    }

    public static Arrow shoot() {         // static factory: called on the class
        return new Arrow();
    }

    public int number() { return number; }
}

Arrow a = Arrow.shoot();    // number 1
Arrow b = Arrow.shoot();    // number 2
```

- `static` field: one copy for the whole class. `static` method: called as `Arrow.shoot()`, no object needed.
- `nextId++` hands out the **current** value and then adds one (`id = nextId++` with nextId at 1 gives 1). `++nextId` adds first and hands out the new value.
- A **static factory** (`spawn`, `of`, `create`) is a named way to build objects. It can validate, cache, or number them.
- Use `static` for things that really are shared. Static state that changes is global state: it makes tests depend on each other, so keep it small.

## Your turn

- Open `src/main/java/oxide/static_members/Mob.java`.
- Write `Mob.spawn(kind)`.
- Make the constructor take the next id from `nextId` (and move `nextId` forward).
