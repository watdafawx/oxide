Mod sites give every mod a number, its project id, and the organiser keeps the pack as a `List<Integer>` of them. Sometimes a mod goes by its slot in the list, sometimes by its project id. On a list of numbers, Java can mix those two up.

**Two removes.** A `List` has `remove(int index)`, "take out whatever is at this position", and `remove(Object o)`, "take out the first item equal to this". Same name, different inputs: overloading, as in the Basics world.

**Text is easy.** On a `List<String>` there is no doubt: `remove(0)` is position 0, and `remove("iris")` is the value, because `"iris"` is not an `int`.

**Numbers are not.** On a `List<Integer>`, what is `ids.remove(1)`: slot 1 or project 1? Java picks the overload that fits without boxing, so `remove(int)`, the slot. Project 1 stays in the pack, and some other mod is gone.

**Ask for the value.** To mean the value, box it yourself: `Integer.valueOf(1)` is an `Integer`, not an `int`, so only `remove(Object)` fits.

```java
List<Integer> pings = new ArrayList<>(List.of(40, 0, 87));
pings.remove(0);                    // slot 0 goes:    [0, 87]
pings.remove(Integer.valueOf(0));   // the value 0 goes: [87]
```

**Change a copy.** The checker hands you lists made with `List.of`, which refuse any change. The starter first makes a changeable copy with `new ArrayList<>(projectIds)`, and you change that.

## Your turn

- In `dropSlot`, remove whatever mod sits at position `slot` from the copy.
- In `dropProject`, remove `projectId` itself from the copy, even when that number is also a valid position.
