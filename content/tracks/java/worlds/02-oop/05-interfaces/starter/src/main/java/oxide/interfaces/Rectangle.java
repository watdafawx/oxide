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
        throw new UnsupportedOperationException("TODO: return the text \"Rectangle\"");
    }

    @Override
    public double area() {
        throw new UnsupportedOperationException("TODO: width times height");
    }

    @Override
    public double perimeter() {
        throw new UnsupportedOperationException("TODO: 2 times (width plus height)");
    }
}
