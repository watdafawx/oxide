package oxide.a_shared_promise;

public class ArmorStand implements Damageable {
    private int health = 6;

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO: hand back the text \"Armor stand\"");
    }

    @Override
    public int health() {
        return health;
    }

    @Override
    public void damage(int amount) {
        throw new UnsupportedOperationException("TODO: take amount / 2 from health, never below 0 (Math.max(0, ...) picks the larger)");
    }
}
