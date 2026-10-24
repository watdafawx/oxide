package oxide.one_loop_every_kind;

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
