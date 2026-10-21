package oxide.any_number_of_values;

public class Numbers {
    /** How many numbers were passed in. howMany() is 0. */
    public static int howMany(int... nums) {
        return nums.length;
    }

    /** Adds any number of ints. sum() is 0. */
    public static int sum(int... nums) {
        int total = 0;
        for (int n : nums) {
            total += n;
        }
        return total;
    }
}
