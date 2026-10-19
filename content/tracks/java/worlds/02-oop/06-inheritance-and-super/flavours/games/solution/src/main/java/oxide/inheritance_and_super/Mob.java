package oxide.inheritance_and_super;

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

    public String describe() {
        return name + " (" + health + " hp)";
    }
}
