package oxide.bigger_numbers;

public class BigMath {
    /** About 8 billion. */
    public static long worldPopulation() {
        return 0L; // TODO: write 8 billion as a long literal
    }

    /** a times b, correct even when it passes 2.1 billion. */
    public static long multiplyAsLong(int a, int b) {
        // TODO: this multiplies as int and then widens, so it still wraps
        return a * b;
    }
}
