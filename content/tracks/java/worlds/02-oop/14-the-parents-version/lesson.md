The shop wants one extra detail for dogs: "Rex says Woof and wags its tail". Animal already builds "Rex says Woof". Rewriting that part in `Dog` would copy it, and a fix in `Animal` would no longer reach dogs. The child needs a way to reach its parent.

**The parent's constructor.** **`super(...)`** as the first line of a child's constructor runs the parent's constructor with those values. `Animal`'s fields are private, so this is the only way `Dog` can get a name into them. The starter passes `"Dog"` by mistake, so every dog is called Dog.

**The parent's method.** Inside an overriding method, **`super.describe()`** runs the parent's version of `describe()`, the one you just replaced. Plain `describe()` would call the Dog's own version again, forever.

**Build on it.** Take the parent's text and add to it, so the shared part stays in one place:

```java
@Override
public String describe() {
    return super.describe() + " on four wheels";
}
```

**Why not copy.** When `Animal` changes how it describes, say "Rex the animal says Woof", every child built on `super.describe()` follows along. Copied text would not.

## Your turn

- In the `Dog` constructor, hand `name` to `super(...)` instead of `"Dog"`.
- Add an `@Override` `describe()` that hands back the parent's text plus `" and wags its tail"`.
- Save. Both objectives turn green.
