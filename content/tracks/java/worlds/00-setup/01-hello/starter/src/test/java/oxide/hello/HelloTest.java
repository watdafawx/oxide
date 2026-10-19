package oxide.hello;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class HelloTest {
    @Test
    void greets_by_name() {
        assertThat(Hello.greets("Ada")).isEqualTo("Hello, Ada!");
    }
}
