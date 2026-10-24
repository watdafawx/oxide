package oxide.cut_a_line;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class FeedCutterTest {
    @Test
    void loadersCutsAtCommas() {
        assertThat(FeedCutter.loaders("fabric,forge,quilt")).containsExactly("fabric", "forge", "quilt");
        assertThat(FeedCutter.loaders("neoforge")).containsExactly("neoforge");
    }

    @Test
    void changelogKeepsItsSpaces() {
        assertThat(FeedCutter.changelogOf("sodium 0.5.8 Fixed a crash in chunk loading"))
                .isEqualTo("Fixed a crash in chunk loading");
        assertThat(FeedCutter.changelogOf("jei 15.3.0 Faster search")).isEqualTo("Faster search");
    }
}
