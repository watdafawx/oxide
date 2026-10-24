package oxide.the_parents_version;

public class Zombie extends Mob {
    public Zombie(String name) {
        super(name, 20);
    }

    @Override
    public String describe() {
        return super.describe() + " burns in daylight";
    }
}
