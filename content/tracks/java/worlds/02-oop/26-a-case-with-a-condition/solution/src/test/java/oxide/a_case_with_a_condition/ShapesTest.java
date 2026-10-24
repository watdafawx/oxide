package oxide.a_case_with_a_condition;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ShapesTest {
    @Test
    void describeNamesEveryShape() {
        assertThat(Shapes.describe(new Circle(2))).isEqualTo("a circle");
        assertThat(Shapes.describe(new Rectangle(3, 4))).isEqualTo("a rectangle");
        assertThat(Shapes.describe(new Triangle(6, 5))).isEqualTo("a triangle");
    }

    @Test
    void zeroCircleIsADot() {
        assertThat(Shapes.describe(new Circle(0))).isEqualTo("a dot");
    }
}
