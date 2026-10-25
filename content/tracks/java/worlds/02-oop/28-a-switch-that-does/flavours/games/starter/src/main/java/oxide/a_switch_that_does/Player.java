package oxide.a_switch_that_does;

public class Player {
    private int health = 20;
    private int respawns;

    public int health() {
        return health;
    }

    /** How many times the player respawned. */
    public int respawns() {
        return respawns;
    }

    /** Heal raises health, Hurt lowers it, Respawn sets it to 20 and counts the respawn. */
    public void apply(Command command) {
        throw new UnsupportedOperationException("TODO: a switch statement over command, one arm per record; the Respawn arm is a block with two steps");
    }
}
