package oxide.a_name_for_a_count;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class TallyTest {
    @Test
    void totalOnlineAddsBothServers() {
        assertThat(Tally.totalOnline()).isEqualTo(15);
    }

    @Test
    void slotsLeftIsCapacityMinusOnline() {
        assertThat(Tally.slotsLeft()).isEqualTo(8);
    }
}
