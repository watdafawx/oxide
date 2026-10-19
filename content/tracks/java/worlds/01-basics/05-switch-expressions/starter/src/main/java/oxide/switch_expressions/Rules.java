package oxide.switch_expressions;

public class Rules {
    static boolean isLeap(int year) {
        return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    }

    /** Days in a month (1-12), leap years included. */
    public static int daysInMonth(int month, int year) {
        return 0; // TODO: return a switch expression on month
    }

    /** "A" for 90-100, "B" 80s, "C" 70s, "D" 60s, otherwise "F". */
    public static String gradeFor(int score) {
        return null; // TODO: switch on score / 10
    }
}
