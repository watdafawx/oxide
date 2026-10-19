package oxide.lists;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class ModListTest {
    @Test
    void dropsModBySlotAndByProjectId() {
        assertThat(ModList.dropMods(List.of(394468, 306612, 238222, 310111), 0, 238222))
                .containsExactly(306612, 310111);
        // project id 3 must not be treated as slot 3
        assertThat(ModList.dropMods(List.of(5, 6, 7, 3), 0, 3)).containsExactly(6, 7);
    }

    @Test
    void modListGrowsPastInitialSize() {
        List<String> names = ModList.placeholderNames(100);
        assertThat(names).hasSize(100);
        assertThat(names.get(0)).isEqualTo("mod0");
        assertThat(names.get(99)).isEqualTo("mod99");
    }
}
