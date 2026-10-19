package oxide.interfaces;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;

import org.junit.jupiter.api.Test;

class ShapeTest {
    @Test
    void circleAreaMatchesFormula() {
        Shape circle = new Circle(2);
        assertThat(circle.area()).isCloseTo(Math.PI * 4, within(1e-9));
        assertThat(circle.name()).isEqualTo("Circle");
        assertThat(circle.perimeter()).isCloseTo(4 * Math.PI, within(1e-9));
    }

    @Test
    void rectanglePerimeter() {
        Shape rectangle = new Rectangle(3, 4);
        assertThat(rectangle.perimeter()).isEqualTo(14.0);
        assertThat(rectangle.area()).isEqualTo(12.0);
    }

    @Test
    void defaultDescribeUsesName() {
        assertThat(new Rectangle(3, 4).describe()).isEqualTo("Rectangle with area 12");
        assertThat(new Circle(2).describe()).isEqualTo("Circle with area 13");
    }
}
