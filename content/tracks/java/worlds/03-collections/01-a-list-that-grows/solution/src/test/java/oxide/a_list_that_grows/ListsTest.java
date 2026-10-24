package oxide.a_list_that_grows;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ListsTest {
    @Test
    void shoppingListKeepsAddOrder() {
        assertThat(Lists.shoppingList()).containsExactly("milk", "eggs", "bread");
    }

    @Test
    void labelsGrowPastAnySize() {
        List<String> labels = Lists.labels(100);
        assertThat(labels).hasSize(100);
        assertThat(labels.get(0)).isEqualTo("item0");
        assertThat(labels.get(99)).isEqualTo("item99");
        assertThat(Lists.labels(0)).isEmpty();
    }
}
