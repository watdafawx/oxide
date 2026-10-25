Every mob has a name, health and an ambient sound. A zombie is all of that with its own groan. Copying `Mob`'s fields and methods into `Zombie`, then into `Skeleton`, then into every mob your mod adds, is the same health code in twenty places.

**Start from a parent.** `public class Zombie extends Mob` reads "a Zombie is a Mob". With **`extends`**, `Zombie` starts with every field and method `Mob` has. `Mob` is the **parent** class, `Zombie` the **child**. This sharing is called **inheritance**.

**Replace one method.** To change one thing, the child writes a method with the same first line as the parent's. That is **overriding**, and `@Override` above it asks the compiler to check that the parent really has such a method:

```java
public class Cat extends Animal {
    @Override
    public String noise() {
        return "meow";      // replaces Animal's noise()
    }
}
```

**Parent code sees it.** `Mob`'s `speak()` calls `ambientSound()`. On a `Zombie`, Java runs the **Zombie's** `ambientSound()`, even though `speak()` is written in `Mob`. The object decides which version runs, not the class the code sits in.

**The constructor line.** The starter's `Zombie` constructor says `super(name, 20);`. It hands the name and 20 hp to `Mob`'s constructor, so the parent's fields are set up first. The next quest is about `super`.

**Interface or parent.** An interface shares a promise. A parent class shares real fields and bodies. A class may implement many interfaces but extends only one parent.

## Your turn

- In `Zombie`, write `ambientSound()` with `@Override`, handing back `"Rrrgh"`.
- Save. Both objectives turn green.
