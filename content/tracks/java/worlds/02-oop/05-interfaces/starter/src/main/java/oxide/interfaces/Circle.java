package oxide.interfaces;

public class Circle implements Shape {
    private final double radius;

    public Circle(double radius) {
        this.radius = radius;
    }

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO");
    }

    @Override
    public double area() {
        throw new UnsupportedOperationException("TODO");
    }

    @Override
    public double perimeter() {
        throw new UnsupportedOperationException("TODO");
    }
}
