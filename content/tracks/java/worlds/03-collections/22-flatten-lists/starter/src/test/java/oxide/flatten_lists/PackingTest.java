package oxide.flatten_lists;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class PackingTest {
    @Test
    void allItemsFlattensEveryOrder() {
        List<List<String>> orders = List.of(List.of("milk", "eggs"), List.of(), List.of("bread", "tea", "jam"));
        assertThat(Packing.allItems(orders)).containsExactly("milk", "eggs", "bread", "tea", "jam");
        assertThat(Packing.allItems(List.of())).isEmpty();
    }
}
