Zombies and skeletons both have a name, so a shared parent `Mob` makes sense. But how hard does a plain mob hit? There is no answer: the parent knows every mob attacks, not how.

**A class you cannot build.** Mark the parent **`abstract`**: `public abstract class Mob`. It can have fields, constructors and methods like any class, but `new Mob("thing")` is refused. Only its children can be built:

```text
error: Mob is abstract; cannot be instantiated
```

**A method with no body.** Inside it, `public abstract int attackDamage();` declares a method without a body, ending in `;`. Every child that can be built must override it, just like a method an interface promises.

**Why not a dummy body.** A parent version that returns 0 would compile, and a new mob that forgot to override it would quietly do no damage. With `abstract`, forgetting is a compile error instead of a balance bug.

**A child fills it in.** The rest of the child is ordinary: its own fields, `super(...)`, and the missing method:

```java
public class Pickaxe extends Tool {
    public Pickaxe() {
        super("pickaxe");
    }

    @Override
    public int durability() {
        return 250;
    }
}
```

## Your turn

- In `Zombie.attackDamage`, hand back 3.
- In `Skeleton.attackDamage`, hand back 2 for every arrow.
- Save. Both objectives turn green.
