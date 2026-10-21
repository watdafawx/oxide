package oxide.while_loop;

public class Backoff {
    /** How many times the delay can be halved until it is no longer above the limit. */
    public static int retriesUntilUnder(int delayMs, int limitMs) {
        int retries = 0;
        while (delayMs > limitMs) {
            delayMs = delayMs / 2;
            retries++;
        }
        return retries;
    }
}
