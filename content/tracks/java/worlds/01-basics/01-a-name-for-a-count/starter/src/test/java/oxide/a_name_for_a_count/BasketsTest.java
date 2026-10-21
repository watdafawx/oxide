package oxide.a_name_for_a_count;

import static org.assertj.core.api.Assertions.assertThat;

import org.junit.jupiter.api.Test;

class BasketsTest {
    @Test
    void totalApplesAddsBothBaskets() {
        assertThat(Baskets.totalApples()).isEqualTo(15);
    }

    @Test
    void roomLeftIsCapacityMinusApples() {
        assertThat(Baskets.roomLeft()).isEqualTo(8);
    }
}
