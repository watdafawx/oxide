A module groups code and controls who can see it. Big programs are trees of modules.

```rust
mod garden {
    const SECRET: i32 = 7;            // private: only this module and its children

    pub mod flowers {                 // pub: visible from outside
        pub fn count() -> i32 {
            super::SECRET * 2         // super:: = parent module
        }
    }

    pub fn total() -> i32 {
        self::flowers::count() + 1    // self:: = this module
    }
}

use garden::total;                    // bring a path into scope
```

- **`mod name { ... }`** defines a module. Items are **private** until marked `pub`.
- **`super::`** goes up one level, **`self::`** stays, **`crate::`** starts at the root.
- **`use`** shortens a path so you don't type it every time.

## Your turn

- Open `src/lib.rs`.
- Fill in `circle::area` (use `super::PI`), `square::area`, `geometry::total_area`.
- In `describe`, `use` the `total_area` function and call it.
