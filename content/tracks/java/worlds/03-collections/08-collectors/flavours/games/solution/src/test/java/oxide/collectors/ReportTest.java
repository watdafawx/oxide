package oxide.collectors;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import java.util.Map;
import org.junit.jupiter.api.Test;

class ReportTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");
    private final List<Mod> pack = List.of(SODIUM, JEI, IRIS);

    @Test
    void groupsModsByLoader() {
        assertThat(Report.groupByLoader(pack))
                .hasSize(2)
                .containsEntry("fabric", List.of(SODIUM, IRIS))
                .containsEntry("forge", List.of(JEI));
        assertThat(Report.countByLoader(pack)).containsEntry("fabric", 2L).containsEntry("forge", 1L);
    }

    @Test
    void joinsModNamesIntoReport() {
        assertThat(Report.report(List.of(SODIUM, IRIS))).isEqualTo("[Sodium, Iris Shaders]");
        assertThat(Report.report(List.of(JEI))).isEqualTo("[Just Enough Items]");
        assertThat(Report.report(List.of())).isEqualTo("[]");
    }

    @Test
    void partitionsOutdatedAndCurrent() {
        Map<String, String> latest = Map.of("sodium", "0.5.8", "iris", "1.7.0", "jei", "15.3.0");
        var parts = Report.partitionByOutdated(pack, latest);
        assertThat(parts.get(true)).containsExactly(SODIUM, JEI);
        assertThat(parts.get(false)).containsExactly(IRIS);
        assertThat(Report.partitionByOutdated(List.of(), latest).get(true)).isEmpty();
    }
}
