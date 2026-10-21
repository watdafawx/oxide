package oxide.the_one_line_choice;

public class Labels {
    /** "yes" or "no". */
    public static String yesNo(boolean answer) {
        return answer ? "yes" : "no";
    }

    /** "item" for exactly one, otherwise "items". */
    public static String plural(int count) {
        return count == 1 ? "item" : "items";
    }
}
