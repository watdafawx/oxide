The debug screen wants one extra detail for zombies: "Bob (20 hp) burns in daylight". Mob already builds "Bob (20 hp)". Rewriting that part in `Zombie` would copy it, and a fix in `Mob` would no longer reach zombies. The child needs a way to reach its parent.

**The parent's constructor.** **`super(...)`** as the first line of a child's constructor runs the parent's constructor with those values. `Mob`'s fields are private, so this is the only way `Zombie` can get its health into them. The starter passes 0 by mistake, so every zombie spawns dead.

**The parent's method.** Inside an overriding method, **`super.describe()`** runs the parent's version of `describe()`, the one you just replaced. Plain `describe()` would call the Zombie's own version again, forever.

**Build on it.** Take the parent's text and add to it, so the shared part stays in one place:

```java
@Override
public String describe() {
    return super.describe() + ", purring";
}
```

**Why not copy.** When `Mob` changes how it describes, say "Bob [20/20 hp]", every child built on `super.describe()` follows along. Copied text would not.

## Your turn

- In the `Zombie` constructor, hand `name` and `20` to `super(...)`.
- Add an `@Override` `describe()` that hands back the parent's text plus `" burns in daylight"`.
- Save. Both objectives turn green.
