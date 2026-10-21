package oxide.counted_loop;

public class Loops {
    /** 1 + 2 + ... + n (n included). sumTo(0) is 0. */
    public static int sumTo(int n) {
        int total = 0;
        for (int i = 1; i <= n; i++) {
            total += i;
        }
        return total;
    }

    /** n stars in a row: stars(3) is "***". */
    public static String stars(int n) {
        String row = "";
        for (int i = 0; i < n; i++) {
            row += "*";
        }
        return row;
    }
}
