package oxide.group_into_lists;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class LoaderListsTest {
    @Test
    void namesByLoaderGroupsModsInOrder() {
        List<Mod> pack = List.of(
                new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3"),
                new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0"),
                new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0"),
                new Mod("quark", "Quark", "neoforge", 6000, "4.0"));
        Map<String, List<String>> groups = LoaderLists.namesByLoader(pack);
        assertThat(groups).hasSize(3);
        assertThat(groups.get("fabric")).containsExactly("Sodium", "Iris Shaders");
        assertThat(groups.get("forge")).containsExactly("Just Enough Items");
        assertThat(groups.get("neoforge")).containsExactly("Quark");
    }
}
