package oxide.equals_not_double_equals;

public class MotdCompare {
    /** True when both MOTDs contain the same characters. */
    public static boolean sameMotd(String a, String b) {
        return a == b; // TODO: == compares identity, not content
    }

    /** True when both MOTDs match, ignoring capitals. */
    public static boolean sameIgnoringCase(String a, String b) {
        return false; // TODO: like equals, but ignore capitals
    }
}
