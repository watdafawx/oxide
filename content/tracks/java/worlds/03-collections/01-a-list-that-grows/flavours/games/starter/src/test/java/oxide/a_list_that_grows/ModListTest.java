package oxide.a_list_that_grows;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ModListTest {
    @Test
    void starterPackKeepsAddOrder() {
        assertThat(ModList.starterPack()).containsExactly("sodium", "iris", "lithium");
    }

    @Test
    void placeholdersGrowPastAnySize() {
        List<String> names = ModList.placeholderNames(100);
        assertThat(names).hasSize(100);
        assertThat(names.get(0)).isEqualTo("mod0");
        assertThat(names.get(99)).isEqualTo("mod99");
        assertThat(ModList.placeholderNames(0)).isEmpty();
    }
}
