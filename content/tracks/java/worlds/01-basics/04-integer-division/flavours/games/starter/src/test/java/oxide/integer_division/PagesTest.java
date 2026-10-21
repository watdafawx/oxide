package oxide.integer_division;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PagesTest {
    @Test
    void fullPagesCountsWholePages() {
        assertThat(Pages.fullPages(17, 5)).isEqualTo(3);
        assertThat(Pages.fullPages(10, 5)).isEqualTo(2);
        assertThat(Pages.fullPages(4, 5)).isEqualTo(0);
    }

    @Test
    void lastPageHoldsTheRest() {
        assertThat(Pages.playersOnLastPage(17, 5)).isEqualTo(2);
        assertThat(Pages.playersOnLastPage(10, 5)).isEqualTo(0);
    }
}
