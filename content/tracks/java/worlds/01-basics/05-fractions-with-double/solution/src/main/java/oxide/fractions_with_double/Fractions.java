package oxide.fractions_with_double;

public class Fractions {
    /** a divided by b as an exact-looking fraction: ratio(17, 20) is 0.85. */
    public static double ratio(int a, int b) {
        return (double) a / b;
    }

    /** The average of two numbers: average(30, 45) is 37.5. */
    public static double average(int a, int b) {
        return (a + b) / 2.0;
    }
}
