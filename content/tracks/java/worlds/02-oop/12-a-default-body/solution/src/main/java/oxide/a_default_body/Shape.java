package oxide.a_default_body;

/** Anything that is a Shape has a name and an area. */
public interface Shape {
    String name();

    double area();

    /** "<name> with area <area rounded to a whole number>", e.g. "Rectangle with area 12". */
    default String describe() {
        return name() + " with area " + Math.round(area());
    }
}
