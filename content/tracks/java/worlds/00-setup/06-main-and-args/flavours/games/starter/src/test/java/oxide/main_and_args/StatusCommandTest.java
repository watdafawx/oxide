package oxide.main_and_args;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusCommandTest {
    @Test
    void headerUsesFirstArgAsServer() {
        assertThat(StatusCommand.header(new String[] {"play.example.net"})).isEqualTo("Status of play.example.net");
        assertThat(StatusCommand.header(new String[] {"mc.example.org", "extra"})).isEqualTo("Status of mc.example.org");
    }

    @Test
    void compareHeaderUsesTwoServers() {
        assertThat(StatusCommand.compareHeader(new String[] {"play.example.net", "mc.example.org"}))
                .isEqualTo("Status of play.example.net and mc.example.org");
    }
}
