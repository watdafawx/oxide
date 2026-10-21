package oxide.numbers_that_wrap;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PlayerMathTest {
    @Test
    void playerCountWrapsPastIntMax() {
        assertThat(PlayerMath.addPlayers(20, 3)).isEqualTo(23);
        assertThat(PlayerMath.addPlayers(Integer.MAX_VALUE, 1)).isEqualTo(Integer.MIN_VALUE);
    }
}
