package oxide.bigger_numbers;

public class StatusStats {
    /** 6 billion bytes. */
    public static long bytesSentLastYear() {
        return 6_000_000_000L;
    }

    /** Players times seconds, correct even when it passes 2.1 billion. */
    public static long playerSeconds(int players, int seconds) {
        return (long) players * seconds;
    }
}
