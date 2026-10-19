package oxide.polymorphism;

public abstract class Mob {
    private final String name;

    protected Mob(String name) {
        this.name = name;
    }

    public String name() {
        return name;
    }

    /** Damage of one attack, in half-hearts. */
    public abstract int attackDamage();
}
