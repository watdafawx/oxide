package oxide.a_default_body;

/** Anything that is a Shape has a name and an area. */
public interface Shape {
    String name();

    double area();

    /** "<name> with area <area rounded to a whole number>", e.g. "Rectangle with area 12". */
    default String describe() {
        // TODO: name(), then " with area ", then area() rounded with Math.round(...)
        return "";
    }
}
