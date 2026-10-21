package oxide.if_and_else;

public class PlayerCount {
    /** Forces a reported player count into min..max. */
    public static int clampPlayerCount(int playerCount, int min, int max) {
        if (playerCount < min) {
            return min;
        }
        if (playerCount > max) {
            return max;
        }
        return playerCount;
    }

    /** -1 when players are leaving, 0 when steady, 1 when joining. */
    public static int trend(int delta) {
        if (delta < 0) {
            return -1;
        } else if (delta > 0) {
            return 1;
        } else {
            return 0;
        }
    }
}
