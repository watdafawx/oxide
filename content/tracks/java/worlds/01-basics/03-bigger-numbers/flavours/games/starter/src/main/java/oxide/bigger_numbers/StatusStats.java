package oxide.bigger_numbers;

public class StatusStats {
    /** 6 billion bytes. */
    public static long bytesSentLastYear() {
        return 0L; // TODO: write 6 billion as a long literal
    }

    /** Players times seconds, correct even when it passes 2.1 billion. */
    public static long playerSeconds(int players, int seconds) {
        // TODO: this multiplies as int and then widens, so it still wraps
        return players * seconds;
    }
}
