package oxide.interfaces;

public interface Shape {
    String name();

    double area();

    double perimeter();

    /** "<name> with area <area rounded to a whole number>". */
    default String describe() {
        return name() + " with area " + Math.round(area());
    }
}
