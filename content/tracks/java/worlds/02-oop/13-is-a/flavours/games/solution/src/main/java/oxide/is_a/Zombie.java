package oxide.is_a;

public class Zombie extends Mob {
    public Zombie(String name) {
        super(name, 20);
    }

    @Override
    public String ambientSound() {
        return "Rrrgh";
    }
}
