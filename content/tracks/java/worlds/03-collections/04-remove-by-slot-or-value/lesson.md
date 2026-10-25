A raffle keeps the ticket numbers still in the drum in a `List<Integer>`. Sometimes a ticket goes by its place in the drum, sometimes by its number. On a list of numbers, Java can mix those two up.

**Two removes.** A `List` has `remove(int index)`, "take out whatever is at this position", and `remove(Object o)`, "take out the first item equal to this". Same name, different inputs: overloading, as in the Basics world.

**Text is easy.** On a `List<String>` there is no doubt: `remove(0)` is position 0, and `remove("ash")` is the value, because `"ash"` is not an `int`.

**Numbers are not.** On a `List<Integer>`, what is `tickets.remove(1)`: slot 1 or ticket number 1? Java picks the overload that fits without boxing, so `remove(int)`, the slot. Ticket 1 stays in the drum, and some other ticket is gone.

**Ask for the value.** To mean the value, box it yourself: `Integer.valueOf(1)` is an `Integer`, not an `int`, so only `remove(Object)` fits.

```java
List<Integer> pings = new ArrayList<>(List.of(40, 0, 87));
pings.remove(0);                    // slot 0 goes:    [0, 87]
pings.remove(Integer.valueOf(0));   // the value 0 goes: [87]
```

**Change a copy.** The checker hands you lists made with `List.of`, which refuse any change. The starter first makes a changeable copy with `new ArrayList<>(tickets)`, and you change that.

## Your turn

- In `dropSlot`, remove whatever sits at position `slot` from the copy.
- In `dropNumber`, remove the ticket `number` itself from the copy, even when that number is also a valid position.
