package oxide.a_shared_promise;

public class ArmorStand implements Damageable {
    private int health = 6;

    @Override
    public String name() {
        return "Armor stand";
    }

    @Override
    public int health() {
        return health;
    }

    @Override
    public void damage(int amount) {
        health = Math.max(0, health - amount / 2);
    }
}
