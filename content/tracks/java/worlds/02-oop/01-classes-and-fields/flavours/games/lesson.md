**Why now:** your mod needs inventory slots, and a slot holds a stack: "17 oak logs". An **ItemStack** class is where that number lives, and everything later in this world (items, mobs, events) hangs off classes like it.

```java
public class Arrows {
    private int shots;               // field: starts at 0

    public int shots() {             // method that reads the field
        return shots;
    }

    public void fire() {             // method that changes the field
        shots++;
    }
}

Arrows quiver = new Arrows();
Arrows spare = new Arrows();
quiver.fire();
quiver.fire();
// quiver.shots() == 2, spare.shots() == 0: every object has its own `shots`
```

- `new Arrows()` builds an object. Number fields start at `0`.
- `private` hides the field: other code has to go through methods.
- `void` means the method gives nothing back.

## Your turn

- Open `src/main/java/oxide/classes_and_fields/ItemStack.java`.
- Make `count()` return the `count` field.
- Make `grow()` add one to it.
