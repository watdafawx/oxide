package oxide.interfaces;

public class Circle implements Shape {
    private final double radius;

    public Circle(double radius) {
        this.radius = radius;
    }

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO: return the text \"Circle\"");
    }

    @Override
    public double area() {
        throw new UnsupportedOperationException("TODO: area is Math.PI times radius times radius");
    }

    @Override
    public double perimeter() {
        throw new UnsupportedOperationException("TODO: perimeter is 2 times Math.PI times radius");
    }
}
