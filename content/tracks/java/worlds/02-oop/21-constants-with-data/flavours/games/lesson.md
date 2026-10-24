Each rarity has a name colour and a drop weight: common items drop 60 times out of 100, epic ones 5. Those numbers belong to the tier, so they should live in the enum, not in a separate loot table that can drift out of step.

**Constants with values.** Since constants are objects, an enum can have fields and a constructor. Each constant passes its values in brackets, and that calls the constructor once for it:

```java
public enum Dimension {
    OVERWORLD(1), NETHER(8), END(1);

    private final int scale;

    Dimension(int scale) {
        this.scale = scale;
    }
}
```

**The shape.** The constants come first and end with `;`. Fields, constructor and methods follow. The constructor is private by nature: nobody can build a fifth tier with `new`, which is the point of an enum.

**Every constant.** `Rarity.values()` is a static method every enum gets. It hands back an array of all constants, in the order they are written, ready for an enhanced `for`.

**For the curious.** `Rarity.valueOf("EPIC")` finds a constant by its exact name and throws an error for any other text, handy for reading config files. `EPIC.ordinal()` is its position, 3.

## Your turn

- Give each constant its real drop weight: COMMON 60, UNCOMMON 25, RARE 10, EPIC 5.
- In `totalWeight`, loop over `values()` and add up every tier's `weight()`.
- Save. Both objectives turn green.
