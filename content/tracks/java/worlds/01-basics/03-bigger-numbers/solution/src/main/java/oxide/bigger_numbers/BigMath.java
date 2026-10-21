package oxide.bigger_numbers;

public class BigMath {
    /** About 8 billion. */
    public static long worldPopulation() {
        return 8_000_000_000L;
    }

    /** a times b, correct even when it passes 2.1 billion. */
    public static long multiplyAsLong(int a, int b) {
        return (long) a * b;
    }
}
