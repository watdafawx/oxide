package oxide.a_half_finished_parent;

public class Zombie extends Mob {
    public Zombie() {
        super("Zombie");
    }

    @Override
    public int attackDamage() {
        throw new UnsupportedOperationException("TODO: a zombie always hits for 3");
    }
}
