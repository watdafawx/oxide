package oxide.constants_with_data;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class RarityTest {
    @Test
    void rarityCarriesColorAndWeight() {
        assertThat(Rarity.COMMON.weight()).isEqualTo(60);
        assertThat(Rarity.UNCOMMON.weight()).isEqualTo(25);
        assertThat(Rarity.RARE.color()).isEqualTo("aqua");
        assertThat(Rarity.RARE.weight()).isEqualTo(10);
        assertThat(Rarity.EPIC.color()).isEqualTo("light_purple");
        assertThat(Rarity.EPIC.weight()).isEqualTo(5);
    }

    @Test
    void totalWeightAddsEveryTier() {
        assertThat(Rarity.totalWeight()).isEqualTo(100);
    }
}
