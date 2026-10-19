package oxide.interfaces;

public class ArmorStand implements Damageable {
    private int health = 6;

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO");
    }

    @Override
    public int health() {
        return health;
    }

    @Override
    public void damage(int amount) {
        throw new UnsupportedOperationException("TODO");
    }
}
