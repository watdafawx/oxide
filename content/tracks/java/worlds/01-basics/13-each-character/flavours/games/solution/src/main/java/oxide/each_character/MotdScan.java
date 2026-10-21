package oxide.each_character;

public class MotdScan {
    /** Counts the colour-code characters (the section sign) in a MOTD. */
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
