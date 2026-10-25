package oxide.count_with_merge;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class TillTest {
    private final List<Sale> day = List.of(
            new Sale("tea", 3), new Sale("cake", 4), new Sale("tea", 3), new Sale("tea", 5));

    @Test
    void salesPerItemCountsEachSale() {
        assertThat(Till.salesPerItem(day)).hasSize(2).containsEntry("tea", 3).containsEntry("cake", 1);
        assertThat(Till.salesPerItem(List.of())).isEmpty();
    }

    @Test
    void takingsPerItemAddsTheAmounts() {
        assertThat(Till.takingsPerItem(day)).hasSize(2).containsEntry("tea", 11).containsEntry("cake", 4);
    }
}
