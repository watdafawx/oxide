package oxide.loops;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class LoopsTest {
    @Test
    void sumToAddsInclusive() {
        assertThat(Loops.sumTo(10)).isEqualTo(55);
        assertThat(Loops.sumTo(1)).isEqualTo(1);
        assertThat(Loops.sumTo(0)).isEqualTo(0);
    }

    @Test
    void countVowelsIgnoresCase() {
        assertThat(Loops.countVowels("Hello World")).isEqualTo(3);
        assertThat(Loops.countVowels("AEIOU")).isEqualTo(5);
        assertThat(Loops.countVowels("rhythm")).isEqualTo(0);
    }
}
