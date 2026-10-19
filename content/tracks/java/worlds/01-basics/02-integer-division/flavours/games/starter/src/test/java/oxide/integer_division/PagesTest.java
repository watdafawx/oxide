package oxide.integer_division;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.within;

import org.junit.jupiter.api.Test;

class PagesTest {
    @Test
    void splitsPlayersIntoPagesWithLeftover() {
        assertThat(Pages.pagesAndLeftover(17, 5)).containsExactly(3, 2);
        assertThat(Pages.pagesAndLeftover(10, 5)).containsExactly(2, 0);
        assertThat(Pages.fillRatio(17, 20)).isCloseTo(0.85, within(1e-9));
    }
}
