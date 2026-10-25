package oxide.unpack_in_the_case;

public class Shapes {
    /** True for a Rectangle whose width equals its height. */
    public static boolean isSquare(Shape shape) {
        return switch (shape) {
            case Rectangle(double w, double h) -> w == h;
            case Circle c -> false;
            case Triangle t -> false;
        };
    }
}
