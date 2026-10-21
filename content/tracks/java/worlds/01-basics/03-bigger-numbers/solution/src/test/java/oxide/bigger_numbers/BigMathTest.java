package oxide.bigger_numbers;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BigMathTest {
    @Test
    void populationNeedsLongLiteral() {
        assertThat(BigMath.worldPopulation()).isEqualTo(8_000_000_000L);
    }

    @Test
    void multiplyAsLongDoesNotWrap() {
        assertThat(BigMath.multiplyAsLong(100_000, 100_000)).isEqualTo(10_000_000_000L);
        assertThat(BigMath.multiplyAsLong(20, 60)).isEqualTo(1_200L);
    }
}
