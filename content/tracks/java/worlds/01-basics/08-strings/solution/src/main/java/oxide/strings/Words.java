package oxide.strings;

public class Words {
    /** "Java" becomes "avaJ". */
    public static String reverse(String text) {
        return new StringBuilder(text).reverse().toString();
    }

    /** True when the text reads the same both ways, ignoring case and spaces. */
    public static boolean isPalindrome(String text) {
        String cleaned = text.replace(" ", "").toLowerCase();
        return cleaned.equals(reverse(cleaned));
    }

    /** True when both strings contain the same characters. */
    public static boolean sameText(String a, String b) {
        return a.equals(b);
    }
}
