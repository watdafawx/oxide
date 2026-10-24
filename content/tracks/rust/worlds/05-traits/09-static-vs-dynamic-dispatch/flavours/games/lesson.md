You have now written both: `total_health<T: Entity>` and a `Vec<Box<dyn Entity>>`. Both say "any mob", and under the hood they find the method in two different ways. Picking between them is a choice you will make often.

**One copy per type.** With a generic `fn health_of<T: Entity>(m: &T)`, the compiler writes a separate copy of `health_of` for each mob type you call it with, one for `Zombie`, one for `Creeper`. This is called **monomorphisation**, which means "making one version per type". Each copy knows exactly which `health` to call, so there is no searching while the game runs. This is **static dispatch**: the method is picked when the program is built.

**One copy and a table.** With `&dyn Entity`, there is only one `health_of`. The reference carries two addresses: one to the mob's data and one to a small table listing that type's methods, the **vtable**. Each call looks `health` up in the table first. This is **dynamic dispatch**: the method is picked while the game runs.

```rust
fn health_of<T: Entity>(m: &T) -> u32 { m.health() }      // static: one copy per T
fn health_of_dyn(m: &dyn Entity) -> u32 { m.health() }    // dynamic: one copy, a vtable lookup
```

**What each costs.** Static dispatch gives the fastest call, but more compiled code, and a `Vec<Zombie>` holds only zombies. Dynamic dispatch costs one table lookup per call, tiny but real, and buys you one world list of every mob.

**Rule of thumb.** Default to generics. Reach for `dyn` when types are mixed in one collection, or only known while the game runs: the entity list of a world, mobs that a mod adds at startup.

**For the curious: not every trait can be dyn.** The vtable needs one entry per method, callable without knowing the type. A method that returns `Self` (like `Clone::clone`) or has its own type parameter can't be listed that way, so there is no `dyn Clone`. The compiler calls such a trait not **dyn compatible**.

## Your turn

- Answer the three questions. No code this time.
