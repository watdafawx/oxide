package oxide.inheritance_and_super;

public class Zombie extends Mob {
    public Zombie(String name) {
        super(name, 20);
    }

    @Override
    public String ambientSound() {
        return "Rrrgh";
    }

    @Override
    public String describe() {
        return super.describe() + " burns in daylight";
    }
}
