package oxide.methods_and_overloads;

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

    @Test
    void totalPlayersSumsAnyNumberOfServers() {
        assertThat(StatusFormat.totalPlayers()).isEqualTo(0);
        assertThat(StatusFormat.totalPlayers(7)).isEqualTo(7);
        assertThat(StatusFormat.totalPlayers(3, 8, 0)).isEqualTo(11);
        assertThat(StatusFormat.totalPlayers(new int[] {4, 4})).isEqualTo(8);
    }
}
