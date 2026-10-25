package oxide.the_parents_version;

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

    public String describe() {
        return name + " (" + health + " hp)";
    }
}
