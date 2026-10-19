**Why now:** every mob has a name, health and a way to describe itself. A Zombie has all of that, plus its own groan and a weakness to daylight. With **inheritance** `Zombie` starts from `Mob` and only writes what is different, so your server does not need 20 copies of the same health code.

```java
public class Animal {
    private final String name;
    private final int health;
    public Animal(String name, int health) { this.name = name; this.health = health; }

    public String noise() { return "..."; }
    public String describe() { return name + " (" + health + " hp) says " + noise(); }
}

public class Cat extends Animal {
    public Cat(String name) { super(name, 9); }         // run the parent constructor first

    @Override
    public String noise() { return "meow"; }             // replace the parent's version

    @Override
    public String describe() {
        return super.describe() + ", purring";           // keep the parent's text, add more
    }
}

new Cat("Tom").describe();   // "Tom (9 hp) says meow, purring"
```

- `extends` = "is a". `Cat` gets every method of `Animal`.
- `@Override` marks a method that replaces the parent's. The compiler checks you spelled it right.
- `super(...)` calls the parent constructor. `super.method()` calls the parent's version of a method.
- `describe()` in the parent calls `noise()`: Java picks the **Cat's** `noise()` even inside the parent's code.

## Your turn

- Open `src/main/java/oxide/inheritance_and_super/Zombie.java`. `Mob` is done.
- In the constructor pass `20` hp to `super`.
- Override `ambientSound()` to return `"Rrrgh"`.
- Override `describe()` so it keeps the `Mob` text and adds `" burns in daylight"`.
