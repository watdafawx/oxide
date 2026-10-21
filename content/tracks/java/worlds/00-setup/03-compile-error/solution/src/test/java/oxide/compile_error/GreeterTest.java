package oxide.compile_error;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class GreeterTest {
    @Test
    void welcomeLineBuilds() {
        assertThat(Greeter.welcome()).isEqualTo("Welcome to Oxide!");
    }
}
