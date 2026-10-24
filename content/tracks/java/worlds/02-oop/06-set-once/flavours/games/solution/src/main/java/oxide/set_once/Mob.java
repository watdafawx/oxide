package oxide.set_once;

public class Mob {
    private final String type;
    private int health;

    public Mob(String type, int health) {
        this.type = type;
        this.health = health;
    }

    public String type() {
        return type;
    }

    public int health() {
        return health;
    }

    /** Takes amount away from health. */
    public void hurt(int amount) {
        this.health = this.health - amount;
    }
}
