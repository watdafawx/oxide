package oxide.what_new_runs;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ItemStackTest {
    @Test
    void constructorSetsItemAndCount() {
        ItemStack logs = new ItemStack("oak_log", 17);
        assertThat(logs.item()).isEqualTo("oak_log");
        assertThat(logs.count()).isEqualTo(17);
        ItemStack pearls = new ItemStack("ender_pearl", 3);
        assertThat(pearls.item()).isEqualTo("ender_pearl");
        assertThat(pearls.count()).isEqualTo(3);
    }

    @Test
    void overfullStackIsCappedAt64() {
        ItemStack stone = new ItemStack("stone", 100);
        assertThat(stone.item()).isEqualTo("stone");
        assertThat(stone.count()).as("a stack never holds more than 64").isEqualTo(64);
    }
}
