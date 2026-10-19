**Why this matters:** payroll code that says "if salaried do this, else if hourly do that" breaks every time the company hires a new kind of worker. With **polymorphism** the loop never changes: each object knows how to pay itself.

An **abstract** class is a half-finished parent: it declares what every child must do, without saying how.

```java
public abstract class Pet {
    private final String name;
    protected Pet(String name) { this.name = name; }
    public String name() { return name; }

    public abstract int mealsPerDay();          // no body: each child must write it
}

public class Cat extends Pet {
    public Cat(String name) { super(name); }
    @Override public int mealsPerDay() { return 3; }
}

public class Fish extends Pet {
    public Fish(String name) { super(name); }
    @Override public int mealsPerDay() { return 1; }
}

int total = 0;
for (Pet p : List.of(new Cat("Tom"), new Fish("Nemo"))) {
    total += p.mealsPerDay();      // the right version runs for each: 3 + 1 = 4
}
```

- `abstract` class: you cannot write `new Pet(...)`; only children can be built.
- `abstract` method: no body, every non-abstract child must override it.
- A `List<Pet>` can hold any mix of children. Calling `p.mealsPerDay()` picks the child's version at run time.

## Your turn

- Open `Salaried.java` and `Hourly.java`: implement `monthlyPay()` (`Employee` already declares it abstract).
- Open `Payroll.java`: `total(List<Employee>)` adds up everyone's `monthlyPay()`, with no `instanceof`.
