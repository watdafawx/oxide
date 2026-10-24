package oxide.is_a;

public class Mob {
    private final String name;
    private final int health;

    public Mob(String name, int health) {
        this.name = name;
        this.health = health;
    }

    public String name() {
        return name;
    }

    public int health() {
        return health;
    }

    public String ambientSound() {
        return "silence";
    }

    public String speak() {
        return name + " says " + ambientSound();
    }
}
