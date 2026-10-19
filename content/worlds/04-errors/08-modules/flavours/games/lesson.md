**Why now:** the loader is growing two jobs: reading the `manifest` and working out the `resolver` (which libraries each mod drags in). Keeping them in separate **modules** stops them tripping over each other, and `pub` decides what the rest of the launcher may touch.

```rust
mod server {
    const SECRET: u32 = 7;            // private: only this module and its children

    pub mod worlds {                  // pub: visible from outside
        pub fn count() -> u32 {
            super::SECRET * 2         // super:: = parent module
        }
    }

    pub fn total() -> u32 {
        self::worlds::count() + 1     // self:: = this module
    }
}

use server::total;                    // bring a path into scope
```

- **`mod name { ... }`** defines a module. Items are **private** until marked `pub`.
- **`super::`** goes up one level, **`self::`** stays, **`crate::`** starts at the root.
- **`use`** shortens a path so you don't type it every time.

## Your turn

- Open `src/lib.rs`.
- Fill in `manifest::size_mb` (use `super::MB_PER_MOD`), `resolver::library_mb`, `pack::total_mb`.
- In `describe`, `use` the `total_mb` function and call it.
