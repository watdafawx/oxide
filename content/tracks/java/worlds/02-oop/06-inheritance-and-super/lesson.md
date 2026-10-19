**Why this matters:** a Dog has everything an Animal has (a name, a way to describe itself), plus its own tricks. **Inheritance** lets `Dog` start from `Animal` and only write what is different, instead of copying 20 lines.

```java
public class Vehicle {
    private final String name;
    public Vehicle(String name) { this.name = name; }
    public String name() { return name; }

    public String noise() { return "..."; }
    public String describe() { return name + " goes " + noise(); }
}

public class Car extends Vehicle {
    public Car(String name) { super(name); }           // run the parent constructor first

    @Override
    public String noise() { return "vroom"; }           // replace the parent's version

    @Override
    public String describe() {
        return super.describe() + " on four wheels";    // keep the parent's text, add more
    }
}

new Car("Mini").describe();   // "Mini goes vroom on four wheels"
```

- `extends` = "is a". `Car` gets every method of `Vehicle`.
- `@Override` marks a method that replaces the parent's. The compiler checks you spelled it right.
- `super(...)` calls the parent constructor. `super.method()` calls the parent's version of a method.
- `describe()` in the parent calls `noise()`: Java picks the **Car's** `noise()` even inside the parent's code.

## Your turn

- Open `src/main/java/oxide/inheritance_and_super/Dog.java`. `Animal` is done.
- Override `sound()` to return `"Woof"`.
- Override `describe()` so it keeps the `Animal` text and adds `" and wags its tail"`.
