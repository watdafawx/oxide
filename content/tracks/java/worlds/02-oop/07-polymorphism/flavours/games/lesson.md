**Why now:** a night-time wave mixes zombies, skeletons and whatever your mod adds next. Damage code that says "if zombie do this, else if skeleton do that" breaks with every new mob. With **polymorphism** the loop never changes: each mob knows its own attack.

An **abstract** class is a half-finished parent: it declares what every child must do, without saying how.

```java
public abstract class Tool {
    private final String name;
    protected Tool(String name) { this.name = name; }
    public String name() { return name; }

    public abstract int durability();           // no body: each child must write it
}

public class Pickaxe extends Tool {
    public Pickaxe() { super("pickaxe"); }
    @Override public int durability() { return 250; }
}

public class Shears extends Tool {
    public Shears() { super("shears"); }
    @Override public int durability() { return 238; }
}

int total = 0;
for (Tool t : List.of(new Pickaxe(), new Shears())) {
    total += t.durability();       // the right version runs for each: 250 + 238
}
```

- `abstract` class: you cannot write `new Tool(...)`; only children can be built.
- `abstract` method: no body, every non-abstract child must override it.
- A `List<Tool>` can hold any mix of children. Calling `t.durability()` picks the child's version at run time.

## Your turn

- Open `Zombie.java` and `Skeleton.java`: implement `attackDamage()` (`Mob` already declares it abstract).
- Open `Wave.java`: `totalDamage(List<Mob>)` adds up every mob's `attackDamage()`, with no `instanceof`.
