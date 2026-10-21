package oxide.numbers_that_wrap;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class IntMathTest {
    @Test
    void intOverflowWraps() {
        assertThat(IntMath.addInts(2, 3)).isEqualTo(5);
        assertThat(IntMath.addInts(Integer.MAX_VALUE, 1)).isEqualTo(Integer.MIN_VALUE);
    }
}
