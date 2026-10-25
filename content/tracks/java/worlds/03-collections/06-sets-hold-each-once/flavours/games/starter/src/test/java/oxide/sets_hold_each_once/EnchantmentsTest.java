package oxide.sets_hold_each_once;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class EnchantmentsTest {
    @Test
    void distinctDropsRepeatedEnchantments() {
        assertThat(Enchantments.distinct(List.of("sharpness", "mending", "sharpness", "unbreaking", "mending")))
                .containsExactlyInAnyOrder("sharpness", "mending", "unbreaking");
    }

    @Test
    void firstRepeatFindsTheEarliestDuplicate() {
        assertThat(Enchantments.firstRepeat(List.of("looting", "mending", "fortune", "mending", "looting")))
                .isEqualTo("mending");
        assertThat(Enchantments.firstRepeat(List.of("efficiency", "silk_touch"))).isNull();
        assertThat(Enchantments.firstRepeat(List.of("thorns", "thorns"))).isEqualTo("thorns");
    }
}
