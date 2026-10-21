package oxide.any_number_of_values;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class ServerTotalsTest {
    @Test
    void serverCountCountsTheNumbers() {
        assertThat(ServerTotals.serverCount()).isEqualTo(0);
        assertThat(ServerTotals.serverCount(7)).isEqualTo(1);
        assertThat(ServerTotals.serverCount(3, 8, 0)).isEqualTo(3);
    }

    @Test
    void totalPlayersSumsAnyNumber() {
        assertThat(ServerTotals.totalPlayers()).isEqualTo(0);
        assertThat(ServerTotals.totalPlayers(7)).isEqualTo(7);
        assertThat(ServerTotals.totalPlayers(3, 8, 0)).isEqualTo(11);
        assertThat(ServerTotals.totalPlayers(new int[] {4, 4})).isEqualTo(8);
    }
}
