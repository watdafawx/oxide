package oxide.hello_oxide;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class HelloTest {
    @Test
    void greetingSaysHello() {
        assertThat(Hello.greeting()).isEqualTo("Hello, Oxide!");
    }
}
