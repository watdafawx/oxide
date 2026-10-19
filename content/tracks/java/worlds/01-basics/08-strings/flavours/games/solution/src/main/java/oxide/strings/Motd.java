package oxide.strings;

public class Motd {
    /** "Welcome" becomes "emocleW". */
    public static String flip(String motd) {
        return new StringBuilder(motd).reverse().toString();
    }

    /** True when the MOTD reads the same both ways, ignoring case and spaces. */
    public static boolean isPalindrome(String motd) {
        String cleaned = motd.replace(" ", "").toLowerCase();
        return cleaned.equals(flip(cleaned));
    }

    /** True when both MOTDs contain the same characters. */
    public static boolean sameMotd(String a, String b) {
        return a.equals(b);
    }
}
