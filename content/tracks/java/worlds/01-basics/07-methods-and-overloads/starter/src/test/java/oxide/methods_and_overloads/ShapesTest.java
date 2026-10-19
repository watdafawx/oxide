package oxide.methods_and_overloads;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;

import org.junit.jupiter.api.Test;

class ShapesTest {
    @Test
    void overloadPicksByType() {
        assertThat(Shapes.area(3)).isEqualTo(9);
        assertThat(Shapes.area(3, 4)).isEqualTo(12);
        assertThat(Shapes.area(2.0)).isCloseTo(12.566370614359172, within(1e-9));
    }

    @Test
    void varargsSumsAny() {
        assertThat(Shapes.sum()).isEqualTo(0);
        assertThat(Shapes.sum(5)).isEqualTo(5);
        assertThat(Shapes.sum(1, 2, 3)).isEqualTo(6);
        assertThat(Shapes.sum(new int[] {4, 4})).isEqualTo(8);
    }
}
