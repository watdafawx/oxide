package oxide.flip_it_round;

public class Words {
    /** "Java" becomes "avaJ". */
    public static String reverse(String text) {
        return new StringBuilder(text).reverse().toString();
    }

    /** True when the text reads the same both ways, ignoring case and spaces. */
    public static boolean isPalindrome(String text) {
        String clean = text.replace(" ", "").toLowerCase();
        return clean.equals(reverse(clean));
    }
}
