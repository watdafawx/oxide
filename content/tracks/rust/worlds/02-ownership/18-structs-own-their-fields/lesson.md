A note is saved and kept around long after the text it was typed from is gone. Try to give it a borrowed `&str` field for its title and Rust stops you.

**Try it.** The starter's `Note` has `title: &str`. Rust refuses to build:

```text
error[E0106]: missing lifetime specifier
```

**Why it stops you.** A `&str` field would point at text owned by someone else. If that text is dropped first, the note holds a dangling pointer. Rust has to know how long the borrowed text lives, and the struct does not say. (The help line offers a way to say it; that is the next quest.)

**Own it instead.** Make the field a `String`. The note owns its title, carries it wherever it goes, and drops it when the note is dropped. Owning is the simple default: reach for it first.

```rust
struct Jar {
    label: String,     // the jar owns its label
    grams: u32,
}

fn new_jar(label: &str, weight: u32) -> Jar {
    Jar { label: label.to_string(), grams: weight }
}
```

**Ownership nests.** A struct owns its fields, and a `Vec` owns its items. Drop the outer value and everything inside is dropped with it, in one go.

## Your turn

- Save and read the error on `Note`.
- Change the `title` field to a `String`.
- In `Note::new`, store an owned copy of `title`.
