package oxide.maps_key_to_value;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class ShopTest {
    @Test
    void priceOfFallsBackToZero() {
        Map<String, Integer> prices = Map.of("bread", 3, "milk", 2);
        assertThat(Shop.priceOf(prices, "bread")).isEqualTo(3);
        assertThat(Shop.priceOf(prices, "pear")).isEqualTo(0);
    }

    @Test
    void countWordsCountsEachWord() {
        Map<String, Integer> counts = Shop.countWords(List.of("to", "be", "or", "not", "to", "be"));
        assertThat(counts).hasSize(4)
                .containsEntry("to", 2).containsEntry("be", 2).containsEntry("or", 1).containsEntry("not", 1);
    }
}
