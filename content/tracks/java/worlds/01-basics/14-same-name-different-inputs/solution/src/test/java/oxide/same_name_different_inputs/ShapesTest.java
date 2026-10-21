package oxide.same_name_different_inputs;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;
import org.junit.jupiter.api.Test;

class ShapesTest {
    @Test
    void areaPicksByInputs() {
        assertThat(Shapes.area(3)).isEqualTo(9);
        assertThat(Shapes.area(3, 4)).isEqualTo(12);
        assertThat(Shapes.area(2.0)).isCloseTo(12.566370614359172, within(1e-9));
    }
}
