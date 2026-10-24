package oxide.a_case_with_a_condition;

public class Shapes {
    /** "a dot" for a Circle of radius 0, else "a circle", "a rectangle" or "a triangle". */
    public static String describe(Shape shape) {
        return switch (shape) {
            case Circle c when c.radius() == 0 -> "a dot";
            case Circle c -> "a circle";
            case Rectangle r -> "a rectangle";
            case Triangle t -> "a triangle";
        };
    }
}
