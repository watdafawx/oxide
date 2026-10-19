package oxide.log_analyzer;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.entry;

import java.util.List;
import org.junit.jupiter.api.Test;

class LogAnalyzerTest {
    private static final List<String> RAW = List.of(
            "10:15:02 INFO web Started",
            "10:15:30 ERROR db Connection refused",
            "10:15:45 ERROR db Timeout after 30s",
            "10:16:10 WARN web Slow response",
            "10:16:11 ERROR web 500 on /login",
            "10:16:59 INFO cache Warmed",
            "10:17:00 ERROR db Connection refused",
            "10:17:05 INFO web Request ok");

    private static List<LogLine> lines() {
        return RAW.stream().map(LogAnalyzer::parse).toList();
    }

    @Test
    void parsesLine() {
        LogLine line = LogAnalyzer.parse("10:15:30 ERROR db Connection refused");
        assertThat(line.time()).isEqualTo("10:15:30");
        assertThat(line.level()).isEqualTo("ERROR");
        assertThat(line.source()).isEqualTo("db");
        assertThat(line.message()).isEqualTo("Connection refused");
    }

    @Test
    void countsPerLevel() {
        assertThat(LogAnalyzer.countsPerLevel(lines()))
                .hasSize(3)
                .containsEntry("INFO", 3L)
                .containsEntry("ERROR", 4L)
                .containsEntry("WARN", 1L);
    }

    @Test
    void topSourcesRanked() {
        assertThat(LogAnalyzer.topSources(lines(), 3)).containsExactly("web", "db", "cache");
        assertThat(LogAnalyzer.topSources(lines(), 1)).containsExactly("web");
        // equal counts: A-Z wins
        List<LogLine> tie = List.of(
                LogAnalyzer.parse("09:00:00 INFO zeta a"), LogAnalyzer.parse("09:00:01 INFO alpha b"));
        assertThat(LogAnalyzer.topSources(tie, 2)).containsExactly("alpha", "zeta");
    }

    @Test
    void errorsPerMinuteBucketed() {
        assertThat(LogAnalyzer.errorsPerMinute(lines()))
                .containsExactly(entry("10:15", 2L), entry("10:16", 1L), entry("10:17", 1L));
        assertThat(LogAnalyzer.errorsPerMinute(List.of())).isEmpty();
    }
}
