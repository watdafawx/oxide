package oxide.counted_loop;

public class StatusLoops {
    /** Players grow by one each minute: 1 + 2 + ... + minutes. playerMinutes(0) is 0. */
    public static int playerMinutes(int minutes) {
        int total = 0;
        for (int i = 1; i <= minutes; i++) {
            total += i;
        }
        return total;
    }

    /** One # per player online: playerBar(3) is "###". */
    public static String playerBar(int online) {
        String bar = "";
        for (int i = 0; i < online; i++) {
            bar += "#";
        }
        return bar;
    }
}
