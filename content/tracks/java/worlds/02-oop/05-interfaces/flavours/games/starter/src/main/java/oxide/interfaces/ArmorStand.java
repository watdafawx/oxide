package oxide.interfaces;

public class ArmorStand implements Damageable {
    private int health = 6;

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO: return \"Armor stand\" (capital A, then a space)");
    }

    @Override
    public int health() {
        return health;
    }

    @Override
    public void damage(int amount) {
        throw new UnsupportedOperationException("TODO: lower health by half of amount (amount / 2 drops the remainder), never below 0");
    }
}
