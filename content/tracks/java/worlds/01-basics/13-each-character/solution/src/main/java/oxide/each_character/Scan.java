package oxide.each_character;

public class Scan {
    /** Counts the spaces in a text. */
    public static int countSpaces(String text) {
        int count = 0;
        for (char c : text.toCharArray()) {
            if (c == ' ') {
                count++;
            }
        }
        return count;
    }
}
