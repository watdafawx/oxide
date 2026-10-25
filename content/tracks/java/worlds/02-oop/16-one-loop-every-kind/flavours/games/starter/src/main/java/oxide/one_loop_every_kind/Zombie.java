package oxide.one_loop_every_kind;

public class Zombie extends Mob {
    public Zombie() {
        super("Zombie");
    }

    @Override
    public int attackDamage() {
        return 3;
    }
}
