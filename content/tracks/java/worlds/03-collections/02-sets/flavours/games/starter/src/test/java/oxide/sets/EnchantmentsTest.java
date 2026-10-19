package oxide.sets;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.Set;
import org.junit.jupiter.api.Test;

class EnchantmentsTest {
    @Test
    void uniqueEnchantmentsDropsDuplicates() {
        Set<String> ids = Enchantments.uniqueEnchantments("Sharpness mending sharpness Unbreaking mending");
        assertThat(ids).hasSize(3);
        assertThat(ids).containsExactlyInAnyOrder("sharpness", "mending", "unbreaking");
        assertThat(ids.contains("mending")).isTrue();
    }
}
