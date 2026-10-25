Now wire it for real: a power pole owns its machines with `Rc`, and each machine points back at its pole with a `Weak`. Owning goes from pole to machine, looking goes back. A machine asks for its pole's name through that link.

**Which way owns.** The rule of thumb: **the owning direction is `Rc`, the back-pointer is `Weak`**. A pole decides whether its machines stay powered; a machine only needs to find its pole while there still is one.

**A cell for one field.** Only the pole's list of machines changes after it is built, so only that field is a `RefCell`: `machines: RefCell<Vec<Rc<Machine>>>`. Names and the pole link are set once, when each value is built, so a plain `Rc<PowerPole>` and `Rc<Machine>` are enough.

**Weak::new.** A machine built before any pole has nothing to point at. `Weak::new()` makes a link that points at nothing; `upgrade()` on it always gives `None`.

```rust
let port = Rc::new(Roboport { name: "roboport-1".to_string() });
let bot = Robot { name: "bot-7".to_string(), home: Rc::downgrade(&port) };

println!("{}", bot.home.upgrade().is_some()); // true: the roboport still stands
```

**Check the counts.** After `place_machine`, the pole's strong count is still 1 (only your handle owns it) and its weak count is 1 (the machine's link). The machine's strong count is 2: your handle plus the pole's `machines` list.

## Your turn

- In `place_machine`, build the machine with `pole: Rc::downgrade(pole)`, push an `Rc::clone` of it onto the pole's `machines`, and return it.
- In `pole_name`, `upgrade` the machine's pole link and return a copy of the pole's name, or `None`.
