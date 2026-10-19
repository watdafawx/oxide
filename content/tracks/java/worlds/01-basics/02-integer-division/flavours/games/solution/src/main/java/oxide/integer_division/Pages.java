package oxide.integer_division;

public class Pages {
    /** Returns {full pages, players left over for the last page}. */
    public static int[] pagesAndLeftover(int players, int perPage) {
        return new int[] {players / perPage, players % perPage};
    }

    /** How full the server is, 0.0 to 1.0. */
    public static double fillRatio(int players, int capacity) {
        return (double) players / capacity;
    }
}
