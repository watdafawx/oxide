package oxide.switch_arms;

public class Rules {
    static boolean isLeap(int year) {
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }

    /** "A" for 90-100, "B" 80s, "C" 70s, "D" 60s, otherwise "F". */
    public static String gradeFor(int score) {
        return switch (score / 10) {
            case 10, 9 -> "A";
            case 8 -> "B";
            case 7 -> "C";
            case 6 -> "D";
            default -> "F";
        };
    }

    /** Days in a month (1-12), leap years included. Any other month throws IllegalArgumentException. */
    public static int daysInMonth(int month, int year) {
        return switch (month) {
            case 1 -> 31;
            case 3, 5, 7, 8, 10, 12 -> 31;
            case 4, 6, 9, 11 -> 30;
            case 2 -> isLeap(year) ? 29 : 28;
            default -> throw new IllegalArgumentException("month " + month);
        };
    }
}
