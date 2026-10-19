package oxide.polymorphism;

public class Zombie extends Mob {
    public Zombie(String name) {
        super(name);
    }

    @Override
    public int attackDamage() {
        return 3;
    }
}
