package oxide.boss_first_report;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ReportTest {
    @Test
    void announcesTheBot() {
        assertThat(Report.announce("play.example.net")).isEqualTo("Status bot online: play.example.net");
    }

    @Test
    void occupancyHasThreeValues() {
        assertThat(Report.occupancy("play.example.net", 3, 20)).isEqualTo("play.example.net has 3 of 20 players");
    }

    @Test
    void serverComesFromArgs() {
        assertThat(Report.serverOf(new String[] {"play.example.net"})).isEqualTo("play.example.net");
    }

    @Test
    void reportHasTwoLines() {
        assertThat(Report.report(new String[] {"play.example.net"}, 3, 20))
                .isEqualTo("Status bot online: play.example.net\nplay.example.net has 3 of 20 players");
    }
}
