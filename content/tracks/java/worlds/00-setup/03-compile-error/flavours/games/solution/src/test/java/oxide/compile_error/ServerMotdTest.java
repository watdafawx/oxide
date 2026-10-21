package oxide.compile_error;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ServerMotdTest {
    @Test
    void motdBuilds() {
        assertThat(ServerMotd.motd()).isEqualTo("Welcome to the server!");
    }
}
