package oxide.interfaces;

public class Zombie implements Damageable {
    private int health = 20;

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO: return \"Zombie\"");
    }

    @Override
    public int health() {
        return health;
    }

    @Override
    public void damage(int amount) {
        throw new UnsupportedOperationException("TODO: lower health by amount, but never below 0");
    }
}
