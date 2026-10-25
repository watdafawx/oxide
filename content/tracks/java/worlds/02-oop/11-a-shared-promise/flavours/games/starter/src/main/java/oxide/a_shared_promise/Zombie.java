package oxide.a_shared_promise;

public class Zombie implements Damageable {
    private int health = 20;

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO: hand back the text \"Zombie\"");
    }

    @Override
    public int health() {
        return health;
    }

    @Override
    public void damage(int amount) {
        throw new UnsupportedOperationException("TODO: take amount from health, never below 0 (Math.max(0, ...) picks the larger)");
    }
}
