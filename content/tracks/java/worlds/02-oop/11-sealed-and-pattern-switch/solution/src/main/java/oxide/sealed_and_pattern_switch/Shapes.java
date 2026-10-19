package oxide.sealed_and_pattern_switch;

public class Shapes {
    /** Area of any shape. A switch over the sealed interface, with no default branch. */
    public static double area(Shape shape) {
        return switch (shape) {
            case Circle c -> Math.PI * c.radius() * c.radius();
            case Rectangle r -> r.width() * r.height();
            case Triangle t -> t.base() * t.height() / 2;
        };
    }

    /** "a dot" for a Circle of radius 0, else "a circle", "a rectangle" or "a triangle". */
    public static String describe(Shape shape) {
        return switch (shape) {
            case Circle c when c.radius() == 0 -> "a dot";
            case Circle c -> "a circle";
            case Rectangle r -> "a rectangle";
            case Triangle t -> "a triangle";
        };
    }

    /** True for a Rectangle with width equal to height. Use a record pattern. */
    public static boolean isSquare(Shape shape) {
        return switch (shape) {
            case Rectangle(double w, double h) -> w == h;
            case Circle c -> false;
            case Triangle t -> false;
        };
    }
}
