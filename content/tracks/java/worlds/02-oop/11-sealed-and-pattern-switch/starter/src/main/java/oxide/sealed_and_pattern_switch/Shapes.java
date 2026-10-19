package oxide.sealed_and_pattern_switch;

public class Shapes {
    /** Area of any shape. A switch over the sealed interface, with no default branch. */
    public static double area(Shape shape) {
        throw new UnsupportedOperationException("TODO: switch (shape) with one case per record");
    }

    /** "a dot" for a Circle of radius 0, else "a circle", "a rectangle" or "a triangle". */
    public static String describe(Shape shape) {
        throw new UnsupportedOperationException("TODO: switch with a `when` guard on Circle");
    }

    /** True for a Rectangle with width equal to height. Use a record pattern. */
    public static boolean isSquare(Shape shape) {
        throw new UnsupportedOperationException("TODO: case Rectangle(double w, double h) -> ...");
    }
}
