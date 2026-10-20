package oxide.switch_expressions;

public class Uptime {
    static boolean isLeap(int year) {
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }

    /** Days in a month (1-12), leap years included. Any other month throws IllegalArgumentException. */
    public static int daysInMonth(int month, int year) {
        return 0; // TODO: return a switch expression on month
    }

    /** "great" under 50 ms, "good" 50-99, "ok" 100-199, otherwise "poor". */
    public static String pingGrade(int pingMs) {
        return null; // TODO: switch on pingMs / 50
    }
}
