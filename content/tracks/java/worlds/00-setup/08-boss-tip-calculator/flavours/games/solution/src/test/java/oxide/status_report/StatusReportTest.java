package oxide.status_report;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusReportTest {
    @Test
    void parsesPlayerCounts() {
        assertThat(StatusReport.parseCounts("12/20")).containsExactly(12, 20);
        assertThat(StatusReport.parseCounts(" 3 / 20 ")).containsExactly(3, 20);
    }

    @Test
    void computesFillPercent() {
        assertThat(StatusReport.fillPercent(12, 20)).isEqualTo(60);
        assertThat(StatusReport.fillPercent(1, 3)).isEqualTo(33);
        assertThat(StatusReport.fillPercent(0, 0)).isZero();
    }

    @Test
    void serversNeededRoundsUp() {
        assertThat(StatusReport.serversNeeded(45, 20)).isEqualTo(3);
        assertThat(StatusReport.serversNeeded(40, 20)).isEqualTo(2);
        assertThat(StatusReport.serversNeeded(0, 20)).isZero();
    }

    @Test
    void reportIsFormatted() {
        assertThat(StatusReport.report("survival", "12/20")).isEqualTo("survival: 12/20 online (60%)");
        assertThat(StatusReport.report("creative", " 3 / 20 ")).isEqualTo("creative: 3/20 online (15%)");
    }
}
