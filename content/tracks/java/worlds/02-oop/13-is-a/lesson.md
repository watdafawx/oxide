A pet shop app has animals: each has a name, a sound, and a line that describes it. A dog is all of that with its own sound. Copying `Animal`'s fields and methods into `Dog`, then into `Cat`, then into `Parrot`, is twenty lines of the same code in three places.

**Start from a parent.** `public class Dog extends Animal` reads "a Dog is an Animal". With **`extends`**, `Dog` starts with every field and method `Animal` has. `Animal` is the **parent** class, `Dog` the **child**. This sharing is called **inheritance**.

**Replace one method.** To change one thing, the child writes a method with the same first line as the parent's. That is **overriding**, and `@Override` above it asks the compiler to check that the parent really has such a method:

```java
public class Car extends Vehicle {
    @Override
    public String noise() {
        return "vroom";     // replaces Vehicle's noise()
    }
}
```

**Parent code sees it.** `Animal`'s `describe()` calls `sound()`. On a `Dog`, Java runs the **Dog's** `sound()`, even though `describe()` is written in `Animal`. The object decides which version runs, not the class the code sits in.

**The constructor line.** The starter's `Dog` constructor says `super(name);`. It hands the name to `Animal`'s constructor, so the parent's fields are set up first. The next quest is about `super`.

**Interface or parent.** An interface shares a promise. A parent class shares real fields and bodies. A class may implement many interfaces but extends only one parent.

## Your turn

- In `Dog`, write `sound()` with `@Override`, handing back `"Woof"`.
- Save. Both objectives turn green.
