package oxide.interfaces;

public interface Shape {
    String name();

    double area();

    double perimeter();

    /** "<name> with area <area rounded to a whole number>". */
    default String describe() {
        // TODO: build the text from name() and area()
        return "";
    }
}
