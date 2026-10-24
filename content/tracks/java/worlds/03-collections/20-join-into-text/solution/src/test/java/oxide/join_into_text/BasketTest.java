package oxide.join_into_text;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class BasketTest {
    @Test
    void bracketedJoinsWithCommas() {
        assertThat(Basket.bracketed(List.of("milk", "eggs", "bread"))).isEqualTo("[milk, eggs, bread]");
        assertThat(Basket.bracketed(List.of("solo"))).isEqualTo("[solo]");
        assertThat(Basket.bracketed(List.of())).isEqualTo("[]");
    }
}
