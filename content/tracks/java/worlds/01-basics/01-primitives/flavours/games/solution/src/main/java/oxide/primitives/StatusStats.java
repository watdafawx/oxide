package oxide.primitives;

public class StatusStats {
    /** Adds two player counts. Overflow wraps around silently. */
    public static int addPlayers(int playerCount, int joining) {
        return playerCount + joining;
    }

    /** Total online time: players times seconds, correct even when it passes 2.1 billion. */
    public static long playerSeconds(int players, int seconds) {
        return (long) players * seconds;
    }
}
