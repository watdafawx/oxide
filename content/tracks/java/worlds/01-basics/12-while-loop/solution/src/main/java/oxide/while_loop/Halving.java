package oxide.while_loop;

public class Halving {
    /** How many times start can be halved until it is no longer above the limit. */
    public static int halvings(int start, int limit) {
        int steps = 0;
        while (start > limit) {
            start = start / 2;
            steps++;
        }
        return steps;
    }
}
