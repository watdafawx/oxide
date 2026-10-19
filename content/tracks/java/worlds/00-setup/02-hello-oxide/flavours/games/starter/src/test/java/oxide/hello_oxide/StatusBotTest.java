package oxide.hello_oxide;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusBotTest {
    @Test
    void botAnnouncesItself() {
        assertThat(StatusBot.greeting()).isEqualTo("Status bot online!");
    }
}
