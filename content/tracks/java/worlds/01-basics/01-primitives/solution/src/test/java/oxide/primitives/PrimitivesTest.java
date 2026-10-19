package oxide.primitives;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PrimitivesTest {
    @Test
    void intOverflowWraps() {
        assertThat(Primitives.addInts(2, 3)).isEqualTo(5);
        assertThat(Primitives.addInts(Integer.MAX_VALUE, 1)).isEqualTo(Integer.MIN_VALUE);
    }

    @Test
    void longAvoidsOverflow() {
        assertThat(Primitives.addAsLong(Integer.MAX_VALUE, 1)).isEqualTo(2_147_483_648L);
        assertThat(Primitives.addAsLong(40, 2)).isEqualTo(42L);
    }
}
