package oxide.interfaces;

public class Rectangle implements Shape {
    private final double width;
    private final double height;

    public Rectangle(double width, double height) {
        this.width = width;
        this.height = height;
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
