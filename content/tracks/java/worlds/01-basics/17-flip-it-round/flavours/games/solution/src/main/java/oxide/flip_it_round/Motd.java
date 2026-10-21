package oxide.flip_it_round;

public class Motd {
    /** "Welcome" becomes "emocleW". */
    public static String flip(String motd) {
        return new StringBuilder(motd).reverse().toString();
    }

    /** True when the MOTD reads the same both ways, ignoring case and spaces. */
    public static boolean isPalindrome(String motd) {
        String clean = motd.replace(" ", "").toLowerCase();
        return clean.equals(flip(clean));
    }
}
