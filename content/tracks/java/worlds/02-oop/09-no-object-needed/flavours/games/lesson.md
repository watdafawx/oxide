Your mod wants one front door for new mobs: `Mob.spawn("Zombie")`, which also cleans the kind to `zombie`. There is no mob yet when you ask for one, so this method cannot belong to a mob. It belongs to the class.

**A static method.** A method marked **`static`** belongs to the class, like a static field. You call it through the class name, `Mob.spawn("zombie")`, with no object before the dot. Inside it there is no `this`, because no object is being worked on.

**Why your methods were static.** Every method in worlds 00 and 01 was `public static`, because there was no object to call it on: the checker called `Report.announce(...)` straight on the class. `String.format(...)` is static for the same reason. Now you know both words: `public` lets anyone call it, `static` means no object needed.

**And main.** `public static void main(String[] args)` is static because when a program starts, no objects exist yet. Java needs one method it can call on the class alone, and that is `main`.

**One way in.** A static method that builds objects is a **factory**. Make the constructor `private` and the factory becomes the only way to get a mob, so its clean-up can never be skipped:

```java
public class Arrow {
    private final String tip;

    private Arrow(String tip) {           // only Arrow's own code may call this
        this.tip = tip;
    }

    public static Arrow shoot(String tip) {
        return new Arrow(tip.toLowerCase());
    }
}
```

## Your turn

- In `spawn`, build a `Mob` from the trimmed, lower-case kind and hand it back.
- Make the constructor `private`.
- Save. Both objectives turn green.
