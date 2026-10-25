A zombie targets whichever player is closer. "Closer" means comparing distances, and `PartialOrd`, the trait behind `<`, has a rule about what it may compare.

**Try the tempting thing.** Implement `PartialOrd` on `Vec3` by length. Now `(3, 4, 0) <= (5, 0, 0)` and `(5, 0, 0) <= (3, 4, 0)` are both true, since both are 5 blocks away, but the derived `==` compares the fields and says they differ.

**Why they must agree.** Sorting, `max`, `BTreeMap` and binary search all assume that "neither is less than the other" means "equal". Break that and the zombie's target choice looks random, and nothing in the build warns you.

**Order the thing you mean.** Put the distance in a type of its own, `Distance(f64)`. Two `Distance`s are equal when their numbers are and ordered when their numbers are: one key for both, so `<` and `==` can't disagree.

**Partial, because of NaN.** `partial_cmp` returns an **`Option<Ordering>`**: `Some(Ordering::Less)`, `Some(Ordering::Equal)`, `Some(Ordering::Greater)`, or `None` when the two have no order at all. For `f64` that is NaN again: it is not less than, equal to or greater than 1.0.

```rust
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct Level(u32);

impl PartialOrd for Level {              // powers <, >, <=, >=
    fn partial_cmp(&self, other: &Level) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)     // the same field == uses
    }
}
```

**Derive would agree too.** `#[derive(PartialEq, PartialOrd)]` on `Distance` writes exactly this, from the same field, so the two always match. Writing it once by hand shows you what derive does for you.

## Your turn

- In `impl PartialOrd for Distance`, compare the numbers inside the two distances.
- Write `closer`: the offset with the smaller distance. Ties go to `a`.
