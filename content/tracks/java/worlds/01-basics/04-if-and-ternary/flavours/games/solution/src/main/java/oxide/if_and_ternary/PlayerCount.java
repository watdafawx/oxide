package oxide.if_and_ternary;

public class PlayerCount {
    /** Forces a reported player count into min..max. */
    public static int clampPlayerCount(int playerCount, int min, int max) {
        if (playerCount < min) {
            return min;
        }
        return playerCount > max ? max : playerCount;
    }

    /** -1 when players are leaving, 0 when steady, 1 when joining. */
    public static int trend(int delta) {
        if (delta < 0) {
            return -1;
        } else if (delta > 0) {
            return 1;
        }
        return 0;
    }
}
