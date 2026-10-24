package oxide.a_default_body;

/** Anything that can be hurt: it has a name and health, and takes damage. */
public interface Damageable {
    String name();

    int health();

    void damage(int amount);

    /** "<name>: <health> hp", or "<name>: dead" at 0 hp. */
    default String status() {
        if (health() <= 0) {
            return name() + ": dead";
        }
        return name() + ": " + health() + " hp";
    }
}
