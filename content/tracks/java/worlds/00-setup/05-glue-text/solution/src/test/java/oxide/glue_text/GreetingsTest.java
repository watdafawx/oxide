package oxide.glue_text;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class GreetingsTest {
    @Test
    void greetsByName() {
        assertThat(Greetings.greet("Ada")).isEqualTo("Hello, Ada!");
        assertThat(Greetings.greet("Bo")).isEqualTo("Hello, Bo!");
    }

    @Test
    void greetsTwoPeople() {
        assertThat(Greetings.greetPair("Ada", "Bo")).isEqualTo("Hello, Ada and Bo!");
    }
}
