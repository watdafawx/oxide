You have now written both: `total_area<T: Shape>` and a `Vec<Box<dyn Shape>>`. Both say "any shape", and under the hood they find the method in two different ways. Picking between them is a choice you will make often.

**One copy per type.** With a generic `fn area_of<T: Shape>(s: &T)`, the compiler writes a separate copy of `area_of` for each shape you call it with, one for `Circle`, one for `Rect`. This is called **monomorphisation**, which means "making one version per type". Each copy knows exactly which `area` to call, so there is no searching while the program runs. This is **static dispatch**: the method is picked when the program is built.

**One copy and a table.** With `&dyn Shape`, there is only one `area_of`. The reference carries two addresses: one to the shape's data and one to a small table listing that type's methods, the **vtable**. Each call looks `area` up in the table first. This is **dynamic dispatch**: the method is picked while the program runs.

```rust
fn area_of<T: Shape>(s: &T) -> f64 { s.area() }     // static: one copy per T
fn area_of_dyn(s: &dyn Shape) -> f64 { s.area() }   // dynamic: one copy, a vtable lookup
```

**What each costs.** Static dispatch gives the fastest call, but more compiled code, and a `Vec<T>` still holds a single type. Dynamic dispatch costs one table lookup per call, tiny but real, and buys you one list of mixed types.

**Rule of thumb.** Default to generics. Reach for `dyn` when types are mixed in one collection, or only known while the program runs: shapes a user draws, plugins loaded at startup.

**For the curious: not every trait can be dyn.** The vtable needs one entry per method, callable without knowing the type. A method that returns `Self` (like `Clone::clone`) or has its own type parameter can't be listed that way, so there is no `dyn Clone`. The compiler calls such a trait not **dyn compatible**.

## Your turn

- Answer the three questions. No code this time.
