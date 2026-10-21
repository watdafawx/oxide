package oxide.fill_the_slots;

public class Scores {
    /** "Ada: 7/10 correct" */
    public static String scoreLine(String name, int score, int total) {
        return String.format("%s: %d/%d correct", name, score, total);
    }

    /** "3 x apple" (the count comes first). */
    public static String itemLine(String item, int count) {
        return String.format("%d x %s", count, item);
    }
}
