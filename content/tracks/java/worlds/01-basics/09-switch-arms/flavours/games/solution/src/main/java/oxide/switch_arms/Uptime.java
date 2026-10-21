package oxide.switch_arms;

public class Uptime {
    static boolean isLeap(int year) {
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }

    /** "great" under 50 ms, "good" 50-99, "ok" 100-199, otherwise "poor". */
    public static String pingGrade(int pingMs) {
        return switch (pingMs / 50) {
            case 0 -> "great";
            case 1 -> "good";
            case 2, 3 -> "ok";
            default -> "poor";
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
