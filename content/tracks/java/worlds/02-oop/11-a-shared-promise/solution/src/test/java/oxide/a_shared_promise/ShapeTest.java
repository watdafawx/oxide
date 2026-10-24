package oxide.a_shared_promise;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;

import org.junit.jupiter.api.Test;

class ShapeTest {
    @Test
    void circleKeepsThePromise() {
        Shape circle = new Circle(2);
        assertThat(circle.name()).isEqualTo("Circle");
        assertThat(circle.area()).isCloseTo(Math.PI * 4, within(1e-9));
    }

    @Test
    void rectangleKeepsThePromise() {
        Shape rectangle = new Rectangle(3, 4);
        assertThat(rectangle.name()).isEqualTo("Rectangle");
        assertThat(rectangle.area()).isEqualTo(12.0);
    }
}
