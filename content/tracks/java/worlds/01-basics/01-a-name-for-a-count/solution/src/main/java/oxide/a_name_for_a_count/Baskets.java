package oxide.a_name_for_a_count;

public class Baskets {
    /** Apples in two baskets, 12 and 3, added together. */
    public static int totalApples() {
        int small = 12;
        int big = 3;
        return small + big;
    }

    /** Room left in a basket that holds 20 and already has 12 apples. */
    public static int roomLeft() {
        int capacity = 20;
        int apples = 12;
        return capacity - apples;
    }
}
