package oxide.interfaces;

public interface Damageable {
    String name();

    int health();

    void damage(int amount);

    /** "<name>: <health> hp", or "<name>: dead" when health is 0. */
    default String status() {
        if (health() <= 0) {
            return name() + ": dead";
        }
        return name() + ": " + health() + " hp";
    }
}
