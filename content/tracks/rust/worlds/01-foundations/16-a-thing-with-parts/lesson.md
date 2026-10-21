So far your planner keeps a label in one variable and a number in another. They belong together, and the first time you pass them around, one gets lost. You want one value that is a pallet: it has a label and a number of boxes.

**Name the bundle.** A **struct** groups values under one name, and each value gets its own name, called a **field**. You describe the shape once, with the type of every field:

```rust
struct Chest {
    label: String,
    slots: u32,
}

let chest = Chest { label: "tools".to_string(), slots: 27 };
let size = chest.slots;      // 27
```

**Text a struct owns.** A struct keeps its own copy of text, which has the type `String`. A plain `"tools"` is only a borrowed piece of text, so `.to_string()` turns it into one the struct can own.

**Make one.** To build a struct, write its name and then every field with a value: `Chest { label: ..., slots: ... }`. Leave one out and Rust tells you which.

**Read a field.** Write the value, a dot, and the field name: `chest.slots`. It works like any other number.

## Your turn

- In `full_pallet`, build a `Pallet` with the label `"paper"` and 64 boxes.
- In `pallet_label`, return text like `paper x64` with `format!`, using both fields.
- Save. Both objectives turn green.
