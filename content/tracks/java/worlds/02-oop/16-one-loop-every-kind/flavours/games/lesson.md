A night-time wave mixes zombies, skeletons and whatever your mod adds next. One way to add up the damage is to ask each mob "are you a zombie? a skeleton?" and do the right sum. That code breaks the day you add creepers, and someone has to remember to add a third question.

**An array of objects.** An array can hold objects as well as numbers. `Mob[] night = { new Zombie(), new Skeleton(3) };` holds both kinds, because each of them is a `Mob`.

**Just ask.** Loop over the array and call `attackDamage()` on each one. Java runs each object's own version: 3 for the zombie, 2 per arrow for the skeleton. That is **polymorphism**: one call, and the object decides what it means:

```java
int total = 0;
for (Tool tool : hotbar) {
    total += tool.durability();    // a Pickaxe's 250, a Shears' 238
}
```

**Why it lasts.** The loop never names a kind. A `Creeper` class written next update, extending `Mob`, is counted by the same loop with no change to `Wave`. The checker's second objective slips in exactly such a class.

**The habit.** When you are about to ask an object what kind it is, ask instead whether the parent should have a method for it. Then each child answers for itself.

## Your turn

- In `Wave.totalDamage`, loop over `mobs` and add up every mob's `attackDamage()`.
- Save. Both objectives turn green.
