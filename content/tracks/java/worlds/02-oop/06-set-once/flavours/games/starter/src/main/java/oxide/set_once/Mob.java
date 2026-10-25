package oxide.set_once;

public class Mob {
    // TODO: a mob's type never changes after the constructor: make it final
    private String type;
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
        // TODO: take amount away from this mob's health
    }
}
