package oxide.group_a_stream;

import static org.assertj.core.api.Assertions.assertThat;

import java.util.List;
import org.junit.jupiter.api.Test;

class LoaderReportTest {
    private static final Mod SODIUM = new Mod("sodium", "Sodium", "fabric", 9500, "0.5.3");
    private static final Mod IRIS = new Mod("iris", "Iris Shaders", "fabric", 8200, "1.7.0");
    private static final Mod JEI = new Mod("jei", "Just Enough Items", "forge", 12000, "15.2.0");
    private final List<Mod> pack = List.of(SODIUM, JEI, IRIS);

    @Test
    void groupByLoaderBucketsMods() {
        assertThat(LoaderReport.groupByLoader(pack)).hasSize(2)
                .containsEntry("fabric", List.of(SODIUM, IRIS))
                .containsEntry("forge", List.of(JEI));
    }

    @Test
    void countByLoaderCountsEachBucket() {
        assertThat(LoaderReport.countByLoader(pack)).hasSize(2)
                .containsEntry("fabric", 2L).containsEntry("forge", 1L);
    }
}
