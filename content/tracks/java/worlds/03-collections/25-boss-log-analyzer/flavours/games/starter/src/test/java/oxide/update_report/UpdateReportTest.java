package oxide.update_report;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.entry;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class UpdateReportTest {
    private static final List<String> RAW = List.of(
            "sodium,Sodium,fabric,9500,0.5.3",
            "iris,Iris Shaders,fabric,8200,1.7.0",
            "lithium,Lithium,fabric,7100,0.12.0",
            "jei,Just Enough Items,forge,12000,15.2.0",
            "create,Create,forge,15000,0.5.1",
            "quark,Quark,neoforge,6000,4.0");

    private static final Map<String, String> LATEST = Map.of(
            "sodium", "0.5.8", "iris", "1.7.0", "lithium", "0.12.1",
            "jei", "15.3.0", "create", "0.5.1", "quark", "4.0");

    private static List<Mod> mods() {
        return RAW.stream().map(UpdateReport::parseMod).toList();
    }

    @Test
    void parsesModLine() {
        Mod mod = UpdateReport.parseMod("sodium,Sodium,fabric,9500,0.5.3");
        assertThat(mod.id()).isEqualTo("sodium");
        assertThat(mod.name()).isEqualTo("Sodium");
        assertThat(mod.loader()).isEqualTo("fabric");
        assertThat(mod.downloads()).isEqualTo(9500);
        assertThat(mod.version()).isEqualTo("0.5.3");
    }

    @Test
    void countsModsPerLoader() {
        assertThat(UpdateReport.countsPerLoader(mods()))
                .hasSize(3)
                .containsEntry("fabric", 3L)
                .containsEntry("forge", 2L)
                .containsEntry("neoforge", 1L);
    }

    @Test
    void topModsRanked() {
        assertThat(UpdateReport.topDownloaded(mods(), 3)).containsExactly("Create", "Just Enough Items", "Sodium");
        assertThat(UpdateReport.topDownloaded(mods(), 1)).containsExactly("Create");
        // equal downloads: A-Z wins
        List<Mod> tie = List.of(
                new Mod("z", "Zeta", "fabric", 100, "1"), new Mod("a", "Alpha", "fabric", 100, "1"));
        assertThat(UpdateReport.topDownloaded(tie, 2)).containsExactly("Alpha", "Zeta");
    }

    @Test
    void outdatedPerLoaderSorted() {
        assertThat(UpdateReport.outdatedPerLoader(mods(), LATEST))
                .containsExactly(entry("fabric", 2L), entry("forge", 1L));
        assertThat(UpdateReport.outdatedPerLoader(List.of(), LATEST)).isEmpty();
    }
}
