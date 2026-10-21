package oxide.text_never_changes;

public class TextTools {
    /** "  Ada " becomes "Ada". */
    public static String tidy(String raw) {
        return raw.trim();
    }

    /** "Ada" becomes "ADA". */
    public static String shout(String text) {
        return text.toUpperCase();
    }

    /** "Hello" has 5 characters. */
    public static int charCount(String text) {
        return text.length();
    }
}
