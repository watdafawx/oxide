package oxide.switch_arms;

public class Uptime {
    static boolean isLeap(int year) {
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }

    /** "great" under 50 ms, "good" 50-99, "ok" 100-199, otherwise "poor". */
    public static String pingGrade(int pingMs) {
        return switch (pingMs / 50) {
            case 0 -> "great";
            // TODO: add arms for 1 ("good") and for 2 and 3 together ("ok")
            default -> "poor";
        };
    }

    /** Days in a month (1-12), leap years included. Any other month throws IllegalArgumentException. */
    public static int daysInMonth(int month, int year) {
        return switch (month) {
            case 1 -> 31;
            // TODO: add arms for the other 31-day months, the 30-day months, and February
            default -> throw new IllegalArgumentException("month " + month);
        };
    }
}
