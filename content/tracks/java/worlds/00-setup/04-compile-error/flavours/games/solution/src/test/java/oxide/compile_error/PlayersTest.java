package oxide.compile_error;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class PlayersTest {
    @Test
    void totalOnlineAddsBothServers() {
        assertThat(Players.totalOnline(3, 4)).isEqualTo(7);
        assertThat(Players.totalOnline(0, 0)).isZero();
    }
}
