package oxide.the_compiler_counts;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;

import org.junit.jupiter.api.Test;

class ShapesTest {
    @Test
    void shapeIsSealed() {
        assertThat(Shape.class.isSealed()).as("write `sealed interface Shape permits ...`").isTrue();
        assertThat(Shape.class.getPermittedSubclasses()).containsExactlyInAnyOrder(Circle.class, Rectangle.class, Triangle.class);
    }

    @Test
    void switchComputesEveryArea() {
        assertThat(Shapes.area(new Circle(2))).isCloseTo(Math.PI * 4, within(1e-9));
        assertThat(Shapes.area(new Rectangle(3, 4))).isEqualTo(12.0);
        assertThat(Shapes.area(new Triangle(6, 5))).isEqualTo(15.0);
    }
}
