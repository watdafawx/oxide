package oxide.interfaces;

public interface Shape {
    String name();

    double area();

    double perimeter();

    /** "<name> with area <area rounded to a whole number>". */
    default String describe() {
        // TODO: name(), then " with area ", then area() rounded with Math.round(...)
        return "";
    }
}
