package oxide.the_compiler_counts;

public class Shapes {
    /** Area of any shape. A switch over the sealed interface, with no default. */
    public static double area(Shape shape) {
        return switch (shape) {
            case Circle c -> Math.PI * c.radius() * c.radius();
            case Rectangle r -> r.width() * r.height();
            case Triangle t -> t.base() * t.height() / 2;
        };
    }
}
