package oxide.compile_error;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class SumTest {
    @Test
    void compilesAndAddsTwoNumbers() {
        assertThat(Sum.add(2, 3)).isEqualTo(5);
        assertThat(Sum.add(-4, 4)).isZero();
    }
}
