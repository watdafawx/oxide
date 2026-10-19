package oxide.interfaces;

public class Zombie implements Damageable {
    private int health = 20;

    @Override
    public String name() {
        return "Zombie";
    }

    @Override
    public int health() {
        return health;
    }

    @Override
    public void damage(int amount) {
        health = Math.max(0, health - amount);
    }
}
