package oxide.methods_and_overloads;

public class Shapes {
    /** Area of a square. */
    public static int area(int side) {
        return side * side;
    }

    /** Area of a rectangle. */
    public static int area(int width, int height) {
        return width * height;
    }

    /** Area of a circle. */
    public static double area(double radius) {
        return Math.PI * radius * radius;
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
