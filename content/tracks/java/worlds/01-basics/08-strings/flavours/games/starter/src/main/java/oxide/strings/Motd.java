package oxide.strings;

public class Motd {
    /** "Welcome" becomes "emocleW". */
    public static String flip(String motd) {
        return null; // TODO: reverse the characters (the lesson shows a StringBuilder trick)
    }

    /** True when the MOTD reads the same both ways, ignoring case and spaces. */
    public static boolean isPalindrome(String motd) {
        return false; // TODO: clean the text, then compare it with its flip
    }

    /** True when both MOTDs contain the same characters. */
    public static boolean sameMotd(String a, String b) {
        return a == b; // TODO: == compares identity, not content
    }
}
