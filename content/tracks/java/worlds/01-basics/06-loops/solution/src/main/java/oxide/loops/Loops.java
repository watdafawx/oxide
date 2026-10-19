package oxide.loops;

public class Loops {
    /** 1 + 2 + ... + n (n included). sumTo(0) is 0. */
    public static int sumTo(int n) {
        int total = 0;
        for (int i = 1; i <= n; i++) {
            total += i;
        }
        return total;
    }

    /** Counts a, e, i, o, u in either case. */
    public static int countVowels(String text) {
        int count = 0;
        for (char c : text.toCharArray()) {
            if ("aeiou".indexOf(Character.toLowerCase(c)) >= 0) {
                count++;
            }
        }
        return count;
    }
}
