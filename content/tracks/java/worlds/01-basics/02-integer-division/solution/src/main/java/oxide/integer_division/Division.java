package oxide.integer_division;

public class Division {
    /** Returns {a / b, a % b}. */
    public static int[] divmod(int a, int b) {
        return new int[] {a / b, a % b};
    }

    /** Returns a / b as an exact fraction. */
    public static double ratio(int a, int b) {
        return (double) a / b;
    }
}
