package oxide.equals_not_double_equals;

public class TextCompare {
    /** True when both texts contain the same characters. */
    public static boolean sameText(String a, String b) {
        return a == b; // TODO: == compares identity, not content
    }

    /** True when both texts match, ignoring capitals. */
    public static boolean sameIgnoringCase(String a, String b) {
        return false; // TODO: like equals, but ignore capitals
    }
}
