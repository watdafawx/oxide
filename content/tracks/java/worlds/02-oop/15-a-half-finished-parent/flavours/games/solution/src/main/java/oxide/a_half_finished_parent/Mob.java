package oxide.a_half_finished_parent;

public abstract class Mob {
    private final String name;

    public Mob(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }

    /** How much damage one attack does. Every kind of mob says how. */
    public abstract int attackDamage();
}
