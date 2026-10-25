package oxide.a_fixed_set;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class RarityTest {
    @Test
    void onlyCommonIsCommon() {
        assertThat(Rarity.COMMON.isCommon()).isTrue();
        assertThat(Rarity.UNCOMMON.isCommon()).isFalse();
        assertThat(Rarity.EPIC.isCommon()).isFalse();
    }

    @Test
    void upgradeWrapsAroundAfterEpic() {
        assertThat(Rarity.COMMON.upgrade()).isEqualTo(Rarity.UNCOMMON);
        assertThat(Rarity.UNCOMMON.upgrade()).isEqualTo(Rarity.RARE);
        assertThat(Rarity.RARE.upgrade()).isEqualTo(Rarity.EPIC);
        assertThat(Rarity.EPIC.upgrade()).isEqualTo(Rarity.COMMON);
    }
}
