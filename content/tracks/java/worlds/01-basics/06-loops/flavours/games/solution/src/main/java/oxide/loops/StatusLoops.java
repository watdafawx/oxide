package oxide.loops;

public class StatusLoops {
    /** Players grow by one each minute: 1 + 2 + ... + minutes. playerMinutes(0) is 0. */
    public static int playerMinutes(int minutes) {
        int total = 0;
        for (int m = 1; m <= minutes; m++) {
            total += m;
        }
        return total;
    }

    /** Counts the color-code characters (the section sign) in a MOTD. */
    public static int countColorCodes(String motd) {
        int count = 0;
        for (char c : motd.toCharArray()) {
            if (c == '§') {
                count++;
            }
        }
        return count;
    }
}
