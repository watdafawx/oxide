package oxide.unpack_in_the_case;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ShapesTest {
    @Test
    void squareIsDetected() {
        assertThat(Shapes.isSquare(new Rectangle(2, 2))).isTrue();
        assertThat(Shapes.isSquare(new Rectangle(2, 3))).isFalse();
    }

    @Test
    void otherShapesAreNotSquares() {
        assertThat(Shapes.isSquare(new Circle(2))).isFalse();
        assertThat(Shapes.isSquare(new Triangle(2, 2))).isFalse();
    }
}
