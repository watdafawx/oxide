package oxide.sealed_and_pattern_switch;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;

import org.junit.jupiter.api.Test;

class ShapesTest {
    @Test
    void exhaustiveSwitchComputesArea() {
        assertThat(Shapes.area(new Circle(2))).isCloseTo(Math.PI * 4, within(1e-9));
        assertThat(Shapes.area(new Rectangle(3, 4))).isEqualTo(12.0);
        assertThat(Shapes.area(new Triangle(6, 5))).isEqualTo(15.0);
        assertThat(Shape.class.isSealed()).isTrue();
    }

    @Test
    void guardedPatternHandlesZero() {
        assertThat(Shapes.describe(new Circle(0))).isEqualTo("a dot");
        assertThat(Shapes.describe(new Circle(2))).isEqualTo("a circle");
        assertThat(Shapes.describe(new Rectangle(3, 4))).isEqualTo("a rectangle");
        assertThat(Shapes.describe(new Triangle(6, 5))).isEqualTo("a triangle");
    }

    @Test
    void recordPatternDeconstructs() {
        assertThat(Shapes.isSquare(new Rectangle(2, 2))).isTrue();
        assertThat(Shapes.isSquare(new Rectangle(2, 3))).isFalse();
        assertThat(Shapes.isSquare(new Circle(2))).isFalse();
        assertThat(Shapes.isSquare(new Triangle(2, 2))).isFalse();
    }
}
