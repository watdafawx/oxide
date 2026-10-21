A courier delivery takes some number of days, and pickup takes none. Each choice needs its own facts: the number of days for one, nothing at all for another. A variant can carry the facts with it.

**A variant with data.** Put the type of the data in round brackets after the variant's name, `Zombie(u32)`. To make one, put the value in the brackets: `Mob::Zombie(20)`. A variant without brackets, like `Bat`, carries nothing.

**Take the data out.** In a match arm, write a name in the brackets and Rust fills it with whatever the variant carries. That name is yours to choose:

```rust
enum Mob {
    Bat,
    Zombie(u32),     // carries its health
}

fn threat(mob: Mob) -> u32 {
    match mob {
        Mob::Bat => 1,
        Mob::Zombie(hp) => hp / 2,
    }
}
```

**Local to the arm.** `hp` exists only in the arm that names it. The next arm cannot see it, and the other variants never had it.

**Different shapes are fine.** In one enum, some variants carry data, some carry none, and each can carry a different amount. `match` handles each arm on its own terms.

## Your turn

- In `days_to_arrive`, give pickup 0 days, a courier its own days, and post its days plus 2.
- Save. Both objectives turn green.
