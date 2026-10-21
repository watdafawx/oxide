package oxide.same_name_different_inputs;

public class Shapes {
    /** Area of a square. */
    public static int area(int side) {
        return side * side;
    }

    /** Area of a rectangle. */
    public static int area(int width, int height) {
        return width * height;
    }

    /** Area of a circle. Math.PI is the number pi, ready made. */
    public static double area(double radius) {
        return Math.PI * radius * radius;
    }
}
