A payroll has salaried staff and hourly staff. Both have a name, so a shared parent `Employee` makes sense. But what is a plain employee's monthly pay? There is no answer: the parent knows every employee is paid, not how.

**A class you cannot build.** Mark the parent **`abstract`**: `public abstract class Employee`. It can have fields, constructors and methods like any class, but `new Employee("Ann")` is refused. Only its children can be built:

```text
error: Employee is abstract; cannot be instantiated
```

**A method with no body.** Inside it, `public abstract int monthlyPay();` declares a method without a body, ending in `;`. Every child that can be built must override it, just like a method an interface promises.

**Why not a dummy body.** A parent version that returns 0 would compile, and a new kind of employee that forgot to override it would quietly get paid nothing. With `abstract`, forgetting is a compile error instead of a payday bug.

**A child fills it in.** The rest of the child is ordinary: its own fields, `super(name)`, and the missing method:

```java
public class Cat extends Pet {
    public Cat(String name) {
        super(name);
    }

    @Override
    public int mealsPerDay() {
        return 3;
    }
}
```

## Your turn

- In `Salaried.monthlyPay`, hand back the yearly pay divided by 12.
- In `Hourly.monthlyPay`, hand back the hourly rate times the hours.
- Save. Both objectives turn green.
