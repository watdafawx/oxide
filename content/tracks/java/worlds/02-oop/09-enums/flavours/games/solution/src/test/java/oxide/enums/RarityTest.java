package oxide.enums;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;

import org.junit.jupiter.api.Test;

class RarityTest {
    @Test
    void rarityCarriesColorAndWeight() {
        assertThat(Rarity.COMMON.color()).isEqualTo("white");
        assertThat(Rarity.RARE.color()).isEqualTo("aqua");
        assertThat(Rarity.RARE.dropWeight()).isEqualTo(10);
        assertThat(Rarity.EPIC.color()).isEqualTo("light_purple");
        assertThat(Rarity.EPIC.dropWeight()).isEqualTo(5);
    }

    @Test
    void parseIsCaseInsensitive() {
        assertThat(Rarity.parse("rare")).isEqualTo(Rarity.RARE);
        assertThat(Rarity.parse(" Epic ")).isEqualTo(Rarity.EPIC);
        assertThatThrownBy(() -> Rarity.parse("legendary")).isInstanceOf(IllegalArgumentException.class);
    }

    @Test
    void upgradeWrapsAroundAfterEpic() {
        assertThat(Rarity.COMMON.upgrade()).isEqualTo(Rarity.UNCOMMON);
        assertThat(Rarity.UNCOMMON.upgrade()).isEqualTo(Rarity.RARE);
        assertThat(Rarity.RARE.upgrade()).isEqualTo(Rarity.EPIC);
        assertThat(Rarity.EPIC.upgrade()).isEqualTo(Rarity.COMMON);
    }
}
