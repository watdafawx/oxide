package oxide.a_shared_promise;

public class Circle implements Shape {
    private final double radius;

    public Circle(double radius) {
        this.radius = radius;
    }

    @Override
    public String name() {
        throw new UnsupportedOperationException("TODO: hand back the text \"Circle\"");
    }

    @Override
    public double area() {
        throw new UnsupportedOperationException("TODO: Math.PI times radius times radius");
    }
}
