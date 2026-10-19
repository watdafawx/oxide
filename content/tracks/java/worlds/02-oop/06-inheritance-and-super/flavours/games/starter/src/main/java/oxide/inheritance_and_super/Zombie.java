package oxide.inheritance_and_super;

public class Zombie extends Mob {
    public Zombie(String name) {
        super(name, 0); // TODO: zombies have 20 hp
    }

    // TODO: override ambientSound() to return "Rrrgh"

    // TODO: override describe(): the Mob text (super.describe()) plus " burns in daylight"
}
