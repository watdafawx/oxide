package oxide.equals_not_double_equals;

public class MotdCompare {
    /** True when both MOTDs contain the same characters. */
    public static boolean sameMotd(String a, String b) {
        return a.equals(b);
    }

    /** True when both MOTDs match, ignoring capitals. */
    public static boolean sameIgnoringCase(String a, String b) {
        return a.equalsIgnoreCase(b);
    }
}
