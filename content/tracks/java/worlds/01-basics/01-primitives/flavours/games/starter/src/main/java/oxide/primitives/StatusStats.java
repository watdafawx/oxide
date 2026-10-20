package oxide.primitives;

public class StatusStats {
    /** Adds two player counts. Overflow wraps around silently. */
    public static int addPlayers(int playerCount, int joining) {
        return 0; // TODO: add the two player counts (a wrap past the max is expected)
    }

    /** Total online time: players times seconds, correct even when it passes 2.1 billion. */
    public static long playerSeconds(int players, int seconds) {
        return players * seconds; // TODO: this multiplies as int and then widens, so it still wraps
    }
}
