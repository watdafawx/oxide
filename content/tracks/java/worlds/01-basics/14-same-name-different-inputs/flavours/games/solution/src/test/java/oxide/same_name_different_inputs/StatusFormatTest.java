package oxide.same_name_different_inputs;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class StatusFormatTest {
    @Test
    void showPicksByArgumentType() {
        assertThat(StatusFormat.show(5)).isEqualTo("5 online");
        assertThat(StatusFormat.show(true)).isEqualTo("ONLINE");
        assertThat(StatusFormat.show(false)).isEqualTo("OFFLINE");
        assertThat(StatusFormat.show("Welcome")).isEqualTo("MOTD: Welcome");
    }
}
