package oxide.equals_not_double_equals;

public class TextCompare {
    /** True when both texts contain the same characters. */
    public static boolean sameText(String a, String b) {
        return a.equals(b);
    }

    /** True when both texts match, ignoring capitals. */
    public static boolean sameIgnoringCase(String a, String b) {
        return a.equalsIgnoreCase(b);
    }
}
