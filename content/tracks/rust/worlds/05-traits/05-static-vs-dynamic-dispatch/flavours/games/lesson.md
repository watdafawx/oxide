**Why now:** the world holds zombies, creepers and villagers at the same time, and the tick loop must walk all of them. There are two ways to say "any type that implements `Entity`", and the tick loop needs the second one.

## Generics: static dispatch

```rust
fn health_of<T: Entity>(mob: &T) -> u32 { mob.health() }
```

The compiler **monomorphises**: it writes a separate `health_of` for every `T` you use. Calls are direct, can be inlined, and cost nothing at runtime. The price is more compiled code, and every `T` in one collection must be the **same** type: a `Vec<Zombie>` holds only zombies.

## `dyn Entity`: dynamic dispatch

```rust
fn health_of(mob: &dyn Entity) -> u32 { mob.health() }
let world: Vec<Box<dyn Entity>> = vec![Box::new(Zombie { hp: 20 }), Box::new(Creeper { hp: 20 })];
```

A `&dyn Entity` is a **fat pointer**: the data plus a pointer to a **vtable** (a table of that mob's methods). Each call looks the method up at runtime. A tiny cost, but one function, one `Vec`, many mob types.

## Rule of thumb

- Default to **generics**. Fast, simple.
- Reach for **`dyn`** when kinds are mixed in one list or only known at runtime: the entity list of a world, mods that add new mobs.
- `dyn` needs a pointer (`&` or `Box`; `Rc` is next world's tool) because its size is unknown.

## Your turn

Answer the two questions below.
