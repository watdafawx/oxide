package oxide.a_shared_promise;

/** Anything that can be hurt: it has a name and health, and takes damage. */
public interface Damageable {
    String name();

    int health();

    void damage(int amount);
}
